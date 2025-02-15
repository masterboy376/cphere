// src/services/chat.rs
use crate::models::chat::Chat;
use crate::models::message::Message;
use mongodb::bson::oid::ObjectId;
use crate::config::database::{DbError, init_db};
use mongodb::error::Error as MongoError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ChatError {
    #[error("Database error: {0}")]
    Db(#[from] DbError),
    #[error("MongoDB error: {0}")]
    Mongo(#[from] MongoError),
}

/// Creates a chat room between two users and stores it in the database.
pub async fn create_chat(participants: Vec<ObjectId>) -> Result<Chat, ChatError> {
    let new_chat = Chat::new(None, participants, None);

    let (client, db) = init_db().await?;
    let collection = db.collection::<Chat>(Chat::collection_name());
    collection.insert_one(new_chat.clone(), None).await?;

    Ok(new_chat)
}

/// Creates a new message in the specified chat and stores it in the database.
pub async fn send_message(
    chat_id: ObjectId,
    sender_id: ObjectId,
    content: &str,
) -> Result<Message, ChatError> {
    let new_message = Message::new(chat_id, sender_id, content);

    let (client, db) = init_db().await?;
    let collection = db.collection::<Message>("messages");
    collection.insert_one(new_message.clone(), None).await?;

    Ok(new_message)
}