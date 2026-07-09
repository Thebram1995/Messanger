use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode,
    encode,
    DecodingKey,
    EncodingKey,
    Header,
    Validation,
};

use crate::domain::dto::jwt::jwt_claims::JwtClaims;

pub struct JwtService {
    secret: String,
}

impl JwtService {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }

    pub fn validate(&self, token: &str) -> Result<JwtClaims, String> {
        decode::<JwtClaims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|error| format!("Token inválido: {}", error))
    }

    pub fn generate(
        &self,
        user_id: String,
        username: String,
        role: String,
        permissions: Vec<String>,
    ) -> Result<String, String> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .ok_or("Error calculando expiración")?
            .timestamp();

        let claims = JwtClaims {
            sub: user_id,
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
        .map_err(|error| error.to_string())
    }

}