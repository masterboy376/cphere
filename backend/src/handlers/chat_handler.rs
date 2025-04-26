use crate::{
    services::{
        chat_service::{
            create_chat, delete_chat, get_chat_messages, get_chat_summary, send_message,
            CreateChatRoomRequest, DeleteChatRequest, SendMessageRequest,
        },
        user_service::extract_user_id_from_session,
    },
    states::app_state::AppState,
};
use actix_session::SessionExt;
use actix_web::{get, post, web, Error, HttpRequest, HttpResponse};
use mongodb::bson::oid::ObjectId;
use std::collections::HashSet;

#[get("/{chat_id}/get_chat_summary")]
pub async fn get_chat_summary_handler(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let session = req.get_session();
    let user_id = extract_user_id_from_session(&session)?;

    let chat_id_str = path.into_inner();
    let chat_id = ObjectId::parse_str(chat_id_str)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid chat room ID"))?;

    let chat_summary = get_chat_summary(&state, chat_id, user_id).await?;

    Ok(HttpResponse::Ok().json(chat_summary))
}

#[post("/create")]
pub async fn create_new_chat_handler(
    req: HttpRequest,
    state: web::Data<AppState>,
    body: web::Json<CreateChatRoomRequest>,
) -> Result<HttpResponse, Error> {
    let session = req.get_session();
    let user_id = extract_user_id_from_session(&session)?;

    let participant_id = ObjectId::parse_str(&body.participant_id)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid participant ID format"))?;

    let mut participant_ids = HashSet::new();
    participant_ids.insert(participant_id);
    participant_ids.insert(user_id.clone());

    let new_chat = create_chat(&state, None, participant_ids).await?;

    Ok(HttpResponse::Ok().json(new_chat))
}

#[post("/delete")]
pub async fn delete_chat_handler(
    req: HttpRequest,
    state: web::Data<AppState>,
    body: web::Json<DeleteChatRequest>,
) -> Result<HttpResponse, Error> {
    let session = req.get_session();
    let user_id = extract_user_id_from_session(&session)?;

    let chat_id = body.chat_id.clone();

    delete_chat(&state, chat_id, user_id).await?;

    Ok(HttpResponse::Ok().json("Chat deleted successfully"))
}

#[post("/send_message")]
pub async fn send_message_handler(
    req: HttpRequest,
    state: web::Data<AppState>,
    body: web::Json<SendMessageRequest>,
) -> Result<HttpResponse, Error> {
    let session = req.get_session();
    let user_id = extract_user_id_from_session(&session)?;

    let chat_id = body.chat_id.clone();

    send_message(&state, chat_id, user_id, &body.content, None).await?;

    Ok(HttpResponse::Ok().json("Message sent successfully"))
}

// NOTE SECURITY ISSUE
#[get("/{chat_id}/messages")]
pub async fn get_chat_messages_handler(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let session = req.get_session();
    let user_id = extract_user_id_from_session(&session)?;

    let chat_id_str = path.into_inner();
    let chat_id = ObjectId::parse_str(&chat_id_str)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid chat room ID"))?;

    let messages = get_chat_messages(&state, chat_id, user_id).await?;

    Ok(HttpResponse::Ok().json(messages))
}
