use actix_web::{test, App, http::StatusCode};
use cphere_backend::handlers::chat::chat_ws_handler;

#[actix_web::test]
async fn test_ws_chat_handler() {
    let app = test::init_service(App::new().service(chat_ws_handler)).await;
    let req = test::TestRequest::get().uri("/ws/chat").to_request();
    let resp = test::call_service(&app, req).await;
    // For a WebSocket upgrade, we expect a 101 Switching Protocols status.
    assert_eq!(resp.status(), StatusCode::SWITCHING_PROTOCOLS);
}
