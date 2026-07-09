use axum::{
    routing::post,
    Router,
};

use crate::{
    config::app_state::AppState,
    proxy::service_registry::Microservice,
    proxy_route,
};

proxy_route!(
    POST,
    login,
    Microservice::Messenger,
    "/auth/login"
);

proxy_route!(
    POST,
    register,
    Microservice::Messenger,
    "/auth/register"
);

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/messenger/auth/login", post(login))
        .route("/messenger/auth/register", post(register))
}