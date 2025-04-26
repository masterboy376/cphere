// src/services/notification.rs
use crate::{models::notification_model::Notification, states::app_state::AppState};
use actix_web::{error::ErrorInternalServerError, Error};
use futures::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct NotificationSummary {
    pub id: String,
    pub notification_type: String,
    pub sender_user_id: String,
    pub sender_username: String,
    pub message: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Retrieve notifications for a given user.
pub async fn get_user_notifications(
    state: &AppState,
    user_id: ObjectId,
) -> Result<Vec<NotificationSummary>, Error> {
    let notifications_collection = state
        .db
        .collection::<Notification>(Notification::collection_name());
    let cursor = notifications_collection
        .find(doc! { "recipient_id": &user_id, "is_handled": false }, None)
        .await
        .map_err(|_| ErrorInternalServerError("Failed to get notifications"))?;
    let notifications: Vec<Notification> = cursor
        .try_collect()
        .await
        .map_err(|_| ErrorInternalServerError("Failed to collect notifications"))?;
    
    // Convert Notification objects to NotificationSummary objects
    // Collect notifications first to prepare for user fetch
    let mut summaries = Vec::with_capacity(notifications.len());
    
    // Gather unique sender IDs to batch fetch user data
    let sender_ids = notifications.iter()
        .map(|notification| notification.sender_id)
        .collect::<std::collections::HashSet<ObjectId>>();
    
    // Fetch user data for all sender_ids in one query
    let users_collection = state.db.collection::<crate::models::user_model::User>(
        crate::models::user_model::User::collection_name());
    
    let user_cursor = users_collection
        .find(doc! { "_id": { "$in": sender_ids.into_iter().collect::<Vec<_>>() } }, None)
        .await
        .map_err(|_| ErrorInternalServerError("Failed to fetch users"))?;
    
    let users: Vec<crate::models::user_model::User> = user_cursor
        .try_collect()
        .await
        .map_err(|_| ErrorInternalServerError("Failed to collect users"))?;
    
    // Create a map of user_id -> username for quick lookup
    let username_map: std::collections::HashMap<ObjectId, String> = users
        .into_iter()
        .map(|user| (user.id.unwrap_or_default(), user.username))
        .collect();
    
    // Create notification summaries with usernames
    for notification in notifications {
        let sender_id = notification.sender_id;
        let sender_username = username_map
            .get(&sender_id)
            .cloned()
            .unwrap_or_else(|| "Unknown User".to_string());
        
        summaries.push(NotificationSummary {
            id: notification.id.unwrap_or_default().to_hex(),
            notification_type: notification.notification_type,
            sender_user_id: sender_id.to_hex(),
            sender_username,
            message: notification.message,
            timestamp: notification.created_at,
        });
    }
    
    Ok(summaries)
}

/// Create a new notification.
pub async fn create_notification(
    state: &AppState,
    new_notification: Notification,
) -> Result<Notification, Error> {
    let notifications_collection = state
        .db
        .collection::<Notification>(Notification::collection_name());
    notifications_collection
        .insert_one(&new_notification, None)
        .await
        .map_err(|_| ErrorInternalServerError("Failed to create notification"))?;
    Ok(new_notification)
}

/// Delete a notification by its ID.
pub async fn delete_notification(state: &AppState, notification_id: ObjectId) -> Result<(), Error> {
    let notifications_collection = state
        .db
        .collection::<Notification>(Notification::collection_name());
    notifications_collection
        .delete_one(doc! { "_id": notification_id }, None)
        .await
        .map_err(|_| ErrorInternalServerError("Failed to delete notification"))?;
    Ok(())
}
