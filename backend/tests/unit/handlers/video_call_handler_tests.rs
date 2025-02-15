use actix_web::{test, App};
use serde_json::json;
use cphere_backend::handlers::video_call::webrtc_signaling;

#[actix_web::test]
async fn test_webrtc_signaling_handler() {
    let app = test::init_service(App::new().service(webrtc_signaling)).await;
    
    let payload = json!({
        "sdp": "test sdp",
        "candidate": "test candidate"
    });
    
    let req = test::TestRequest::post()
        .uri("/webrtc/signaling")
        .set_json(&payload)
        .to_request();
    
    let resp: serde_json::Value = test::read_body_json(test::call_service(&app, req).await).await;
    
    // We expect the signaling handler to echo back the payload.
    assert_eq!(resp, payload);
}
