use std::sync::Arc;

use bcrypt::verify;

use crate::{
    application::ports::output::user_repository::UserRepository, domain::dto::login::{
        login_request::LoginRequest,
        login_response::LoginResponse,
    }, 
    infrastructure::security::jwt_service::JwtService,
};

pub struct LoginService {
    user_repository: Arc<dyn UserRepository>,
    jwt_service: JwtService,
}

impl LoginService {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        jwt_service: JwtService,
    ) -> Self {
        Self {
            user_repository,
            jwt_service,
        }
    }

    pub async fn execute(
        &self,
        request: LoginRequest,
    ) -> Result<LoginResponse, String> {

        let user = self
            .user_repository
            .find_by_username(&request.username)
            .await?
            .ok_or_else(|| "Usuario o contraseña incorrectos".to_string())?;

        if !user.active {
            return Err("Usuario inactivo".to_string());
        }

        let password_valid = verify(
            &request.password,
            &user.password,
        )
        .map_err(|error| format!("Error validando password: {}", error))?;

        if !password_valid {
            return Err("Usuario o contraseña incorrectos".to_string());
        }

        let token = self.jwt_service.generate(
            user.id.to_string(),
            user.username.clone(),
            user.role.name.clone(),
            user.role.permissions.clone(),
        )?;

        Ok(LoginResponse {
            token,
            username: user.username,
            role: user.role.name,
            permissions: user.role.permissions,
        })
    }
}