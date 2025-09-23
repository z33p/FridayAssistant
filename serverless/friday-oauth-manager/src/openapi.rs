use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    api_response::ApiResponse,
    oauth_provider::OAuthProvider,
    oauth_tokens_mod::{
        oauth_tokens::OAuthTokens,
        oauth_tokens_controller::{
            get_oauth_tokens_request::GetOAuthTokensRequest,
            refresh_access_token_request::RefreshAccessTokenRequest,
        },
    },
};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Friday OAuth API",
        description = "OAuth 2.0 authentication service supporting Google and Microsoft providers",
        version = "1.0.0",
    ),
    paths(
        crate::oauth_tokens_mod::oauth_tokens_controller::generate_access_token,
        crate::oauth_tokens_mod::oauth_tokens_controller::refresh_access_token,
        crate::oauth_tokens_mod::oauth_tokens_controller::generate_oauth_url_endpoint,
        crate::oauth_tokens_mod::oauth_tokens_controller::generate_google_oauth_url,
        crate::oauth_tokens_mod::oauth_tokens_controller::generate_microsoft_oauth_url,
        crate::oauth_tokens_mod::oauth_tokens_controller::get_oauth_tokens,
        crate::oauth_tokens_mod::oauth_tokens_controller::health_check,
    ),
    components(schemas(
        ApiResponse,
        OAuthProvider,
        GetOAuthTokensRequest,
        RefreshAccessTokenRequest,
        OAuthTokens,
    )),
    tags(
        (name = "OAuth", description = "OAuth token management endpoints"),
        (name = "OAuth URLs", description = "OAuth authorization URL generation endpoints"),
        (name = "Health", description = "Service health check endpoints")
    )
)]
pub struct ApiDoc;

pub fn swagger_config() -> SwaggerUi {
    SwaggerUi::new("/api/friday-oauth-manager/swagger/{_:.*}").url(
        "/api/friday-oauth-manager/api-docs/openapi.json",
        ApiDoc::openapi(),
    )
}
