use chrono::{Duration, Utc};
use jsonwebtoken::{
    encode,
    EncodingKey,
    Header,
};

use crate::security::jwt_claims::JwtClaims;

#[derive(Clone)]
pub struct JwtService {
    secret: String,
}

impl JwtService {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }

    /// Genera un token JWT con la identidad, rol y permisos del usuario.
    pub fn generate(
        &self,
        user_id: String,
        email: String,
        username: String,
        role: String,
        permissions: Vec<String>,
    ) -> Result<String, String> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .ok_or_else(|| {
                "No se pudo calcular la expiración del token".to_string()
            })?
            .timestamp();

        let claims = JwtClaims {
            sub: user_id,
            email,
            username,
            role,
            permissions,
            exp: expiration as usize,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|error| {
            format!("Error generando JWT: {error}")
        })
    }
}