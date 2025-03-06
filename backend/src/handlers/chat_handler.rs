use crate::{
    services::{
        user_service::extract_user_id_from_session,
        chat_service::{CreateChatRoomRequest, DeleteChatRequest, SendMessageRequest,
            create_chat, delete_chat, send_message, get_chat_messages},
    },
    states::app_state::AppState,
};
use actix_session::SessionExt;
use actix_web::{get, post, web, Error, HttpRequest, HttpResponse};
use mongodb::bson::oid::ObjectId;
use std::collections::HashSet;

#[post("/create_new_chat")]
pub async fn create_new_chat_handler(
    req: HttpRequest,
    state: web::Data<AppState>,
    body: web::Json<CreateChatRoomRequest>,
) -> Result<HttpResponse, Error> {
    // Get user ID from session
    let session = req.get_session();
    let user_id = extract_user_id_from_session(&session)?;

    // Parse participant IDs
    let mut participant_ids = body
        .participant_ids
        .iter()
        .filter_map(|id| ObjectId::parse_str(id).ok())
        .collect::<HashSet<ObjectId>>();

    // Add the creator to the participants if not already included
    participant_ids.insert(user_id.clone());

    // Create the chat room
    let new_chat = create_chat(&state, None, participant_ids).await?;

    Ok(HttpResponse::Ok().json(new_chat))
}

#[post("/delete_chat")]
pub async fn delete_chat_handler(
    req: HttpRequest,
    state: web::Data<AppState>,
    body: web::Json<DeleteChatRequest>,
) -> Result<HttpResponse, Error> {
    // Get user ID from session
    let session = req.get_session();
    let user_id = extract_user_id_from_session(&session)?;

    // Parse chat ID
    let chat_id = body.chat_id.clone();

    // Call the service to delete the chat room
    delete_chat(&state, chat_id, user_id).await?;
    
    Ok(HttpResponse::Ok().json("Chat deleted successfully"))
}

// REST API handlers for sending messsage, when the user is not connected to the WebSocket.
#[post("/send_message")]
pub async fn send_message_handler(
    req: HttpRequest,
    state: web::Data<AppState>,
    body: web::Json<SendMessageRequest>,
) -> Result<HttpResponse, Error> {
    // Get user ID from session
    let session = req.get_session();
    let user_id = extract_user_id_from_session(&session)?;

    // Parse chat ID
    let chat_id = body.chat_id.clone();

    send_message(&state, chat_id, user_id, &body.content).await?;

    Ok(HttpResponse::Ok().json("Message sent successfully"))
}

// NOTE SECURITY ISSUE
#[get("/{chat_id}/messages")]
pub async fn get_chat_messages_handler(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    // Get user ID from session
    let session = req.get_session();
    let user_id = extract_user_id_from_session(&session)?;

    // Parse chat room ID
    let chat_id_str = path.into_inner();
    let chat_id = ObjectId::parse_str(&chat_id_str)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid chat room ID"))?;

    let messages = get_chat_messages(&state, chat_id, user_id).await?;

    Ok(HttpResponse::Ok().json(messages))
}
