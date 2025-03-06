// src/services/notification.rs
use crate::{models::notification_model::Notification, states::app_state::AppState};
use actix_web::{error::ErrorInternalServerError, Error};
use futures::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId};

/// Retrieve notifications for a given user.
pub async fn get_user_notifications(
    state: &AppState,
    user_id: ObjectId,
) -> Result<Vec<Notification>, Error> {
    let notifications_collection = state
        .db
        .collection::<Notification>(Notification::collection_name());
    let cursor = notifications_collection
        .find(doc! { "recipient_id": &user_id }, None)
        .await
        .map_err(|_| ErrorInternalServerError("Failed to get notifications"))?;
    let results: Vec<Notification> = cursor
        .try_collect()
        .await
        .map_err(|_| ErrorInternalServerError("Failed to collect notifications"))?;
    Ok(results)
}

/// Retrieve a notification by its ID.
pub async fn get_notification_by_id(
    state: &AppState,
    notification_id: ObjectId,
) -> Result<Notification, Error> {
    let notifications_collection = state
        .db
        .collection::<Notification>(Notification::collection_name());
    let notification = notifications_collection
        .find_one(doc! { "_id": notification_id }, None)
        .await
        .map_err(|_| ErrorInternalServerError("Failed to find notification"))?
        .ok_or_else(|| ErrorInternalServerError("Notification not found"))?;
    Ok(notification)
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
pub async fn delete_notification(
    state: &AppState,
    notification_id: ObjectId,
) -> Result<(), Error> {
    let notifications_collection = state
        .db
        .collection::<Notification>(Notification::collection_name());
    notifications_collection
        .delete_one(doc! { "_id": notification_id }, None)
        .await
        .map_err(|_| ErrorInternalServerError("Failed to delete notification"))?;
    Ok(())
}
