use axum::{
    routing::get,
    Router,
};

use crate::{
    config::app_state::AppState,
    infrastructure::adapters::input::rest::controllers::health_controller,
};

pub fn health_routes() -> Router<AppState> {
    Router::new()
        .route("/health", get(health_controller::health))
}