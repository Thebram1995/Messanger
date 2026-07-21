use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Información almacenada dentro del token JWT.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct JwtClaims {
    pub sub: String,
    pub email: String,
    pub username: String,
    pub role: String,
    pub permissions: Vec<String>,
    pub exp: usize,
}