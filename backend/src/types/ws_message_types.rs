use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WsMessageType {
    Logout,
    ChatMessage,
    WebrtcOffer,
    WebrtcAnswer,
    WebrtcIceCandidate,
    VideoCallAccepted,
    VideoCallDeclined,
    Unknown,
}

impl From<&str> for WsMessageType {
    fn from(value: &str) -> Self {
        match value {
            "logout" => Self::Logout,
            "chat_message" => Self::ChatMessage,
            "webrtc_offer" => Self::WebrtcOffer,
            "webrtc_answer" => Self::WebrtcAnswer,
            "webrtc_ice_candidate" => Self::WebrtcIceCandidate,
            "video_call_accepted" => Self::VideoCallAccepted,
            "video_call_declined" => Self::VideoCallDeclined,
            _ => Self::Unknown,
        }
    }
}