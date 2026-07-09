use axum::{
    routing::get,
    Json,
    Router,
};
use serde_json::json;

use crate::config::app_state::AppState;

async fn health() -> Json<serde_json::Value> {
    Json(json!({
        "status": "UP",
        "service": "messenger-gateway"
    }))
}

pub fn health_routes() -> Router<AppState> {
    Router::new().route("/health", get(health))
}