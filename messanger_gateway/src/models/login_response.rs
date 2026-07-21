use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Respuesta interna devuelta por el microservicio después
/// de validar el correo y la contraseña.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicroLoginResponse {
    pub user_id: String,
    pub email: String,
    pub username: String,
    pub role: String,
    pub permissions: Vec<String>,
}

/// Respuesta pública enviada por el gateway al frontend.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LoginResponse {
    pub token: String,
    pub user_id: String,
    pub email: String,
    pub username: String,
    pub role: String,
    pub permissions: Vec<String>,
}