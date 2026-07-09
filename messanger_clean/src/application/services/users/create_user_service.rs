use std::sync::Arc;

use bson::DateTime as BsonDateTime;
use mongodb::bson::oid::ObjectId;

use crate::application::ports::output::user_repository::UserRepository;
use crate::domain::dto::users::create_user_request::CreateUserRequest;
use crate::domain::entities::user::User;

pub struct CreateUserService {
    user_repository: Arc<dyn UserRepository>,
}

impl CreateUserService {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn execute(
        &self,
        request: CreateUserRequest,
        created_by: Option<ObjectId>,
    ) -> Result<User, String> {
        let exists = self
            .user_repository
            .exists_by_username(&request.username)
            .await?;

        if exists {
            return Err("El username ya existe".to_string());
        }

        let password_hash = bcrypt::hash(&request.password, bcrypt::DEFAULT_COST)
            .map_err(|error| format!("Error encryptando password: {}", error))?;

        let now = BsonDateTime::now();

        let role_id = ObjectId::parse_str(&request.role_id)
            .map_err(|_| "role_id inválido".to_string())?;

        let user = User {
            id: ObjectId::new(),
            username: request.username,
            display_name: request.display_name,
            email: request.email,
            password: password_hash,
            role_id,
            active: true,
            created_at: now,
            updated_at: now,
            created_by,
            updated_by: created_by,
        };

        self.user_repository.create(user).await
    }
}