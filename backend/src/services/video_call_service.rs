use crate::{
    config::app_config::AppConfig,
    models::{chat_model::Chat, notification_model::Notification},
    states::app_state::AppState,
    websocket::websocket_session::TextMessage,
};
use actix_web::{error::ErrorInternalServerError, Error};
use mongodb::bson::{doc, oid::ObjectId};
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
pub struct VideoCallRequest {
    pub recipient_id: String, // ID of the user to call
    pub chat_id: String,      // ID of the existing chat
}

#[derive(Debug, Deserialize)]
pub struct VideoCallResponse {
    pub notification_id: String, // ID of the notification being responded to
    pub accepted: bool,          // Indicates if the call is accepted or declined
}

pub async fn initiate_video_call_logic(
    state: &AppState,
    caller_id: ObjectId,
    recipient_id: ObjectId,
    chat_id: ObjectId,
) -> Result<(), Error> {
    let config = AppConfig::new().map_err(|_| ErrorInternalServerError("Config error"))?;

    // Check if recipient is online
    let ws_sessions = state.ws_sessions.read().await;
    if !ws_sessions.contains_key(&recipient_id) {
        return Err(actix_web::error::ErrorBadRequest("Recipient is not online"));
    }
    drop(ws_sessions); // Release read lock

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
        recipient_id.clone(),
        caller_id.clone(),
        "Incoming video call request",
    );

    let notifications_collection = state.db.collection::<Notification>(Notification::collection_name());
    notifications_collection
        .insert_one(notification.clone(), None)
        .await
        .map_err(|_| ErrorInternalServerError("Failed to create notification"))?;

    // Send WebSocket notification
    let ws_sessions = state.ws_sessions.read().await;
    if let Some(recipient_addr) = ws_sessions.get(&recipient_id) {
        let notification_json = serde_json::to_string(&notification)
            .map_err(|_| ErrorInternalServerError("Serialization error"))?;
        recipient_addr.do_send(TextMessage(notification_json));
    }

    Ok(())
}

pub async fn respond_video_call_logic(
    state: &AppState,
    recipient_id: ObjectId,
    notification_id: ObjectId,
    accepted: bool,
) -> Result<(), Error> {
    // Retrieve and update notification
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

    let caller_id = notification.sender_id.clone();

    // Send response to caller via WebSocket
    let ws_sessions = state.ws_sessions.read().await;
    if let Some(caller_addr) = ws_sessions.get(&caller_id) {
        let response_message = json!({
            "type": if accepted { "video_call_accepted" } else { "video_call_declined" },
            "from": recipient_id.to_hex(),
            "notification_id": notification_id.to_hex(),
        });

        caller_addr.do_send(TextMessage(response_message.to_string()));
    }

    Ok(())
}
