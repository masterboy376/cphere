// src/webrtc/mod.rs

use actix_web::{HttpResponse, web, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SignalRequest {
    pub sdp: Option<String>,
    pub candidate: Option<String>,
}

pub async fn signaling_handler(req: web::Json<SignalRequest>) -> impl Responder {
    // In a real implementation, process the signaling data.
    // Here, we simply echo the data back.
    HttpResponse::Ok().json(req.into_inner())
}
