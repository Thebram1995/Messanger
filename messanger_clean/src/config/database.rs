use mongodb::{Client, Database};

use crate::config::env::AppConfig;

pub async fn connect_database(config: &AppConfig) -> Result<Database, String> {
    let client = Client::with_uri_str(&config.mongodb_uri)
        .await
        .map_err(|error| format!("Error conectando a MongoDB: {}", error))?;

    Ok(client.database(&config.mongodb_database))
}