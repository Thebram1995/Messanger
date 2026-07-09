use reqwest::Client;

use crate::config::env::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub client: Client,
    pub config: AppConfig,
}