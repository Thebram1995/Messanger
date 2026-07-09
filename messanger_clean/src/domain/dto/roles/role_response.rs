use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::entities::role::Role;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RoleResponse {
    pub id: String,
    pub name: String,
    pub permissions: Vec<String>,
    pub active: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Role> for RoleResponse {
    fn from(role: Role) -> Self {
        Self {
            id: role.id.to_string(),
            name: role.name,
            permissions: role.permissions,
            active: role.active,
            created_at: role.created_at.to_string(),
            updated_at: role.updated_at.to_string(),
        }
    }
}