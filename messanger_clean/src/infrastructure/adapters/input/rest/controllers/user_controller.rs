use axum::{Json, extract::{State,Path}};

use crate::{
    config::app_state::AppState,

    application::{
        ports::output::user_repository::UserRepository,
        services::users::create_user_service::CreateUserService,
    },

    domain::dto::users::{
        create_user_request::CreateUserRequest,
        user_lookup_response::UserLookupResponse,
    },
};

#[utoipa::path(
    post,
    path = "/users",
    tag = "Users",
    request_body = CreateUserRequest,
    responses(
        (status = 200, description = "User created successfully")
    )
)]

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<String>, String> {

    let service = CreateUserService::new(
        std::sync::Arc::new(
            crate::infrastructure::adapters::output::mongo::user_mongo_repository::MongoUserRepository::new(
                state.database.collection("users")
            )
        )
    );

    let user = service
        .execute(payload, None)
        .await?;

    Ok(Json(user.id.to_string()))
}

#[utoipa::path(
    get,
    path = "/users/{username}",
    tag = "Users",
    params(
        ("username" = String, Path, description = "Username")
    ),
    responses(
        (status = 200, description = "User found", body = UserLookupResponse),
        (status = 404, description = "User not found")
    )
)]
pub async fn get_user_by_username(
    State(state): State<AppState>,
    Path(username): Path<String>,
) -> Result<Json<UserLookupResponse>, String> {
    let repository = crate::infrastructure::adapters::output::mongo::user_mongo_repository::MongoUserRepository::new(
        state.database.collection("users")
    );

    let user = repository
        .find_by_username(&username)
        .await?
        .ok_or_else(|| "Usuario no encontrado".to_string())?;

    Ok(Json(UserLookupResponse::from(user)))
}