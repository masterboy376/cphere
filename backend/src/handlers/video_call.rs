use actix_session::SessionExt;
use actix_web::{post, web, HttpRequest, HttpResponse, Error};
use serde::Deserialize;
use mongodb::bson::{doc, oid::ObjectId};
use crate::{
    states::app_state::AppState,
    models::{notification::Notification, chat::Chat},
};
use chrono::Utc;

#[derive(Debug, Deserialize)]
pub struct VideoCallRequest {
    pub recipient_id: String,  // ID of the user to call
    pub chat_id: String,       // ID of the existing chat
}

#[post("/video_call/init")]
pub async fn initiate_video_call(
    req: HttpRequest,
    state: web::Data<AppState>,
    body: web::Json<VideoCallRequest>,
) -> Result<HttpResponse, Error> {
    // Get the caller's user ID from the session
    let session = req.get_session();
    let caller_id_str = session
        .get::<String>("user_id")
        .map_err(|_| actix_web::error::ErrorInternalServerError("Session error"))?
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;
    let caller_id = ObjectId::parse_str(&caller_id_str)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid user ID in session"))?;

    // Parse recipient_id and chat_id
    let recipient_id = ObjectId::parse_str(&body.recipient_id)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid recipient ID"))?;
    let chat_id = ObjectId::parse_str(&body.chat_id)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid chat ID"))?;

    // Check if the recipient is online
    let ws_sessions = state.ws_sessions.read().await;
    if !ws_sessions.contains_key(&recipient_id) {
        return Err(actix_web::error::ErrorBadRequest("Recipient is not online"));
    }
    drop(ws_sessions); // Release the read lock

    // Check if both users are participants in the chat
    let chats_collection = state.db.collection::<Chat>(Chat::collection_name());
    let chat = chats_collection
        .find_one(doc! { "_id": &chat_id }, None)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?
        .ok_or_else(|| actix_web::error::ErrorNotFound("Chat not found"))?;

    if !chat.participants.contains(&caller_id) || !chat.participants.contains(&recipient_id) {
        return Err(actix_web::error::ErrorForbidden("Users are not participants in the chat"));
    }

    // Create a notification for the recipient
    let notification = Notification {
        id: None,
        user_id: recipient_id.clone(),
        sender_id: caller_id.clone(),
        notification_type: "video_call_request".to_string(),
        message: "Incoming video call request".to_string(),
        is_handled: false,
        created_at: Utc::now(),
    };

    let notifications_collection = state.db.collection::<Notification>(Notification::collection_name());
    notifications_collection
        .insert_one(notification.clone(), None)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to create notification"))?;

    // Send the notification to the recipient via WebSocket
    let ws_sessions = state.ws_sessions.read().await;
    if let Some(recipient_addr) = ws_sessions.get(&recipient_id) {
        let notification_json = serde_json::to_string(&notification)
            .map_err(|_| actix_web::error::ErrorInternalServerError("Serialization error"))?;

        recipient_addr.do_send(crate::websocket::session::TextMessage(notification_json));
    }

    Ok(HttpResponse::Ok().json("Video call request sent"))
}

#[derive(Debug, Deserialize)]
pub struct VideoCallResponse {
    pub notification_id: String,  // ID of the notification being responded to
    pub accepted: bool,           // Indicates if the call is accepted or declined
}

#[post("/video_call/respond")]
pub async fn respond_video_call(
    req: HttpRequest,
    state: web::Data<AppState>,
    body: web::Json<VideoCallResponse>,
) -> Result<HttpResponse, Error> {
    // Get the recipient's user ID from the session
    let session = req.get_session();
    let recipient_id_str = session
        .get::<String>("user_id")
        .map_err(|_| actix_web::error::ErrorInternalServerError("Session error"))?
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;
    let recipient_id = ObjectId::parse_str(&recipient_id_str)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid user ID in session"))?;

    // Parse notification_id
    let notification_id = ObjectId::parse_str(&body.notification_id)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid notification ID"))?;

    // Retrieve and update the notification
    let notifications_collection = state.db.collection::<Notification>(Notification::collection_name());
    let notification = notifications_collection
        .find_one_and_update(
            doc! { "_id": &notification_id, "user_id": &recipient_id },
            doc! { "$set": { "is_handled": true } },
            None,
        )
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?
        .ok_or_else(|| actix_web::error::ErrorNotFound("Notification not found"))?;

    // Extract caller's user ID from the notification
    let caller_id = notification.sender_id.clone();

    // Send response to the caller via WebSocket
    let ws_sessions = state.ws_sessions.read().await;
    if let Some(caller_addr) = ws_sessions.get(&caller_id) {
        let response_message = if body.accepted {
            serde_json::json!({
                "type": "video_call_accepted",
                "from": recipient_id_str,
                "notification_id": notification_id.to_hex(),
            })
        } else {
            serde_json::json!({
                "type": "video_call_declined",
                "from": recipient_id_str,
                "notification_id": notification_id.to_hex(),
            })
        };
        caller_addr.do_send(crate::websocket::session::TextMessage(response_message.to_string()));
    }

    Ok(HttpResponse::Ok().json("Response sent"))
}
