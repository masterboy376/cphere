use actix_web::{post, web, Responder};
use crate::webrtc::{signaling_handler, SignalRequest};

#[post("/webrtc/signaling")]
async fn webrtc_signaling(req: web::Json<SignalRequest>) -> impl Responder {
    signaling_handler(req).await
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(webrtc_signaling);
}
