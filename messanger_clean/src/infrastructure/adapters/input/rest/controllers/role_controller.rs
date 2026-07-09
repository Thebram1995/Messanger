use std::sync::Arc;

use axum::{
    extract::State,
    Json,
};

use crate::{
    application::{
        ports::output::role_repository::RoleRepository,
        services::roles::create_role_service::CreateRoleService,
    },
    config::app_state::AppState,
    domain::dto::roles::{
        create_role_request::CreateRoleRequest,
        role_response::RoleResponse,
    },
    infrastructure::{
        adapters::output::mongo::role_mongo_repository::MongoRoleRepository,
        middleware::jwt_auth::AuthUser,
    },
};

#[utoipa::path(
    post,
    path = "/roles",
    tag = "Roles",
    security(
        ("bearer_auth" = [])
    ),
    request_body = CreateRoleRequest,
    responses(
        (status = 200, description = "Role created successfully")
    )
)]
pub async fn create_role(
    auth: AuthUser,
    State(state): State<AppState>,
    Json(payload): Json<CreateRoleRequest>,
) -> Result<Json<String>, String> {

    if !auth.claims.permissions.contains(&"CREATE_ROLE".to_string()) {
        return Err("No autorizado".to_string());
    }

    let repository = MongoRoleRepository::new(
        state.database.collection("roles")
    );

    let service = CreateRoleService::new(Arc::new(repository));

    let role = service.execute(payload).await?;

    Ok(Json(role.id.to_string()))
}

#[utoipa::path(
    get,
    path = "/roles",
    tag = "Roles",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Roles listed successfully")
    )
)]
pub async fn get_roles(
    auth: AuthUser,
    State(state): State<AppState>,
) -> Result<Json<Vec<RoleResponse>>, String> {

    if !auth.claims.permissions.contains(&"READ_ROLE".to_string()) {
        return Err("No autorizado".to_string());
    }

    tracing::info!(
        "User authenticated: {}",
        auth.claims.username
    );

    let repository = MongoRoleRepository::new(
        state.database.collection("roles")
    );

    let roles = repository.find_all().await?;

    let response = roles
        .into_iter()
        .map(RoleResponse::from)
        .collect();

    Ok(Json(response))
}