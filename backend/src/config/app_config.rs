// src/config/app_config.rs
use serde::Deserialize;
use dotenv::dotenv;
use std::env;
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub database_url: String,
    pub database_name: String,
    // Add more configuration options here (e.g., server port, email settings, etc.)
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Missing environment variable: {0}")]
    MissingVar(String),
    #[error("Failed to parse config: {0}")]
    ParseError(String),
}

impl AppConfig {
    /// Loads configuration from environment variables.
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv().ok(); // Load from .env if present
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| ConfigError::MissingVar("DATABASE_URL".into()))?;
        let database_name = env::var("DATABASE_NAME")
            .map_err(|_| ConfigError::MissingVar("DATABASE_NAME".into()))?;
        Ok(AppConfig {
            database_url,
            database_name,
        })
    }
}
