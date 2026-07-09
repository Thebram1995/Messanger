use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub app_name: String,

    pub host: String,
    pub port: u16,

    pub mongodb_uri: String,
    pub mongodb_database: String,
    pub mongodb_ca_file: Option<String>,

    pub mongodb_users_collection: String,
    pub mongodb_roles_collection: String,
    pub mongodb_authorization_collection: String,
    pub mongodb_messages_collection: String,
    pub mongodb_chat_rooms_collection: String,

    pub jwt_secret: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        Self {
            app_name: env::var("APP_NAME")
                .unwrap_or_else(|_| "Messanger".to_string()),

            host: env::var("HOST")
                .unwrap_or_else(|_| "0.0.0.0".to_string()),

            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .expect("PORT inválido"),

            mongodb_uri: env::var("MONGO_URI")
                .expect("MONGO_URI es requerido en .env"),

            mongodb_database: env::var("MONGO_DB")
                .unwrap_or_else(|_| "BDMESSANGER".to_string()),

            mongodb_ca_file: env::var("MONGO_CA_FILE").ok(),

            mongodb_users_collection: env::var("MONGO_USERS_COLLECTION")
                .unwrap_or_else(|_| "users".to_string()),

            mongodb_roles_collection: env::var("MONGO_ROLES_COLLECTION")
                .unwrap_or_else(|_| "roles".to_string()),

            mongodb_authorization_collection: env::var("MONGO_AUTHORIZATION_COLLECTION")
                .unwrap_or_else(|_| "authorization".to_string()),

            mongodb_messages_collection: env::var("MONGO_MESSAGES_COLLECTION")
                .unwrap_or_else(|_| "messages".to_string()),

            mongodb_chat_rooms_collection: env::var("MONGO_CLANS_COLLECTION")
                .unwrap_or_else(|_| "clan_rooms".to_string()),

            jwt_secret: env::var("JWT_SECRET")
                .expect("JWT_SECRET es requerido en .env"),
        }
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}