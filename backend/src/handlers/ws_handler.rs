use crate::{
    services::user_service::extract_user_id_from_session,
    states::app_state::AppState,
    websocket::websocket_session::WsSession,
};
use actix_session::SessionExt;
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

// Handshake must use a get method, as per the WebSocket protocol (RFC 6455)
#[get("/connect")]
pub async fn ws_session_start_handler(
    req: HttpRequest,
    stream: web::Payload,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    // Extract the user ID from the session
    let session = req.get_session();
    // Parse the user ID as an ObjectId
    let user_id = extract_user_id_from_session(&session)?;
    // Start the WebSocket connection
    ws::start(WsSession::new(user_id, state.clone()), &req, stream)
}
