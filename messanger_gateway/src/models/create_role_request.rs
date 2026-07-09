use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateRoleRequest {
    #[schema(example = "ADMIN")]
    pub name: String,

    #[schema(example = json!(["CREATE_USER", "READ_USER"]))]
    pub permissions: Vec<String>,
}