use crate::{
    config::app_config::AppConfig,
    models::{chat_model::Chat, notification_model::Notification, user_model::User},
    services::notification_service::NotificationSummary,
    states::app_state::AppState,
    websocket::websocket_session::TextMessage,
};
use actix_web::{error::ErrorInternalServerError, Error};
use mongodb::bson::{doc, oid::ObjectId};
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
pub struct VideoCallRequest {
    pub recipient_id: String,
    pub chat_id: String,
}

#[derive(Debug, Deserialize)]
pub struct VideoCallResponse {
    pub notification_id: String,
    pub accepted: bool,
}

/// Initiates a video call by verifying that the recipient is online,
/// checking that both caller and recipient are in the chat participants,
/// and then creating and sending a WebSocket notification.
pub async fn initiate_video_call_logic(
    state: &AppState,
    caller_id: ObjectId,
    recipient_id: ObjectId,
    chat_id: ObjectId,
) -> Result<(), Error> {
    // Load configuration
    let config = AppConfig::new().map_err(|_| ErrorInternalServerError("Config error"))?;

    // Check if the recipient is online
    let ws_sessions = state.ws_sessions.read().await;
    if !ws_sessions.contains_key(&recipient_id) {
        return Err(actix_web::error::ErrorBadRequest("Recipient is not online"));
    }
    drop(ws_sessions);

    // Validate chat participants
    let chats_collection = state.db.collection::<Chat>(Chat::collection_name());
    let chat = chats_collection
        .find_one(doc! { "_id": &chat_id }, None)
        .await
        .map_err(|_| ErrorInternalServerError("Database error"))?
        .ok_or_else(|| actix_web::error::ErrorNotFound("Chat not found"))?;

    if !chat.participant_ids.contains(&caller_id) || !chat.participant_ids.contains(&recipient_id) {
        return Err(actix_web::error::ErrorForbidden("Users are not participants in the chat"));
    }

    // Create notification
    let notification = Notification::new(
        config.video_call_notification,
        recipient_id,
        caller_id.clone(),
        "Incoming video call request",
    );

    let notifications_collection = state.db.collection::<Notification>(Notification::collection_name());
    let insert_result = notifications_collection
        .insert_one(notification.clone(), None)
        .await
        .map_err(|_| ErrorInternalServerError("Failed to create notification"))?;
    
    let notification_id = insert_result.inserted_id.as_object_id()
        .ok_or_else(|| ErrorInternalServerError("Failed to get notification ID"))?;
    
    // Get sender's username
    let users_collection = state.db.collection::<User>(User::collection_name());
    let sender = users_collection
        .find_one(doc! { "_id": &caller_id }, None)
        .await
        .map_err(|_| ErrorInternalServerError("Database error"))?
        .ok_or_else(|| ErrorInternalServerError("Sender user not found"))?;

    // Create notification summary and send WebSocket message
    let notification_summary = NotificationSummary {
        id: notification_id.to_hex(),
        notification_type: config.video_call_notification.to_string(),
        sender_user_id: caller_id.to_hex(),
        sender_username: sender.username,
        message: "Incoming video call request".to_string(),
        timestamp: notification.created_at
    };

    let ws_message = json!({
        "type": "video_call_request",
        "notification": notification_summary
    });

    let ws_sessions = state.ws_sessions.read().await;
    if let Some(recipient_addr) = ws_sessions.get(&recipient_id) {
        recipient_addr.do_send(TextMessage(ws_message.to_string()));
    }

    Ok(())
}

/// Responds to an incoming video call request, updates the notification,
/// and sends a WebSocket message to the caller with the response.
pub async fn respond_video_call_logic(
    state: &AppState,
    recipient_id: ObjectId,
    notification_id: ObjectId,
    accepted: bool,
) -> Result<(), Error> {
    // Update notification status
    let notifications_collection = state.db.collection::<Notification>(Notification::collection_name());
    let notification = notifications_collection
        .find_one_and_update(
            doc! { "_id": &notification_id, "user_id": &recipient_id },
            doc! { "$set": { "is_handled": true } },
            None,
        )
        .await
        .map_err(|_| ErrorInternalServerError("Database error"))?
        .ok_or_else(|| actix_web::error::ErrorNotFound("Notification not found"))?;

    // Send response to caller
    let ws_sessions = state.ws_sessions.read().await;
    if let Some(caller_addr) = ws_sessions.get(&notification.sender_id) {
        let response_message = json!({
            "type": if accepted { "video_call_accepted" } else { "video_call_declined" },
            "from": recipient_id.to_hex(),
            "notification_id": notification_id.to_hex(),
        });

        caller_addr.do_send(TextMessage(response_message.to_string()));
    }

    Ok(())
}
