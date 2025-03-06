use crate::{
    services::{
        user_service::extract_user_id_from_session,
        video_call_service::{VideoCallRequest, VideoCallResponse,
        initiate_video_call_logic, respond_video_call_logic},
    },
    states::app_state::AppState,
};
use actix_session::SessionExt;
use actix_web::{post, web, Error, HttpRequest, HttpResponse};
use mongodb::bson::oid::ObjectId;

#[post("/initiate")]
pub async fn initiate_video_call(
    req: HttpRequest,
    state: web::Data<AppState>,
    body: web::Json<VideoCallRequest>,
) -> Result<HttpResponse, Error> {
    // Get the caller's user ID from the session
    let session = req.get_session();
    let caller_id = extract_user_id_from_session(&session)?;

    // Parse recipient_id and chat_id
    let recipient_id = ObjectId::parse_str(&body.recipient_id)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid recipient ID"))?;
    let chat_id = ObjectId::parse_str(&body.chat_id)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid chat ID"))?;

    // Initiate video call logic
    initiate_video_call_logic(&state, caller_id, recipient_id, chat_id).await?;

    Ok(HttpResponse::Ok().json("Video call request sent"))
}

#[post("/respond")]
pub async fn respond_video_call(
    req: HttpRequest,
    state: web::Data<AppState>,
    body: web::Json<VideoCallResponse>,
) -> Result<HttpResponse, Error> {
    // Get the recipient's user ID from the session
    let session = req.get_session();
    let recipient_id = extract_user_id_from_session(&session)?;

    // Parse notification_id
    let notification_id = ObjectId::parse_str(&body.notification_id)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid notification ID"))?;

    // Respond to video call logic
    respond_video_call_logic(&state, recipient_id, notification_id, body.accepted).await?;

    Ok(HttpResponse::Ok().json("Response sent"))
}
