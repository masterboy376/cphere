use actix::{Actor, AsyncContext, Context, Handler, Recipient, StreamHandler};
use actix::prelude::*;
use actix_web_actors::ws;
use mongodb::bson::oid::ObjectId;
use serde_json::Value;
use std::collections::HashSet;

use crate::{
    models::chat::Chat,
    models::message::Message,
    states::app_state::AppState,
};

#[derive(actix::Message)]
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
        // When you wrap AppState in actix_web::web::Data, it internally uses an Arc<AppState>. This means that
        // cloning web::Data<AppState> creates a new reference to the same AppState, not a deep copy.
        let ws_sessions = self.state.ws_sessions.clone();
        let user_id = self.user_id;
        let addr = ctx.address().recipient();

        // Spawn an async task(using future) insidde an sync handler to insert the session.
        ctx.spawn(async move {
            ws_sessions.write().await.insert(user_id, addr);
        }
        .into_actor(self)); // Convert the future to an actor future
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        let ws_sessions = self.state.ws_sessions.clone();
        let user_id = self.user_id;

        // Spawn an async task to remove the session
        ctx.spawn(async move {
            ws_sessions.write().await.remove(&user_id);
        }
        .into_actor(self));
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(ws::Message::Text(text)) = msg {
            // ByteString implements the Display trait, which allows you to use to_string() to get a String.
            let text_string = text.to_string();

            let state = self.state.clone();
            let user_id = self.user_id;

            // Spawn an async task to handle the message
            ctx.spawn(async move {
                // Parse the incoming JSON message
                let msg_json: Value = match serde_json::from_str(&text_string) {
                    Ok(val) => val,
                    Err(e) => {
                        eprintln!("Failed to parse JSON: {}", e);
                        return;
                    }
                };

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
                        eprintln!("content not found in message");
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
                let messages = state.db.collection::<Message>(Message::collection_name());
                let message = Message::new(chat_id.clone(), user_id, content);
                if let Err(e) = messages.insert_one(message, None).await {
                    eprintln!("Failed to insert message into database: {}", e);
                }

                // Broadcast the message to other participants
                let chats = state.chats.read().await;
                if let Some(participants) = chats.get(&chat_id.to_string()) {
                    for participant in participants {
                        if participant != &user_id {
                            if let Some(addr) = state.ws_sessions.read().await.get(participant) {
                                // Send the message to the participant
                                let _ = addr.do_send(TextMessage(text_string.clone()));
                            }
                        }
                    }
                }
            }
            .into_actor(self));
        }
    }
}

impl Handler<TextMessage> for WsChatSession {
    type Result = ();

    fn handle(&mut self, msg: TextMessage, ctx: &mut Self::Context) {
        // Send the text message to the WebSocket client
        ctx.text(msg.0);
    }
}
