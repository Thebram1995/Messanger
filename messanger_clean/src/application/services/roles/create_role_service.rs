use std::sync::Arc;

use bson::DateTime as BsonDateTime;
use mongodb::bson::oid::ObjectId;

use crate::{
    application::ports::output::role_repository::RoleRepository,
    domain::{
        dto::roles::create_role_request::CreateRoleRequest,
        entities::role::Role,
    },
};

pub struct CreateRoleService {
    role_repository: Arc<dyn RoleRepository>,
}

impl CreateRoleService {
    pub fn new(role_repository: Arc<dyn RoleRepository>) -> Self {
        Self { role_repository }
    }

    pub async fn execute(&self, request: CreateRoleRequest) -> Result<Role, String> {
        let exists = self
            .role_repository
            .exists_by_name(&request.name)
            .await?;

        if exists {
            return Err("El rol ya existe".to_string());
        }

        let now = BsonDateTime::now();

        let role = Role {
            id: ObjectId::new(),
            name: request.name,
            permissions: request.permissions,
            active: true,
            created_at: now,
            updated_at: now,
        };

        self.role_repository.create(role).await
    }
}