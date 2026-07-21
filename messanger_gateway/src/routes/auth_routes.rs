use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Json,
    Router,
};

use crate::{
    config::app_state::AppState,
    models::{
        login_request::LoginRequest,
        login_response::{
            LoginResponse,
            MicroLoginResponse,
        },
    },
    proxy::service_registry::{
        Microservice,
        ServiceRegistry,
    },
    proxy_route,
};

#[utoipa::path(
    post,
    path = "/messenger/auth/login",
    tag = "Authentication",
    request_body = LoginRequest,
    responses(
        (
            status = 200,
            description = "Login successful",
            body = LoginResponse
        ),
        (
            status = 401,
            description = "Invalid email or password"
        )
    )
)]
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Response {
    let url = ServiceRegistry::resolve(
        &state,
        Microservice::Messenger,
        "/auth/login",
    );

    let response = match state
        .client
        .post(url)
        .json(&payload)
        .send()
        .await
    {
        Ok(response) => response,

        Err(error) => {
            return (
                StatusCode::BAD_GATEWAY,
                format!(
                    "Error conectando con el microservicio: {error}"
                ),
            )
                .into_response();
        }
    };

    let status = StatusCode::from_u16(
        response.status().as_u16(),
    )
    .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

    if !status.is_success() {
        let message = response
            .text()
            .await
            .unwrap_or_else(|_| {
                "Error procesando inicio de sesión".to_string()
            });

        return (status, message).into_response();
    }

    let micro_response = match response
        .json::<MicroLoginResponse>()
        .await
    {
        Ok(response) => response,

        Err(error) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!(
                    "Respuesta inválida del microservicio: {error}"
                ),
            )
                .into_response();
        }
    };

    let token = match state.services.jwt.generate(
        micro_response.user_id.clone(),
        micro_response.email.clone(),
        micro_response.username.clone(),
        micro_response.role.clone(),
        micro_response.permissions.clone(),
    ) {
        Ok(token) => token,

        Err(error) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                error,
            )
                .into_response();
        }
    };

    Json(LoginResponse {
        token,
        user_id: micro_response.user_id,
        email: micro_response.email,
        username: micro_response.username,
        role: micro_response.role,
        permissions: micro_response.permissions,
    })
    .into_response()
}

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