use axum::{
    body::Body,
    extract::{FromRequestParts, State},
    http::{Request, StatusCode, header::AUTHORIZATION, request::Parts},
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::{config::app_state::AppState, security::jwt_claims::JwtClaims};

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub claims: JwtClaims,
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let token = extract_bearer_token(parts)?;

        let token_data = state
            .services
            .jwt
            .validate(token)
            .map_err(unauthorized_response)?;

        Ok(Self {
            claims: token_data.claims,
        })
    }
}

pub async fn jwt_auth_middleware(
    State(state): State<AppState>,
    mut request: Request<Body>,
    next: Next,
) -> Response {
    let token = match extract_bearer_token_from_request(&request) {
        Ok(token) => token,
        Err(response) => return response,
    };

    let token_data = match state.services.jwt.validate(token) {
        Ok(token_data) => token_data,
        Err(error) => return unauthorized_response(error),
    };

    request
        .extensions_mut()
        .insert::<JwtClaims>(token_data.claims);

    next.run(request).await
}

/// Extrae y valida el formato del token Bearer desde las partes
/// utilizadas por un extractor de Axum.
fn extract_bearer_token(parts: &Parts) -> Result<&str, Response> {
    let authorization = parts
        .headers
        .get(AUTHORIZATION)
        .ok_or_else(|| unauthorized_response("El encabezado Authorization es requerido"))?;

    let authorization = authorization
        .to_str()
        .map_err(|_| unauthorized_response("El encabezado Authorization no es válido"))?;

    parse_bearer_token(authorization)
}

/// Extrae y valida el formato del token Bearer desde una petición.
fn extract_bearer_token_from_request(request: &Request<Body>) -> Result<&str, Response> {
    let authorization = request
        .headers()
        .get(AUTHORIZATION)
        .ok_or_else(|| unauthorized_response("El encabezado Authorization es requerido"))?;

    let authorization = authorization
        .to_str()
        .map_err(|_| unauthorized_response("El encabezado Authorization no es válido"))?;

    parse_bearer_token(authorization)
}

/// Valida que el encabezado tenga el formato Bearer esperado.
fn parse_bearer_token(authorization: &str) -> Result<&str, Response> {
    let token = authorization
        .strip_prefix("Bearer ")
        .ok_or_else(|| unauthorized_response("El token debe usar el formato Bearer <token>"))?
        .trim();

    if token.is_empty() {
        return Err(unauthorized_response(
            "El token Bearer no puede estar vacío",
        ));
    }

    Ok(token)
}

/// Crea una respuesta HTTP 401 uniforme para errores de autenticación.
fn unauthorized_response(message: impl Into<String>) -> Response {
    (StatusCode::UNAUTHORIZED, message.into()).into_response()
}
