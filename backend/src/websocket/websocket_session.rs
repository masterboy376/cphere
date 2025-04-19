use crate::{
    services::chat_service::{get_chat_by_id, send_message},
    services::user_service::get_user_by_id,
    states::app_state::AppState,
    types::ws_message_types::WsMessageType,
};
use actix::prelude::*;
use actix::{Actor, AsyncContext, Handler, StreamHandler};
use actix_web_actors::ws;
use mongodb::bson::oid::ObjectId;
use serde_json::Value;
use std::collections::HashSet;

#[derive(Message)]
#[rtype(result = "()")]
pub struct TextMessage(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct StopSession;

pub struct WsSession {
    pub user_id: ObjectId,
    pub state: actix_web::web::Data<AppState>,
}

impl WsSession {
    pub fn new(user_id: ObjectId, state: actix_web::web::Data<AppState>) -> Self {
        Self { user_id, state }
    }
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let ws_sessions = self.state.ws_sessions.clone();
        let user_id = self.user_id;
        let addr = ctx.address();

        ctx.spawn(
            async move {
                let mut sessions = ws_sessions.write().await;
                if let Some(existing_session) = sessions.get(&user_id) {
                    existing_session.do_send(StopSession);
                }
                sessions.insert(user_id, addr);
            }
            .into_actor(self),
        );
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        let ws_sessions = self.state.ws_sessions.clone();
        let user_id = self.user_id;

        ctx.spawn(
            async move {
                ws_sessions.write().await.remove(&user_id);
            }
            .into_actor(self),
        );
    }
}

impl Handler<StopSession> for WsSession {
    type Result = ();

    fn handle(&mut self, _msg: StopSession, ctx: &mut Self::Context) -> Self::Result {
        ctx.close(None);
        ctx.stop();
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(ws::Message::Text(text)) = msg {
            let text_string = text.to_string();

            let state = self.state.clone();
            let user_id = self.user_id;

            ctx.spawn(
                async move {
                    let msg_json: Value = match serde_json::from_str(&text_string) {
                        Ok(val) => val,
                        Err(e) => {
                            eprintln!("Failed to parse JSON: {}", e);
                            return;
                        }
                    };

                    // Handle different message types.
                    if let Some(message_type_str) = msg_json.get("type").and_then(|v| v.as_str()) {
                        let message_type = WsMessageType::from(message_type_str);
                        match message_type {
                            WsMessageType::Logout => {
                                if let Some(addr) = state.ws_sessions.read().await.get(&user_id) {
                                    addr.do_send(StopSession);
                                }
                            }
                            WsMessageType::ChatMessage => {
                                handle_chat_message(msg_json, &state, user_id).await;
                            }
                            WsMessageType::WebrtcOffer
                            | WsMessageType::WebrtcAnswer
                            | WsMessageType::WebrtcIceCandidate => {
                                handle_webrtc_signaling(msg_json, &state).await;
                            }
                            WsMessageType::VideoCallAccepted | WsMessageType::VideoCallDeclined => {
                                handle_video_call_response(msg_json, &state, user_id).await;
                            }
                            WsMessageType::Unknown => {
                                eprintln!("Unknown message type: {}", message_type_str);
                            }
                        }
                    } else {
                        eprintln!("Message type not specified");
                    }
                }
                .into_actor(self),
            );
        }
    }
}

impl Handler<TextMessage> for WsSession {
    type Result = ();

    fn handle(&mut self, msg: TextMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

// Helper functions for handling messages.

async fn handle_chat_message(
    msg_json: Value,
    state: &actix_web::web::Data<AppState>,
    user_id: ObjectId,
) {
    // Extract or generate a chat_id.
    let chat_id = if let Some(chat_id_str) = msg_json.get("chat_id").and_then(|v| v.as_str()) {
        match ObjectId::parse_str(chat_id_str) {
            Ok(oid) => oid,
            Err(e) => {
                eprintln!("Invalid chat_id: {}", e);
                return;
            }
        }
    } else {
        ObjectId::new()
    };

    let chat_id_str = chat_id.to_string();

    let content = match msg_json.get("content").and_then(|v| v.as_str()) {
        Some(c) => c,
        None => {
            eprintln!("Content not found in message");
            return;
        }
    };

    let created_at = match msg_json.get("created_at").and_then(|v| v.as_str()) {
        Some(c) => match chrono::DateTime::parse_from_rfc3339(c) {
            Ok(dt) => Some(dt.with_timezone(&chrono::Utc)),
            Err(e) => {
                eprintln!("Failed to parse created_at: {}", e);
                None
            }
        },
        None => None,
    };

    // Lock the chats map.
    let mut chats = state.chats.write().await;

    if !chats.contains_key(&chat_id_str) {
        // Create a new chat and add the sender.
        let mut participant_ids = HashSet::new();
        participant_ids.insert(user_id);

        if let Ok(chat) = get_chat_by_id(&state, chat_id.clone()).await {
            for participant_id in chat.participant_ids {
                participant_ids.insert(participant_id);
            }
            chats.insert(chat_id_str.clone(), participant_ids.clone());
        }
    }

    drop(chats);

    // Store the message in the database.
    let message_result = send_message(
        &state,
        chat_id.clone(),
        user_id.clone(),
        content,
        created_at,
    )
    .await;
    if let Err(e) = message_result {
        eprintln!("Failed to send message: {:?}", e);
        return;
    }

    let message = message_result.unwrap();

    // Get sender username
    let sender = match get_user_by_id(&state, user_id.clone()).await {
        Ok(user) => user,
        Err(e) => {
            eprintln!("Failed to get sender user: {:?}", e);
            return;
        }
    };

    // Broadcast the message to other participants.
    let chats = state.chats.read().await;
    if let Some(participant_ids) = chats.get(&chat_id_str) {
        for participant_id in participant_ids {
            if let Some(addr) = state.ws_sessions.read().await.get(participant_id) {
                let mut outgoing_msg = serde_json::Map::new();
                outgoing_msg.insert(
                    "type".to_string(),
                    Value::String("chat_message".to_string()),
                );
                outgoing_msg.insert(
                    "message_id".to_string(),
                    Value::String(message.id.unwrap().to_hex()),
                );
                outgoing_msg.insert("chat_id".to_string(), Value::String(chat_id.to_hex()));
                outgoing_msg.insert("sender_id".to_string(), Value::String(user_id.to_hex()));
                outgoing_msg.insert(
                    "sender_username".to_string(),
                    Value::String(sender.username.clone()),
                );
                outgoing_msg.insert("content".to_string(), Value::String(content.to_string()));
                outgoing_msg.insert(
                    "created_at".to_string(),
                    Value::String(message.created_at.to_string()),
                );

                let message_text = Value::Object(outgoing_msg).to_string();
                let _ = addr.do_send(TextMessage(message_text));
            }
        }
    }
}

async fn handle_webrtc_signaling(msg_json: Value, state: &actix_web::web::Data<AppState>) {
    if let Some(target_user_id_str) = msg_json.get("target_user_id").and_then(|v| v.as_str()) {
        if let Ok(target_user_id) = ObjectId::parse_str(target_user_id_str) {
            let ws_sessions = state.ws_sessions.read().await;
            if let Some(target_addr) = ws_sessions.get(&target_user_id) {
                let message_text = msg_json.to_string();
                let _ = target_addr.do_send(TextMessage(message_text));
            } else {
                eprintln!("Target user is not online");
            }
        } else {
            eprintln!("Invalid target user ID");
        }
    } else {
        eprintln!("Target user ID not provided");
    }
}

async fn handle_video_call_response(
    msg_json: Value,
    state: &actix_web::web::Data<AppState>,
    _user_id: ObjectId,
) {
    if let Some(caller_id_str) = msg_json.get("caller_id").and_then(|v| v.as_str()) {
        if let Ok(caller_id) = ObjectId::parse_str(caller_id_str) {
            let ws_sessions = state.ws_sessions.read().await;
            if let Some(target_addr) = ws_sessions.get(&caller_id) {
                let message_text = msg_json.to_string();
                let _ = target_addr.do_send(TextMessage(message_text));
            } else {
                log::warn!("Caller is not online");
            }
        } else {
            log::warn!("Invalid caller ID format");
        }
    } else {
        log::warn!("Caller ID not provided");
    }
}
