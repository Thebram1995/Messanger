use std::sync::Arc;

use axum::{
    extract::State,
    Json,
};

use crate::{
    application::services::{
        login::login_service::LoginService,
        auth::register_service::RegisterService,
    },
    config::app_state::AppState,
    domain::dto::{
        login::{
            login_request::LoginRequest,
            login_response::LoginResponse,
        },
        auth::register_request::RegisterRequest,
    },
    infrastructure::adapters::output::mongo::{
        user_mongo_repository::MongoUserRepository,
        role_mongo_repository::MongoRoleRepository,
    },
    infrastructure::security::jwt_service::JwtService,
};

#[utoipa::path(
    post,
    path = "/auth/login",
    tag = "Auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse)
    )
)]
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, String> {

    let repository = MongoUserRepository::new(
        state.database.collection("users")
    );

    let jwt_service = JwtService::new(state.config.jwt_secret.clone());

    let service = LoginService::new(
        Arc::new(repository),
        jwt_service,
    );

    let response = service.execute(payload).await?;

    Ok(Json(response))
}

#[utoipa::path(
    post,
    path = "/auth/register",
    tag = "Auth",
    request_body = RegisterRequest,
    responses(
        (status = 200, description = "Register successful")
    )
)]
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<String>, String> {

    let user_repository = MongoUserRepository::new(
        state.database.collection("users")
    );

    let role_repository = MongoRoleRepository::new(
        state.database.collection("roles")
    );

    let service = RegisterService::new(
        Arc::new(user_repository),
        Arc::new(role_repository),
    );

    let user = service.execute(payload).await?;

    Ok(Json(user.id.to_string()))
}