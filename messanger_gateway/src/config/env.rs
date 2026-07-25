use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub app_name: String,
    pub host: String,
    pub port: u16,

    pub messenger_ms_url: String,
    pub statistics_ms_url: String,
    pub notifications_ms_url: String,

    pub jwt_secret: String,
    pub jwt_expiration_hours: i64,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenvy::from_filename(".env").expect("No se pudo cargar el archivo .env");

        Self {
            app_name: env::var("APP_NAME").unwrap_or_else(|_| "Messenger Gateway".to_string()),

            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),

            port: env::var("PORT")
                .unwrap_or_else(|_| "8081".to_string())
                .parse()
                .expect("PORT inválido"),

            messenger_ms_url: env::var("MESSENGER_MS_URL").expect("MESSENGER_MS_URL es requerido"),

            statistics_ms_url: env::var("STATISTICS_MS_URL")
                .expect("STATISTICS_MS_URL es requerido"),

            notifications_ms_url: env::var("NOTIFICATIONS_MS_URL")
                .expect("NOTIFICATIONS_MS_URL es requerido"),

            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET es requerido en .env"),

            jwt_expiration_hours: env::var("JWT_EXPIRATION_HOURS")
                .unwrap_or_else(|_| "24".to_string())
                .parse()
                .expect("JWT_EXPIRATION_HOURS inválido"),
        }
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
