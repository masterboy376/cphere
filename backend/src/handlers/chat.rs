use actix_session::SessionExt;
use actix_web::{get, web, HttpRequest, HttpResponse, Error};
use actix_web_actors::ws;
use mongodb::bson::oid::ObjectId;
use serde::Deserialize;

use crate::{
    states::app_state::AppState,
    websocket::session::WsChatSession,
    models::message::Message,
    middleware::auth_middleware::AuthMiddlewareFactory, // Import your auth middleware
};

#[derive(Debug, Deserialize)]
struct CreateChatRoomRequest {
    participant_ids: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct SendMessageRequest {
    chat_id: ObjectId,
    content: String,
}

#[get("/init")]
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


#[post("/chats/create")]
pub async fn create_new_chat_handler(
    state: web::Data<AppState>,
    body: web::Json<CreateChatRoomRequest>,
) -> Result<HttpResponse, Error> {
    let participant_ids = body
        .participant_ids
        .iter()
        .filter_map(|id| ObjectId::parse_str(id).ok())
        .collect::<Vec<ObjectId>>();
    
    let new_chat = Chat::new(participant_ids);
    let chats = state.db.collection::<Chat>(Chat::collection_name());

    chats.insert_one(new_chat.clone(), None).await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to create chat room"))?;
    
    Ok(HttpResponse::Ok().json(new_chat))
}


// REST API handlers for sending messsage, when the user is not connected to the WebSocket.
#[post("/message")]
pub async fn send_message_handler(
    req: HttpRequest,
    state: web::Data<AppState>,
    body: web::Json<SendMessageRequest>,
) -> Result<HttpResponse, Error> {
    let session = req.get_session();
    let user_id_str = session.get::<String>("user_id").unwrap_or(None).unwrap_or_default();

    let user_id = ObjectId::parse_str(&user_id_str)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid user ID"))?;
    
    let new_message = Message::new(body.chat_id.clone(), user_id, &body.content);
    let messages = state.db.collection::<Message>(Message::collection_name());

    messages.insert_one(new_message, None).await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to insert message"))?;

    Ok(HttpResponse::Ok().finish())
}


#[get("/user/{user_id}/online")]
pub async fn is_online_handler(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<ObjectId>,
) -> Result<HttpResponse, Error> {
    let user_id = path.into_inner();
    let ws_sessions = state.ws_sessions.read().await;
    
    if ws_sessions.contains_key(&user_id) {
        Ok(HttpResponse::Ok().json(true))
    } else {
        Ok(HttpResponse::Ok().json(false))
    }
}


// Function to register routes in this module.
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(ws_session_start_handler);
}
