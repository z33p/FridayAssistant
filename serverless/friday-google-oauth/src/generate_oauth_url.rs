use oauth2::CsrfToken;
use serde_json::json;
use tracing::debug;

use crate::{
    get_oauth_client,
    lambda_handler::lambda_response::LambdaResponse,
    oauth_provider::{OAuthProvider, OAuthProviderFactory},
};

pub async fn generate_oauth_url() -> Result<LambdaResponse, Box<dyn std::error::Error>> {
    generate_oauth_url_for_provider(OAuthProvider::Microsoft).await
}

pub async fn generate_oauth_url_for_provider(
    provider: OAuthProvider,
) -> Result<LambdaResponse, Box<dyn std::error::Error>> {
    let client = get_oauth_client(provider.clone())?;

    // Create the provider to get scopes and params
    let oauth_provider = OAuthProviderFactory::create_provider(
        &provider,
        String::new(), // We don't need credentials for just getting scopes
        String::new(),
        String::new(),
    );

    let mut auth_url_builder = client.authorize_url(CsrfToken::new_random);

    // Add provider-specific scopes
    for scope in oauth_provider.get_auth_scopes() {
        auth_url_builder = auth_url_builder.add_scope(scope);
    }

    // Add provider-specific parameters
    for (key, value) in oauth_provider.get_additional_auth_params() {
        auth_url_builder = auth_url_builder.add_extra_param(key, value);
    }

    let (auth_url, _) = auth_url_builder.url();

    debug!("Generated {} OAuth URL: {}", provider, auth_url.to_string());

    Ok(LambdaResponse {
        status_code: 200,
        data: json!({
            "url": auth_url.to_string(),
            "provider": provider.to_string()
        }),
        errors: None,
    })
}
