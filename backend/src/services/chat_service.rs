use crate::{
    models::{chat_model::Chat, message_model::Message, user_model::User},
    states::app_state::AppState,
};
use actix_web::{
    error::{ErrorForbidden, ErrorInternalServerError},
    Error,
};
use futures::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Deserialize)]
pub struct CreateChatRoomRequest {
    pub participant_id: String,
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
#[derive(Debug, Serialize)]
pub struct ChatSummary {
    pub id: String,
    pub participant_username: String,
    pub participant_user_id: String,
    pub last_message: Option<String>,
    pub last_message_timestamp: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn get_user_chats(
    state: &AppState,
    user_id: ObjectId,
) -> Result<Vec<ChatSummary>, Error> {
    let chats_collection = state.db.collection::<Chat>(Chat::collection_name());
    let users_collection = state
        .db
        .collection::<crate::models::user_model::User>("users");
    let messages_collection = state.db.collection::<Message>(Message::collection_name());

    // Get all chats for this user
    let cursor = chats_collection
        .find(doc! { "participant_ids": &user_id }, None)
        .await
        .map_err(|_| ErrorInternalServerError("Failed to get chat rooms"))?;

    let chats: Vec<Chat> = cursor
        .try_collect()
        .await
        .map_err(|_| ErrorInternalServerError("Failed to collect chat rooms"))?;

    let mut chat_summaries = Vec::new();

    for chat in chats {
        // Find the other participant (assuming 2-person chats)
        let other_participant_id = chat
            .participant_ids
            .iter()
            .find(|&id| id != &user_id)
            .ok_or_else(|| ErrorInternalServerError("Failed to find other participant"))?;

        // Get other participant's username
        let other_user = users_collection
            .find_one(doc! { "_id": other_participant_id }, None)
            .await
            .map_err(|_| ErrorInternalServerError("Failed to get other participant info"))?
            .ok_or_else(|| ErrorInternalServerError("Other participant not found"))?;

        // Get last message
        let mut last_message_cursor = messages_collection
            .find(
                doc! { "chat_id": &chat.id },
                Some(
                    mongodb::options::FindOptions::builder()
                        .sort(doc! { "created_at": -1 })
                        .limit(1)
                        .build(),
                ),
            )
            .await
            .map_err(|_| ErrorInternalServerError("Failed to get last message"))?;

        let last_message = match last_message_cursor.try_next().await {
            Ok(Some(message)) => Some((message.content, message.created_at)),
            Ok(None) => None,
            Err(_) => None, // Instead of returning an error, just treat as no message
        };

        // Ensure chat ID exists
        let chat_id = match chat.id {
            Some(id) => id,
            None => return Err(ErrorInternalServerError("Chat ID is None")),
        };

        if last_message.is_none() {
            continue; // Skip if no last message
        }

        // Create chat summary
        chat_summaries.push(ChatSummary {
            id: chat_id.to_string(),
            participant_username: other_user.username,
            participant_user_id: other_participant_id.clone().to_string(),
            last_message: last_message.as_ref().map(|(content, _)| content.clone()),
            last_message_timestamp: last_message.map(|(_, timestamp)| timestamp),
        });
    }

    // Sort chat summaries by last message timestamp, most recent first
    chat_summaries.sort_by(|a, b| {
        b.last_message_timestamp
            .unwrap_or_else(chrono::Utc::now)
            .cmp(&a.last_message_timestamp.unwrap_or_else(chrono::Utc::now))
    });

    Ok(chat_summaries)
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

pub async fn get_chat_summary(state: &AppState, chat_id: ObjectId, user_id: ObjectId) -> Result<ChatSummary, Error> {
    let chat_collection = state.db.collection::<Chat>(Chat::collection_name());
    let users_collection = state.db.collection::<User>(User::collection_name());
    let messages_collection = state.db.collection::<Message>(Message::collection_name());

    let chat = chat_collection
        .find_one(doc! { "_id": &chat_id, "participant_ids": &user_id }, None)
        .await
        .map_err(|_| ErrorInternalServerError("Database error retrieving chat"))?
        .ok_or_else(|| ErrorForbidden("Chat not found"))?;
    
    // Find the other participant 
    let other_participant_id = chat
        .participant_ids
        .iter()
        .find(|&id| id != &user_id)
        .ok_or_else(|| ErrorInternalServerError("Failed to find other participant"))?;

    // Get other participant's username
    let other_user = users_collection
        .find_one(doc! { "_id": other_participant_id }, None)
        .await
        .map_err(|_| ErrorInternalServerError("Failed to get other participant info"))?
        .ok_or_else(|| ErrorInternalServerError("Other participant not found"))?;

    // Get last message
    let mut last_message_cursor = messages_collection
        .find(
            doc! { "chat_id": &chat_id },
            Some(
                mongodb::options::FindOptions::builder()
                    .sort(doc! { "created_at": -1 })
                    .limit(1)
                    .build(),
            ),
        )
        .await
        .map_err(|_| ErrorInternalServerError("Failed to get last message"))?;

    let last_message = match last_message_cursor.try_next().await {
        Ok(Some(message)) => Some((message.content, message.created_at)),
        Ok(None) => None,
        Err(_) => None,
    };

    Ok(ChatSummary {
        id: chat_id.to_string(),
        participant_username: other_user.username,
        participant_user_id: other_participant_id.to_string(),
        last_message: last_message.as_ref().map(|(content, _)| content.clone()),
        last_message_timestamp: last_message.map(|(_, timestamp)| timestamp),
    })
}

/// Create a new chat room.
pub async fn create_chat(
    state: &AppState,
    chat_id: Option<ObjectId>,
    participant_ids: HashSet<ObjectId>,
) -> Result<serde_json::Value, Error> {
    let chats = state.db.collection::<Chat>(Chat::collection_name());

    // Check if a chat between these participants already exists
    let participant_ids_vec: Vec<ObjectId> = participant_ids.iter().cloned().collect();
    let existing_chat = chats
        .find_one(
            doc! {
                "participant_ids": {
                    "$all": &participant_ids_vec,
                    "$size": participant_ids_vec.len() as i32
                }
            },
            None,
        )
        .await
        .map_err(|_| ErrorInternalServerError("Failed to check for existing chat"))?;

    // If chat exists, return it
    if let Some(chat) = existing_chat {
        let result = serde_json::json!({
            "id": chat.id.map_or_else(String::new, |id| id.to_string()),
            "participant_ids": chat.participant_ids,
            "created_at": chat.created_at
        });
        return Ok(result);
    }

    // Otherwise, create a new Chat document
    let new_chat = Chat::new(chat_id, participant_ids.into_iter().collect(), None);
    chats
        .insert_one(new_chat.clone(), None)
        .await
        .map_err(|_| ErrorInternalServerError("Failed to create chat room"))?;

    // Update the document in MongoDB to ensure id field is set correctly
    // This is necessary because MongoDB auto-generates _id, but our model expects id
    let new_result = serde_json::json!({
        "id": new_chat.id.map_or_else(String::new, |id| id.to_string()),
        "participant_ids": new_chat.participant_ids,
        "created_at": new_chat.created_at
    });
    Ok(new_result)
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
    created_at: Option<chrono::DateTime<chrono::Utc>>,
) -> Result<Message, Error> {
    let chats = state.db.collection::<Chat>(Chat::collection_name());
    // Verify the user is a participant in the chat.
    chats
        .find_one(doc! { "_id": &chat_id, "participant_ids": &user_id }, None)
        .await
        .map_err(|_| ErrorInternalServerError("Database error during participation check"))?
        .ok_or_else(|| ErrorForbidden("You are not a participant in this chat"))?;

    // Insert the new message
    let new_message = Message::new(chat_id.clone(), user_id, content, created_at);
    let messages = state.db.collection::<Message>(Message::collection_name());
    let message = messages
        .insert_one(new_message.clone(), None)
        .await
        .map_err(|_| ErrorInternalServerError("Failed to insert message"))?;

    // Return the newly created message with its auto-generated ID
    let new_message_id = message
        .inserted_id
        .as_object_id()
        .ok_or_else(|| ErrorInternalServerError("Failed to get inserted message ID"))?;

    // Update the message model with the new ID
    let mut message_model = new_message.clone();
    message_model.id = Some(new_message_id);

    Ok(message_model)
}

/// Retrieve all messages for a chat room if the user is a participant.
pub async fn get_chat_messages(
    state: &AppState,
    chat_id: ObjectId,
    user_id: ObjectId,
) -> Result<Vec<serde_json::Value>, Error> {
    let chats = state.db.collection::<Chat>(Chat::collection_name());
    // Verify that the user is a participant.
    chats
        .find_one(doc! { "_id": &chat_id, "participant_ids": &user_id }, None)
        .await
        .map_err(|_| ErrorInternalServerError("Database error during participation check"))?
        .ok_or_else(|| ErrorForbidden("You are not a participant in this chat"))?;

    let messages_coll = state.db.collection::<Message>(Message::collection_name());

    // Create find options to sort by created_at in ascending order (oldest to newest)
    let find_options = mongodb::options::FindOptions::builder()
        .sort(doc! { "created_at": 1 })
        .build();

    let cursor = messages_coll
        .find(doc! { "chat_id": &chat_id }, find_options)
        .await
        .map_err(|e| ErrorInternalServerError(format!("Failed to get messages: {}", e)))?;

    let messages: Vec<Message> = cursor
        .try_collect()
        .await
        .map_err(|e| ErrorInternalServerError(format!("Failed to collect messages: {}", e)))?;

    let results = messages
        .into_iter()
        .map(|message| {
            serde_json::json!({
                "id": message.id.map_or_else(String::new, |id| id.to_string()),
                "chat_id": message.chat_id.to_string(),
                "sender_id": message.sender_id.to_string(),
                "content": message.content,
                "created_at": message.created_at
            })
        })
        .collect();

    Ok(results)
}
