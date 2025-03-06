use mongodb::{
    Client,
    Database,
    options::ClientOptions
};
use crate::config::app_config::AppConfig;
use std::error::Error;

pub async fn init_db() -> Result<(Client, Database), Box<dyn Error>> {
    let config = AppConfig::new()?; // AppConfig::new() must return Result<AppConfig, Box<dyn Error>>
    let client_options = ClientOptions::parse(&config.database_url)
        .await
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;
    let client = Client::with_options(client_options)
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;
    let db = client.database(&config.database_name);
    Ok((client, db))
}
