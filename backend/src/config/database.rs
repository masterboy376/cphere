use mongodb::{
    Client,
    Database,
    error::Error as MongoError,
    options::ClientOptions
};
use thiserror::Error;

use crate::config::app_config::{AppConfig, ConfigError};

#[derive(Debug, Error)]
pub enum DbError {
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),
    #[error("MongoDB error: {0}")]
    Mongo(#[from] MongoError),
}

pub async fn init_db() -> Result<(Client, Database), DbError> {
    let config = AppConfig::from_env()?;
    let client_options = ClientOptions::parse(&config.database_url).await?;
    let client = Client::with_options(client_options)?;
    let db = client.database(&config.database_name);
    Ok((client, db))
}
