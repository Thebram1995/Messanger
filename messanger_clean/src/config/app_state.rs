use mongodb::Database;

use crate::config::env::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub database: Database,
    pub config: AppConfig,
}