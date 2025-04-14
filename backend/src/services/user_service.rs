use crate::models::user_model::User;
use crate::states::app_state::AppState;
use actix_web::{
    error::{
        ErrorInternalServerError,
        ErrorNotFound,
        ErrorBadRequest,
    },
    Error
};
use futures::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct BatchCheckOnlineRequest {
    pub user_ids: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct BatchCheckOnlineResponse {
    pub online_status: Vec<(String, bool)>,
}

/// Extract the user ID from the session.
pub fn extract_user_id_from_session(session: &actix_session::Session) -> Result<ObjectId, Error> {
    let user_id_str = match session.get::<String>("user_id") {
        Ok(Some(uid)) => uid,
        Ok(None) => {
            return Err(actix_web::error::ErrorUnauthorized("Not authenticated"));
        }
        Err(e) => {
            eprintln!("Session error: {}", e);
            return Err(actix_web::error::ErrorInternalServerError("Session error"));
        }
    };
    ObjectId::parse_str(&user_id_str).map_err(|_| ErrorBadRequest("Invalid user ID in session"))
}

/// Search users by username or email (matching a search slice)
pub async fn search_users(state: &AppState, query: &str) -> Result<Vec<serde_json::Value>, Error> {
    let users_collection = state.db.collection::<User>(User::collection_name());

    // Match against both username and email using regex (case-insensitive)
    let filter = doc! {
        "$or": [
            { "username": { "$regex": query, "$options": "i" } },
            { "email": { "$regex": query, "$options": "i" } },
        ]
    };

    let cursor = users_collection
        .find(filter, None)
        .await
        .map_err(|_| ErrorInternalServerError("Database error: Failed to search users"))?;

    let users: Vec<User> = cursor
        .try_collect()
        .await
        .map_err(|e| {
            eprintln!("Database error when collecting users: {}", e);
            ErrorInternalServerError("Database error: Failed to collect users")
        })?;

    // Convert to simplified format with just id and username
    let results = users.into_iter()
        .map(|user| {
            serde_json::json!({
                "id": user.id.map_or_else(String::new, |id| id.to_string()),
                "username": user.username
            })
        })
        .collect();

    Ok(results)
}

/// Get a user by their ObjectId.
pub async fn get_user_by_id(state: &AppState, user_id: ObjectId) -> Result<User, Error> {
    let users_collection = state.db.collection::<User>(User::collection_name());
    let filter = doc! { "_id": user_id };
    let user = users_collection
        .find_one(filter, None)
        .await
        .map_err(|_| ErrorInternalServerError("Database error: Failed to get user by id"))?;
    user.ok_or_else(|| ErrorNotFound("User not found"))
}

/// Get user data excluding sensitive fields like password_hash using the user id in the session.
pub async fn get_user_data(
    state: &AppState,
    session: &actix_session::Session
) -> Result<serde_json::Value, Error> {
    let user_id = extract_user_id_from_session(session)?;
    let user = get_user_by_id(state, user_id).await?;
    let mut user_json = serde_json::to_value(user)
        .map_err(|_| ErrorInternalServerError("Serialization error"))?;
    if let Some(obj) = user_json.as_object_mut() {
        obj.remove("password_hash");
    }
    Ok(user_json)
}

/// Check if a user is online, based on ws_sessions stored in the AppState.
pub async fn is_user_online(state: &AppState, user_id: ObjectId) -> bool {
    let ws_sessions = state.ws_sessions.read().await;
    ws_sessions.contains_key(&user_id)
}
