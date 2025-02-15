// src/services/notification.rs
use crate::models::notification::Notification;
use crate::config::database::{DbError, init_db};
use mongodb::bson::oid::ObjectId;
use mongodb::error::Error as MongoError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NotificationError {
    #[error("Database error: {0}")]
    Db(#[from] DbError),
    #[error("MongoDB error: {0}")]
    Mongo(#[from] MongoError),
}

pub async fn send_notification(user_id: ObjectId, message: &str) -> Result<Notification, NotificationError> {
    let notification = Notification::new(user_id, message);
    
    let (client, db) = init_db().await?;
    let collection = db.collection::<Notification>("notifications");
    collection.insert_one(notification.clone(), None).await?;

    Ok(notification)
}
