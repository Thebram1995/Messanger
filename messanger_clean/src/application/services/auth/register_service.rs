use std::sync::Arc;

use bcrypt::{hash, DEFAULT_COST};
use mongodb::bson::{
    oid::ObjectId,
    DateTime as BsonDateTime,
};

use crate::{
    application::ports::output::{
        role_repository::RoleRepository,
        user_repository::UserRepository,
    },
    domain::{
        dto::auth::{
            register_request::RegisterRequest,
            register_role::RegisterRole,
        },
        entities::user::User,
    },
};

pub struct RegisterService {
    user_repository: Arc<dyn UserRepository>,
    role_repository: Arc<dyn RoleRepository>,
}

impl RegisterService {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        role_repository: Arc<dyn RoleRepository>,
    ) -> Self {
        Self {
            user_repository,
            role_repository,
        }
    }

    pub async fn execute(
        &self,
        request: RegisterRequest,
    ) -> Result<User, String> {
        if self
            .user_repository
            .exists_by_username(&request.username)
            .await?
        {
            return Err("El nombre de usuario ya existe".to_string());
        }

        if self
            .user_repository
            .exists_by_email(&request.email)
            .await?
        {
            return Err("El correo ya está registrado".to_string());
        }

        let role_name = match request.role {
            RegisterRole::USER => "USER",
            RegisterRole::CLAN_LEADER => "CLAN_LEADER",
        };

        let role = self
            .role_repository
            .find_by_name(role_name)
            .await?
            .ok_or_else(|| format!("El rol '{}' no existe", role_name))?;

        let password_hash = hash(request.password, DEFAULT_COST)
            .map_err(|error| format!("Error encryptando password: {}", error))?;

        let now = BsonDateTime::now();

        let user = User {
            id: ObjectId::new(),
            username: request.username,
            display_name: request.display_name,
            email: request.email,
            password: password_hash,
            role_id: role.id,
            active: true,
            created_at: now,
            updated_at: now,
            created_by: None,
            updated_by: None,
        };

        self.user_repository.create(user).await
    }
}