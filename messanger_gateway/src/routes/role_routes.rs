use axum::{
    Router,
    routing::{get, post},
};

use crate::{config::app_state::AppState, proxy::service_registry::Microservice, proxy_route};

proxy_route!(GET, get_roles, Microservice::Messenger, "/roles");

proxy_route!(POST, create_role, Microservice::Messenger, "/roles");

pub fn role_routes() -> Router<AppState> {
    Router::new().route("/messenger/roles", get(get_roles).post(create_role))
}
