use crate::websocket::websocket_session::TextMessage;
use actix::Recipient;
use mongodb::{bson::oid::ObjectId, Client, Database};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};
use tokio::sync::RwLock;

pub struct AppState {
    pub ws_sessions: Arc<RwLock<HashMap<ObjectId, Recipient<TextMessage>>>>,
    pub chats: Arc<RwLock<HashMap<String, HashSet<ObjectId>>>>,
    pub mongo_client: Client,
    pub db: Database,
}

impl AppState {
    pub fn new(client: Client, db: Database) -> Self {
        Self {
            ws_sessions: Arc::new(RwLock::new(HashMap::new())),
            chats: Arc::new(RwLock::new(HashMap::new())),
            mongo_client: client,
            db,
        }
    }
}
