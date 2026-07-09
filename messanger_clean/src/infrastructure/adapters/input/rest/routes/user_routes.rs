use axum::{
    routing::{
        post,get
    },
    Router,
};

use crate::{
    config::app_state::AppState,
    infrastructure::adapters::input::rest::controllers::user_controller,
};

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/users", post(user_controller::create_user))
        .route("/users/{username}", get(user_controller::get_user_by_username))
}