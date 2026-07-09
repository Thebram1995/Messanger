use axum::{routing::{post, get}, Router};

use crate::{
    config::app_state::AppState,
    infrastructure::adapters::input::rest::controllers::role_controller,
};

pub fn role_routes() -> Router<AppState> {
    Router::new()
        .route("/roles", post(role_controller::create_role))
        .route("/roles", get(role_controller::get_roles))
}