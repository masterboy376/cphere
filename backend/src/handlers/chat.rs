use actix_session::SessionExt;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Error};
use actix_web_actors::ws;
use futures::TryStreamExt;
use mongodb::bson::{
    oid::ObjectId,
    doc
};
use serde::{
    Deserialize,
    Serialize
};
use std::collections::HashSet;

use crate::{
    states::app_state::AppState,
    websocket::session::WsChatSession,
    models::chat::Chat,
    models::message::Message,
    models::notification::Notification,
    middleware::auth_middleware::AuthMiddlewareFactory, // Import your auth middleware
};


#[get("/ws")]
pub async fn ws_session_start_handler(
    req: HttpRequest,
    stream: web::Payload,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    // Extract the user ID from the session
    let session = req.get_session();
    let user_id_str = match session.get::<String>("user_id") {
        Ok(Some(uid)) => uid,
        Ok(None) => {
            return Err(actix_web::error::ErrorUnauthorized("Not authenticated"));
        }
        Err(e) => {
            eprintln!("Session error: {}", e);
            return Err(actix_web::error::ErrorInternalServerError("Session error"));
        }
    };

    // Parse the user ID as an ObjectId
    let user_id = match ObjectId::parse_str(&user_id_str) {
        Ok(oid) => oid,
        Err(e) => {
            eprintln!("Invalid user ID in session: {}", e);
            return Err(actix_web::error::ErrorUnauthorized("Invalid user ID"));
        }
    };

    // Start the WebSocket connection
    ws::start(
        WsChatSession::new(user_id, state.clone()),
        &req,
        stream,
    )
}

#[derive(Debug, Deserialize)]
struct CreateChatRoomRequest {
    participant_ids: Vec<String>,
}

#[post("/chats/create")]
pub async fn create_new_chat_handler(
    req: HttpRequest,
    state: web::Data<AppState>,
    body: web::Json<CreateChatRoomRequest>,
) -> Result<HttpResponse, Error> {
    // Get user ID from session
    let session = req.get_session();
    let user_id_str = session.get::<String>("user_id")
        .map_err(|_| actix_web::error::ErrorInternalServerError("Session error"))?
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    let user_id = ObjectId::parse_str(&user_id_str)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid user ID in session"))?;

    // Parse participant IDs
    let mut participant_ids = body
        .participant_ids
        .iter()
        .filter_map(|id| ObjectId::parse_str(id).ok())
        .collect::<HashSet<ObjectId>>();

    // Add the creator to the participants if not already included
    participant_ids.insert(user_id.clone());

    // Create the chat room
    let new_chat = Chat::new(None, participant_ids.iter().cloned().collect(), None);
    let chats = state.db.collection::<Chat>(Chat::collection_name());

    chats.insert_one(new_chat.clone(), None).await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to create chat room"))?;

    Ok(HttpResponse::Ok().json(new_chat))
}

#[derive(Debug, Deserialize)]
struct SendMessageRequest {
    chat_id: ObjectId,
    content: String,
}

// REST API handlers for sending messsage, when the user is not connected to the WebSocket.
#[post("/message")]
pub async fn send_message_handler(
    req: HttpRequest,
    state: web::Data<AppState>,
    body: web::Json<SendMessageRequest>,
) -> Result<HttpResponse, Error> {
    // Get user ID from session
    let session = req.get_session();
    let user_id_str = session.get::<String>("user_id")
        .map_err(|_| actix_web::error::ErrorInternalServerError("Session error"))?
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    let user_id = ObjectId::parse_str(&user_id_str)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid user ID in session"))?;

    // Parse chat ID
    let chat_id = body.chat_id.clone();

    // Check if user is a participant in the chat room
    let chats_collection = state.db.collection::<Chat>(Chat::collection_name());

    let chat = chats_collection
        .find_one(doc! { "_id": &chat_id, "participant_ids": &user_id }, None)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?
        .ok_or_else(|| actix_web::error::ErrorForbidden("You are not a participant in this chat"))?;

    // Create and insert the message
    let new_message = Message::new(chat_id.clone(), user_id.clone(), &body.content);
    let messages = state.db.collection::<Message>(Message::collection_name());

    messages.insert_one(new_message, None).await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to insert message"))?;

    Ok(HttpResponse::Ok().json("Message sent successfully"))
}


// NOTE SECURITY ISSUE
#[get("/chats/{chat_id}/messages")]
pub async fn get_chat_messages_handler(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    // Get user ID from session
    let session = req.get_session();
    let user_id_str = session.get::<String>("user_id")
        .map_err(|_| actix_web::error::ErrorInternalServerError("Session error"))?
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    let user_id = ObjectId::parse_str(&user_id_str)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid user ID in session"))?;

    // Parse chat room ID
    let chat_id_str = path.into_inner();
    let chat_id = ObjectId::parse_str(&chat_id_str)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid chat room ID"))?;

    // Check if the user is a participant in the chat room
    let chats_collection = state.db.collection::<Chat>(Chat::collection_name());

    let chat = chats_collection
        .find_one(doc! { "_id": &chat_id, "participant_ids": &user_id }, None)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?
        .ok_or_else(|| actix_web::error::ErrorForbidden("You are not a participant in this chat"))?;

    // Retrieve messages
    let messages_collection = state.db.collection::<Message>(Message::collection_name());

    let cursor = messages_collection
        .find(doc! { "chat_id": &chat_id }, None)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to get messages"))?;

    let results: Vec<Message> = cursor
        .try_collect()
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to collect messages"))?;

    Ok(HttpResponse::Ok().json(results))
}

#[get("/user/{user_id}/chats")]
pub async fn get_user_chats_handler(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    // Get user ID from session
    let session = req.get_session();
    let session_user_id_str = session.get::<String>("user_id")
        .map_err(|_| actix_web::error::ErrorInternalServerError("Session error"))?
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    let session_user_id = ObjectId::parse_str(&session_user_id_str)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid user ID in session"))?;

    // Parse user ID from path
    let user_id_str = path.into_inner();
    let user_id = ObjectId::parse_str(&user_id_str)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid user ID in path"))?;

    // Ensure the user is requesting their own chat rooms
    if user_id != session_user_id {
        return Err(actix_web::error::ErrorForbidden("You can only access your own chat rooms"));
    }

    // Retrieve chat rooms
    let chats_collection = state.db.collection::<Chat>(Chat::collection_name());

    let cursor = chats_collection
        .find(doc! { "participant_ids": &user_id }, None)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to get chat rooms"))?;

    let results: Vec<Chat> = cursor
        .try_collect()
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to collect chat rooms"))?;

    Ok(HttpResponse::Ok().json(results))
}


#[get("/user/{user_id}/online")]
pub async fn check_online_handler(
    _req: HttpRequest, // Authentication handled by middleware
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let user_id_str = path.into_inner();
    let user_id = ObjectId::parse_str(&user_id_str)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid user ID"))?;

    let ws_sessions = state.ws_sessions.read().await;

    let is_online = ws_sessions.contains_key(&user_id);

    Ok(HttpResponse::Ok().json(is_online))
}

#[derive(Debug, Deserialize)]
struct BatchCheckOnlineRequest {
    user_ids: Vec<String>,
}

#[derive(Debug, Serialize)]
struct BatchCheckOnlineResponse {
    online_status: Vec<(String, bool)>,
}

#[post("/user/batch_online")]
pub async fn batch_check_online_handler(
    _req: HttpRequest, // Authentication handled by middleware
    state: web::Data<AppState>,
    body: web::Json<BatchCheckOnlineRequest>,
) -> Result<HttpResponse, Error> {
    let user_ids = body
        .user_ids
        .iter()
        .filter_map(|id| ObjectId::parse_str(id).ok())
        .collect::<Vec<ObjectId>>();

    let ws_sessions = state.ws_sessions.read().await;

    let online_status = user_ids
        .into_iter()
        .map(|id| (id.to_hex(), ws_sessions.contains_key(&id)))
        .collect::<Vec<(String, bool)>>();

    Ok(HttpResponse::Ok().json(BatchCheckOnlineResponse { online_status }))
}

#[get("/notifications")]
pub async fn get_notifications_handler(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    // Get user ID from session
    let session = req.get_session();
    let user_id_str = session.get::<String>("user_id")
        .map_err(|_| actix_web::error::ErrorInternalServerError("Session error"))?
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    let user_id = ObjectId::parse_str(&user_id_str)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid user ID in session"))?;

    let notifications_collection = state.db.collection::<Notification>(Notification::collection_name());

    let cursor = notifications_collection
        .find(doc! { "user_id": &user_id }, None)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to get notifications"))?;

    let results: Vec<Notification> = cursor
        .try_collect()
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to collect notifications"))?;

    Ok(HttpResponse::Ok().json(results))
}
