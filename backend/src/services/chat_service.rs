use crate::{
    models::{chat_model::Chat, message_model::Message},
    states::app_state::AppState,
};
use actix_web::{
    error::{ErrorForbidden, ErrorInternalServerError},
    Error,
};
use futures::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId};
use serde::Deserialize;
use std::collections::HashSet;

#[derive(Debug, Deserialize)]
pub struct CreateChatRoomRequest {
    pub participant_ids: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct DeleteChatRequest {
    pub chat_id: ObjectId,
}

#[derive(Debug, Deserialize)]
pub struct SendMessageRequest {
    pub chat_id: ObjectId,
    pub content: String,
}

/// Retrieve chat rooms for a given user.
pub async fn get_user_chats(state: &AppState, user_id: ObjectId) -> Result<Vec<Chat>, Error> {
    let chats_collection = state.db.collection::<Chat>(Chat::collection_name());
    let cursor = chats_collection
        .find(doc! { "participant_ids": &user_id }, None)
        .await
        .map_err(|_| ErrorInternalServerError("Failed to get chat rooms"))?;
    let results: Vec<Chat> = cursor
        .try_collect()
        .await
        .map_err(|_| ErrorInternalServerError("Failed to collect chat rooms"))?;
    Ok(results)
}

/// Retrieve a chat room by its ID.
pub async fn get_chat_by_id(state: &AppState, chat_id: ObjectId) -> Result<Chat, Error> {
    let chats = state.db.collection::<Chat>(Chat::collection_name());
    let chat = chats
        .find_one(doc! { "_id": &chat_id }, None)
        .await
        .map_err(|_| ErrorInternalServerError("Database error retrieving chat"))?
        .ok_or_else(|| ErrorForbidden("Chat not found"))?;
    Ok(chat)
}


/// Create a new chat room.
pub async fn create_chat(
    state: &AppState,
    chat_id: Option<ObjectId>,
    participant_ids: HashSet<ObjectId>,
) -> Result<Chat, Error> {
    // Create a new Chat document.
    let new_chat = Chat::new(chat_id, participant_ids.into_iter().collect(), None);
    let chats = state.db.collection::<Chat>(Chat::collection_name());
    chats
        .insert_one(new_chat.clone(), None)
        .await
        .map_err(|_| ErrorInternalServerError("Failed to create chat room"))?;
    Ok(new_chat)
}

/// Delete a chat room if the given user is a participant.
pub async fn delete_chat(
    state: &AppState,
    chat_id: ObjectId,
    user_id: ObjectId,
) -> Result<(), Error> {
    let chats = state.db.collection::<Chat>(Chat::collection_name());
    let delete_result = chats
        .delete_one(doc! { "_id": &chat_id, "participant_ids": &user_id }, None)
        .await
        .map_err(|_| ErrorInternalServerError("Database error during deletion"))?;
    if delete_result.deleted_count == 0 {
        Err(ErrorForbidden("You are not a participant in this chat"))
    } else {
        Ok(())
    }
}

/// Send a message in a chat room.  
/// First verifies that the user is a participant.
pub async fn send_message(
    state: &AppState,
    chat_id: ObjectId,
    user_id: ObjectId,
    content: &str,
) -> Result<(), Error> {
    let chats = state.db.collection::<Chat>(Chat::collection_name());
    // Verify the user is a participant in the chat.
    chats
        .find_one(doc! { "_id": &chat_id, "participant_ids": &user_id }, None)
        .await
        .map_err(|_| ErrorInternalServerError("Database error during participation check"))?
        .ok_or_else(|| ErrorForbidden("You are not a participant in this chat"))?;

    // Insert the new message
    let new_message = Message::new(chat_id.clone(), user_id, content);
    let messages = state.db.collection::<Message>(Message::collection_name());
    messages
        .insert_one(new_message, None)
        .await
        .map_err(|_| ErrorInternalServerError("Failed to insert message"))?;
    Ok(())
}

/// Retrieve all messages for a chat room if the user is a participant.
pub async fn get_chat_messages(
    state: &AppState,
    chat_id: ObjectId,
    user_id: ObjectId,
) -> Result<Vec<Message>, Error> {
    let chats = state.db.collection::<Chat>(Chat::collection_name());
    // Verify that the user is a participant.
    chats
        .find_one(doc! { "_id": &chat_id, "participant_ids": &user_id }, None)
        .await
        .map_err(|_| ErrorInternalServerError("Database error during participation check"))?
        .ok_or_else(|| ErrorForbidden("You are not a participant in this chat"))?;

    let messages_coll = state.db.collection::<Message>(Message::collection_name());
    let cursor = messages_coll
        .find(doc! { "chat_id": &chat_id }, None)
        .await
        .map_err(|_| ErrorInternalServerError("Failed to get messages"))?;
    let messages: Vec<Message> = cursor
        .try_collect()
        .await
        .map_err(|_| ErrorInternalServerError("Failed to collect messages"))?;
    Ok(messages)
}
