use utoipa::OpenApi;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::infrastructure::adapters::input::rest::controllers::health_controller::health,
        crate::infrastructure::adapters::input::rest::controllers::user_controller::create_user,
        crate::infrastructure::adapters::input::rest::controllers::role_controller::create_role,
        crate::infrastructure::adapters::input::rest::controllers::role_controller::get_roles,
        crate::infrastructure::adapters::input::rest::controllers::user_controller::get_user_by_username,
        crate::infrastructure::adapters::input::rest::controllers::auth_controller::login,
        crate::infrastructure::adapters::input::rest::controllers::auth_controller::register,
    ),
    components(),
    modifiers(&SecurityAddon),
    tags(
        (name = "Health", description = "Health endpoints"),
        (name = "Users", description = "User management endpoints"),
        (name = "Roles", description = "Role management endpoints"),
        (name = "Auth", description = "Authentication endpoints")
    )
)]
pub struct ApiDoc;

pub struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}