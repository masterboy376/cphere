// src/services/notification.rs
use crate::{
    models::notification::Notification,
    config::app_config::{AppConfig, ConfigError}
};
use crate::config::database::{DbError, init_db};
use mongodb::bson::oid::ObjectId;
use mongodb::error::Error as MongoError;
use thiserror::Error;


#[derive(Debug, Error)]
pub enum NotificationError {
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),
    #[error("Database error: {0}")]
    Db(#[from] DbError),
    #[error("MongoDB error: {0}")]
    Mongo(#[from] MongoError),
}

pub async fn send_notification(to_user: ObjectId, from_user:ObjectId, message: &str) -> Result<Notification, NotificationError> {
    let config = AppConfig::new()?;
    let notification = Notification::new(config.video_call_notification, to_user, from_user, message);
    
    let (client, db) = init_db().await?;
    let collection = db.collection::<Notification>("notifications");
    collection.insert_one(notification.clone(), None).await?;

    Ok(notification)
}
