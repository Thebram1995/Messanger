use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LoginRequest {
    #[schema(example = "Abraham")]
    pub username: String,

    #[schema(example = "123456")]
    pub password: String,
}