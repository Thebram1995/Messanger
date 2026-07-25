use reqwest::Client;

use crate::{config::env::AppConfig, security::jwt_service::JwtService};

#[derive(Clone)]
pub struct Services {
    pub jwt: JwtService,
}

#[derive(Clone)]
pub struct AppState {
    pub client: Client,
    pub config: AppConfig,
    pub services: Services,
}
