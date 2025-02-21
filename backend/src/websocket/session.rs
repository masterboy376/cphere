use actix::{Actor, AsyncContext, Context, Handler, Recipient, StreamHandler};
use actix::prelude::*;
use actix_web_actors::ws;
use mongodb::bson::oid::ObjectId;
use serde_json::Value;
use std::collections::HashSet;

use crate::{
    models::{chat::Chat, message::Message, notification::Notification},
    states::app_state::AppState,
};

#[derive(Message)]
#[rtype(result = "()")]
pub struct TextMessage(pub String);

pub struct WsChatSession {
    pub user_id: ObjectId,
    pub state: actix_web::web::Data<AppState>,
}

impl WsChatSession {
    pub fn new(user_id: ObjectId, state: actix_web::web::Data<AppState>) -> Self {
        Self { user_id, state }
    }
}

impl Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let ws_sessions = self.state.ws_sessions.clone();
        let user_id = self.user_id;
        let addr = ctx.address().recipient();

        ctx.spawn(
            async move {
                ws_sessions.write().await.insert(user_id, addr);
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

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(ws::Message::Text(text)) = msg {
            let text_string = text.to_string();

            let state = self.state.clone();
            let user_id = self.user_id;

            ctx.spawn(
                async move {
                    // Parse the incoming JSON message
                    let msg_json: Value = match serde_json::from_str(&text_string) {
                        Ok(val) => val,
                        Err(e) => {
                            eprintln!("Failed to parse JSON: {}", e);
                            return;
                        }
                    };

                    // Handle different message types
                    if let Some(message_type) = msg_json.get("type").and_then(|v| v.as_str()) {
                        match message_type {
                            "chat_message" => {
                                handle_chat_message(msg_json, &state, user_id).await;
                            }
                            "webrtc_offer" | "webrtc_answer" | "webrtc_ice_candidate" => {
                                handle_webrtc_signaling(msg_json, &state, user_id).await;
                            }
                            _ => {
                                eprintln!("Unknown message type: {}", message_type);
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

impl Handler<TextMessage> for WsChatSession {
    type Result = ();

    fn handle(&mut self, msg: TextMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

// Helper functions for handling different message types

async fn handle_chat_message(msg_json: Value, state: &actix_web::web::Data<AppState>, user_id: ObjectId) {
    // Extract or generate chat_id
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

    // Lock the chats map for writing
    let mut chats = state.chats.write().await;

    if !chats.contains_key(&chat_id_str) {
        // New chat; create it
        let mut participants = HashSet::new();
        participants.insert(user_id); // Add sender

        // Extract recipient_ids
        if let Some(recipient_ids) = msg_json.get("recipient_ids").and_then(|v| v.as_array()) {
            for recipient_id_value in recipient_ids {
                if let Some(recipient_id_str) = recipient_id_value.as_str() {
                    if let Ok(recipient_oid) = ObjectId::parse_str(recipient_id_str) {
                        participants.insert(recipient_oid);
                    }
                }
            }
        }

        // Insert the new chat into the chats map
        chats.insert(chat_id_str.clone(), participants.clone());

        // Persist the new chat
        let chats_collection = state.db.collection::<Chat>(Chat::collection_name());
        let new_chat = Chat::new(Some(chat_id.clone()), participants.iter().cloned().collect(), None);
        if let Err(e) = chats_collection.insert_one(new_chat, None).await {
            eprintln!("Failed to insert chat into database: {}", e);
        }
    }

    // Store the message in the database
    let messages_collection = state.db.collection::<Message>(Message::collection_name());
    let message = Message::new(chat_id.clone(), user_id, content);
    if let Err(e) = messages_collection.insert_one(message, None).await {
        eprintln!("Failed to insert message into database: {}", e);
    }

    // Broadcast the message to other participants
    let chats = state.chats.read().await;
    if let Some(participants) = chats.get(&chat_id_str) {
        for participant in participants {
            if participant != &user_id {
                if let Some(addr) = state.ws_sessions.read().await.get(participant) {
                    let mut outgoing_msg = serde_json::Map::new();
                    outgoing_msg.insert("type".to_string(), Value::String("chat_message".to_string()));
                    outgoing_msg.insert("chat_id".to_string(), Value::String(chat_id.to_hex()));
                    outgoing_msg.insert("content".to_string(), Value::String(content.to_string()));
                    outgoing_msg.insert("sender_id".to_string(), Value::String(user_id.to_hex()));

                    let message_text = serde_json::Value::Object(outgoing_msg).to_string();
                    let _ = addr.do_send(TextMessage(message_text));
                }
            }
        }
    }
}

async fn handle_webrtc_signaling(msg_json: Value, state: &actix_web::web::Data<AppState>, user_id: ObjectId) {
    if let Some(target_user_id_str) = msg_json.get("target_user_id").and_then(|v| v.as_str()) {
        if let Ok(target_user_id) = ObjectId::parse_str(target_user_id_str) {
            let ws_sessions = state.ws_sessions.read().await;
            if let Some(target_addr) = ws_sessions.get(&target_user_id) {
                // Relay the signaling message to the target user
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
