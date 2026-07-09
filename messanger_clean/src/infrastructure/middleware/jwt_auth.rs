use axum::{
    extract::{FromRequestParts},
    http::{request::Parts, StatusCode},
};

use crate::{
    config::app_state::AppState, domain::dto::jwt::jwt_claims::JwtClaims, 
    infrastructure::security::jwt_service::JwtService,
};

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub claims: JwtClaims,
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|value| value.to_str().ok())
            .ok_or((
                StatusCode::UNAUTHORIZED,
                "Authorization header requerido".to_string(),
            ))?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or((
                StatusCode::UNAUTHORIZED,
                "Bearer token requerido".to_string(),
            ))?;

        let jwt_service = JwtService::new(state.config.jwt_secret.clone());

        let claims = jwt_service
            .validate(token)
            .map_err(|error| (StatusCode::UNAUTHORIZED, error))?;

        Ok(AuthUser { claims })
    }
}