use axum::{
    routing::post,
    Router,
};

use crate::{
    config::app_state::AppState,
    infrastructure::adapters::input::rest::controllers::auth_controller,
};

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/auth/login",post(auth_controller::login),)
        .route("/auth/register", post(auth_controller::register))
}