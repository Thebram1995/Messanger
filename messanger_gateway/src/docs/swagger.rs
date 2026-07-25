use utoipa::{
    Modify, OpenApi,
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
};

use crate::models::{
    create_role_request::CreateRoleRequest, login_request::LoginRequest,
    login_response::LoginResponse,
};

pub struct SecurityAddon;

impl Modify for SecurityAddon {
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

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),

    paths(
        crate::routes::auth_routes::login,
    ),

    components(
        schemas(
            LoginRequest,
            LoginResponse,
            CreateRoleRequest,
        )
    ),

    tags(
        (name = "Health"),
        (name = "Authentication"),
        (name = "Roles")
    )
)]
pub struct ApiDoc;
