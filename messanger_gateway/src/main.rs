mod config;
mod docs;
mod macros;
mod middleware;
mod models;
mod proxy;
mod routes;
mod security;

use std::net::SocketAddr;

use axum::{
    Router,
    middleware::{from_fn, from_fn_with_state},
};

use crate::middleware::jwt_auth::jwt_auth_middleware;

use crate::{config::app_state::Services, middleware::request_id::request_id_middleware};

use crate::security::jwt_service::JwtService;

use reqwest::Client;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    config::{app_state::AppState, env::AppConfig},
    docs::swagger::ApiDoc,
};

use crate::routes::{
    auth_routes::auth_routes, health_routes::health_routes, role_routes::role_routes,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_env_filter("debug").init();

    let config = AppConfig::from_env();

    let address: SocketAddr = config.address().parse().expect("HOST o PORT inválido");

    let port = config.port;

    let services = Services {
        jwt: JwtService::new(config.jwt_secret.clone(), config.jwt_expiration_hours),
    };

    let app_state = AppState {
        client: Client::new(),
        config,
        services,
    };

    let protected_roles =
        role_routes().layer(from_fn_with_state(app_state.clone(), jwt_auth_middleware));

    let app = Router::new()
        .merge(health_routes())
        .merge(auth_routes())
        .merge(protected_roles)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(from_fn(request_id_middleware))
        .with_state(app_state);

    tracing::info!("Gateway running on http://localhost:{}", port);

    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("Error starting Gateway");

    axum::serve(listener, app).await.expect("Gateway error");
}
