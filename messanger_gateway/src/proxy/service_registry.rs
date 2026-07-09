use crate::config::app_state::AppState;

#[derive(Debug, Clone, Copy)]
pub enum Microservice {
    Messenger,
    Statistics,
    Notifications,
}

pub struct ServiceRegistry;

impl ServiceRegistry {
    pub fn resolve(
        state: &AppState,
        microservice: Microservice,
        path: &str,
    ) -> String {
        let base_url = match microservice {
            Microservice::Messenger => &state.config.messenger_ms_url,

            // Futuro
            Microservice::Statistics => &state.config.messenger_ms_url,
            Microservice::Notifications => &state.config.messenger_ms_url,
        };

        format!("{}{}", base_url, path)
    }
}