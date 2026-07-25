use chrono::{Duration, Utc};
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode,
};

use crate::security::jwt_claims::JwtClaims;

#[derive(Clone)]
pub struct JwtService {
    secret: String,
    expiration_hours: i64,
}

impl JwtService {
    /// Crea el servicio JWT con el secreto de firma y la duración configurada.
    pub fn new(secret: String, expiration_hours: i64) -> Self {
        assert!(
            secret.as_bytes().len() >= 32,
            "JWT_SECRET debe contener al menos 32 bytes"
        );

        assert!(
            expiration_hours > 0,
            "JWT_EXPIRATION_HOURS debe ser mayor que cero"
        );

        Self {
            secret,
            expiration_hours,
        }
    }

    /// Genera un token firmado con la identidad y permisos del usuario.
    pub fn generate(
        &self,
        user_id: String,
        email: String,
        username: String,
        role: String,
        permissions: Vec<String>,
    ) -> Result<String, String> {
        let issued_at = Utc::now();

        let expiration = issued_at
            .checked_add_signed(Duration::hours(self.expiration_hours))
            .ok_or_else(|| "No se pudo calcular la expiración del token".to_string())?;

        let claims = JwtClaims {
            sub: user_id,
            email,
            username,
            role,
            permissions,
            iat: issued_at.timestamp() as usize,
            exp: expiration.timestamp() as usize,
        };

        encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|error| format!("Error generando JWT: {error}"))
    }

    /// Valida la firma, el algoritmo y la expiración del token.
    pub fn validate(&self, token: &str) -> Result<TokenData<JwtClaims>, String> {
        let mut validation = Validation::new(Algorithm::HS256);

        validation.validate_exp = true;

        decode::<JwtClaims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &validation,
        )
        .map_err(|error| format!("Token inválido o expirado: {error}"))
    }
}
