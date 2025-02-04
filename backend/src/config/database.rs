// src/config/database.rs
use mongodb::{Client, Database, options::ClientOptions};
use crate::config::app_config::{AppConfig, ConfigError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("Configuration error: {0}")]
    ConfigError(#[from] ConfigError),
    #[error("MongoDB error: {0}")]
    MongoError(#[from] mongodb::error::Error),
}

pub async fn init_db() -> Result<Database, DbError> {
    // Load application configuration
    let config = AppConfig::from_env()?;
    // Parse the MongoDB client options from the connection string
    let client_options = ClientOptions::parse(&config.database_url).await?;
    let client = Client::with_options(client_options)?;
    // Return the database using the name provided in configuration
    Ok(client.database(&config.database_name))
}
