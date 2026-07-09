mod application;
mod config;
mod domain;
mod infrastructure;

use axum::Router;
use std::net::SocketAddr;

use crate::config::app_state::AppState;
use config::database::connect_database;
use config::env::AppConfig;

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::infrastructure::docs::swagger::ApiDoc;

use infrastructure::adapters::input::rest::routes::{
    auth_routes::auth_routes,
    health_routes::health_routes,
    user_routes::user_routes,
    role_routes::role_routes,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("debug")
        .init();

    let config = AppConfig::from_env();

    let database = connect_database(&config)
        .await
        .expect("No se pudo conectar a MongoDB");

    let address: SocketAddr = config
        .address()
        .parse()
        .expect("HOST o PORT inválido");
    
    let port = config.port;

    let app_state = AppState {
        database, config,
    };

    tracing::info!("MongoDB connected");

    let app = Router::new()
        .merge(health_routes())
        .merge(user_routes())
        .merge(role_routes())
        .merge(auth_routes())
        .merge(
            SwaggerUi::new("/swagger-ui")
                .url("/api-docs/openapi.json", ApiDoc::openapi())
        )
        .with_state(app_state);

    tracing::info!("Server running on http://localhost:{}", port);

    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("Error starting server");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}