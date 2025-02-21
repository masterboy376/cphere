// src/config/app_config.rs
use serde::Deserialize;
use dotenv::dotenv;
use std::env;
use thiserror::Error;
use crate::constants;


#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub database_url: String,
    pub database_name: String,
    pub friend_request_notification: &'static str,
    pub video_call_notification: &'static str,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Missing environment variable: {0}")]
    MissingVariable(String),
    #[error("Failed to parse config: {0}")]
    ParseConfig(String),
}

impl AppConfig {
    /// Loads configuration from environment variables.
    pub fn new() -> Result<Self, ConfigError> {
        dotenv().ok(); // Load from .env if present
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| ConfigError::MissingVariable("DATABASE_URL".into()))?;
        let database_name = env::var("DATABASE_NAME")
            .map_err(|_| ConfigError::MissingVariable("DATABASE_NAME".into()))?;
        let friend_request_notification = constants::FRIEND_REQUEST_NOTIFICATION;
        let video_call_notification = constants::VIDEO_CALL_NOTIFICATION;
        Ok(AppConfig {
            database_url,
            database_name,
            friend_request_notification,
            video_call_notification
        })
    }
}
