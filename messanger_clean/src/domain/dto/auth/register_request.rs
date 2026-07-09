use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::register_role::RegisterRole;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RegisterRequest {
    #[schema(example = "TheOnix")]
    pub username: String,

    #[schema(example = "Nombre y Apellido")]
    pub display_name: String,

    #[schema(example = "Email")]
    pub email: String,

    #[schema(example = "Contraseña")]
    pub password: String,

    #[schema(example = "USER/CLAN_LEADER")]
    pub role: RegisterRole,
}
