use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    business_response::Response, secrets_controller::DeleteSecretRequest,
    secrets_mod::secret::Secret,
};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Friday Secret Manager API",
        description = "Secret management service for storing and retrieving application secrets",
        version = "1.0.0",
    ),
    paths(
        crate::secrets_controller::get_secret_value,
        crate::secrets_controller::get_all_secrets,
        crate::secrets_controller::insert_secret,
        crate::secrets_controller::update_secret,
        crate::secrets_controller::delete_secret,
        crate::secrets_controller::refresh_secrets,
    ),
    components(schemas(
        Response<Secret>,
        Response<Vec<Secret>>,
        Response<String>,
        Secret,
        DeleteSecretRequest,
    )),
    tags(
        (name = "Secrets", description = "Secret management endpoints"),
        (name = "Operations", description = "Administrative operations")
    )
)]
pub struct ApiDoc;

pub fn swagger_config() -> SwaggerUi {
    SwaggerUi::new("/api/friday-secret-manager/swagger/{_:.*}")
        .url("/api/friday-secret-manager/api-docs/openapi.json", ApiDoc::openapi())
}
