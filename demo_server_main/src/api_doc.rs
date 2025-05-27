use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};

use crate::api::{auth, file, stream, user};

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
    nest(
        (path = "/v1/auth", api = auth::JWTApi),
        (path = "/v1/user", api = user::UserApi),
        (path = "/v1/file", api = file::FileApi),
        (path = "/v1/stream", api = stream::StreamApi),
    ),
    tags(
        (name = "demo api", description = "Demo items management API")
    )
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "Authorization",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("Authorization"))),
            )
        }
    }
}
