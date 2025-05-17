// src/config/app_config.rs
use crate::constants;
use dotenv::{dotenv, from_filename};
use serde::Deserialize;
use std::env;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub database_url: String,
    pub database_name: String,
    pub email_address: String,
    pub email_password: String,
    pub friend_request_notification: &'static str,
    pub video_call_notification: &'static str,
    pub reset_token_length: usize,
    pub reset_token_expiration_minutes: i64,
}

impl AppConfig {
    /// Loads configuration from environment variables.
    pub fn new() -> Result<Self, Box<dyn Error>> {
        dotenv().ok();
        let env = env::var("APP_ENV").unwrap_or_else(|_| "development".into());
        if env == "production" {
            from_filename(".env.prod").ok();
        }

        let database_url = env::var("DATABASE_URL")
            .map_err(|e| Box::<dyn Error>::from(format!("Missing DATABASE_URL: {}", e)))?;
        let database_name = env::var("DATABASE_NAME")
            .map_err(|e| Box::<dyn Error>::from(format!("Missing DATABASE_NAME: {}", e)))?;
        let email_address = env::var("EMAIL_ADDRESS")
            .map_err(|e| Box::<dyn Error>::from(format!("Missing EMAIL_ADDRESS: {}", e)))?;
        let email_password = env::var("EMAIL_PASSWORD")
            .map_err(|e| Box::<dyn Error>::from(format!("Missing EMAIL_PASSWORD: {}", e)))?;

        let friend_request_notification = constants::FRIEND_REQUEST_NOTIFICATION;
        let video_call_notification = constants::VIDEO_CALL_NOTIFICATION;
        let reset_token_length = constants::RESET_TOKEN_LENGTH;
        let reset_token_expiration_minutes = constants::TOKEN_EXPIRATION_MINUTES;

        Ok(AppConfig {
            database_url,
            database_name,
            email_address,
            email_password,
            friend_request_notification,
            video_call_notification,
            reset_token_length,
            reset_token_expiration_minutes,
        })
    }
}
