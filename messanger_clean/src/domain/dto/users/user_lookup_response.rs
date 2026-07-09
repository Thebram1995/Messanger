use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::entities::user::UserLookup;
use crate::domain::dto::roles::role_response::RoleResponse;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserLookupResponse {
    pub id: String,
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub role: RoleResponse,
    pub active: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl From<UserLookup> for UserLookupResponse {
    fn from(user: UserLookup) -> Self {
        Self {
            id: user.id.to_string(),
            username: user.username,
            display_name: user.display_name,
            email: user.email,
            role: RoleResponse::from(user.role),
            active: user.active,
            created_at: user.created_at.to_string(),
            updated_at: user.updated_at.to_string(),
        }
    }
}