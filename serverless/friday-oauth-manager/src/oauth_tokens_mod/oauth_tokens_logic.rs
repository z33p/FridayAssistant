use std::time::Duration;

use chrono::Utc;
use oauth2::{AuthorizationCode, CsrfToken, RefreshToken, TokenResponse};

use serde_json::json;
use tracing::{debug, error, info, warn};

use crate::{
    api_response::ApiResponse,
    get_oauth_client,
    oauth_provider::{OAuthProvider, OAuthProviderFactory}, oauth_tokens_mod::{oauth_tokens::OAuthTokens, oauth_tokens_controller::{get_oauth_tokens_request::GetOAuthTokensRequest, refresh_access_token_request::RefreshAccessTokenRequest}, oauth_tokens_data},
};

/// Business logic for obtaining OAuth tokens from authorization code
pub async fn get_oauth_tokens(
    request: GetOAuthTokensRequest,
) -> Result<ApiResponse, Box<dyn std::error::Error>> {
    let client = get_oauth_client(request.provider.clone())?;

    let code = AuthorizationCode::new(extract_code_from_url(&request.url)?);

    // Create the provider to get additional params
    let oauth_provider = OAuthProviderFactory::create_provider(
        &request.provider,
        String::new(),
        String::new(),
        String::new(),
    );

    let mut token_request = client.exchange_code(code);

    // Add provider-specific parameters
    for (key, value) in oauth_provider.get_additional_token_params() {
        token_request = token_request.add_extra_param(key, value);
    }

    let tokens_response = token_request
        .request_async(oauth2::reqwest::async_http_client)
        .await?;

    let oauth_tokens = extract_oauth_tokens(tokens_response, request.provider);
    oauth_tokens_data::insert_oauth_token(&oauth_tokens).await?;

    Ok(ApiResponse {
        status_code: 200,
        data: json!({ "oauth_tokens": oauth_tokens }),
        errors: None,
    })
}

fn handle_get_refresh_token(
    tokens_response: &oauth2::StandardTokenResponse<
        oauth2::EmptyExtraTokenFields,
        oauth2::basic::BasicTokenType,
    >,
) -> String {
    let refresh_token = match tokens_response.refresh_token() {
        Some(token) => token.secret().to_string(),
        None => {
            warn!("refresh_token não estava presente na resposta");
            String::new()
        }
    };

    refresh_token
}

fn extract_oauth_tokens(
    tokens_response: oauth2::StandardTokenResponse<
        oauth2::EmptyExtraTokenFields,
        oauth2::basic::BasicTokenType,
    >,
    provider: OAuthProvider,
) -> OAuthTokens {
    let now = Utc::now();
    let access_token = tokens_response.access_token().secret().to_string();
    debug!("Access Token: {}", access_token);

    let refresh_token = handle_get_refresh_token(&tokens_response);

    let expires_in = tokens_response.expires_in().unwrap().as_millis();
    let expiry_date = now + Duration::from_millis(expires_in.try_into().unwrap());

    let oauth_tokens = OAuthTokens {
        id_oauth_tokens: None,
        access_token,
        refresh_token,
        expiry_date,
        provider,
    };

    oauth_tokens
}

fn extract_code_from_url(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = reqwest::Url::parse(url)?;
    let code = url
        .query_pairs()
        .find(|(key, _)| key == "code")
        .map(|(_, value)| value.into_owned())
        .ok_or_else(|| panic!("Code not found in URL"));

    Ok(code.unwrap())
}

/// Business logic for refreshing an OAuth access token
pub async fn refresh_access_token(
    request: RefreshAccessTokenRequest,
) -> Result<ApiResponse, Box<dyn std::error::Error>> {
    let client = get_oauth_client(request.provider.clone())?;

    // Create the provider to get scopes and params
    let oauth_provider = OAuthProviderFactory::create_provider(
        &request.provider,
        String::new(),
        String::new(),
        String::new(),
    );

    let refresh_token = RefreshToken::new(request.refresh_token.to_owned());
    let mut refresh_request = client.exchange_refresh_token(&refresh_token);

    // Add provider-specific scopes
    for scope in oauth_provider.get_auth_scopes() {
        refresh_request = refresh_request.add_scope(scope);
    }

    // Add provider-specific parameters
    for (key, value) in oauth_provider.get_additional_token_params() {
        refresh_request = refresh_request.add_extra_param(key, value);
    }

    match refresh_request
        .request_async(oauth2::reqwest::async_http_client)
        .await
    {
        Ok(tokens_response) => {
            let mut oauth_tokens = extract_oauth_tokens(tokens_response, request.provider.clone());
            oauth_tokens.refresh_token = request.refresh_token;

            if let Some(on_update_exception) =
                oauth_tokens_data::update_oauth_token_by_refresh_token(&oauth_tokens)
                    .await
                    .err()
            {
                error!(
                    "Erro ao tentar atualizar oauth tokens: {}",
                    on_update_exception
                );
            }

            info!("Access Token gerado com sucesso");

            Ok(ApiResponse {
                status_code: 200,
                data: json!({ "oauth_tokens": oauth_tokens }),
                errors: None,
            })
        }
        Err(e) => {
            error!("Failed to refresh access token: {}", e);
            Err(Box::new(e))
        }
    }
}

/// Business logic for generating access token using stored refresh tokens
pub async fn generate_access_token() -> Result<ApiResponse, Box<dyn std::error::Error>> {
    let response_oauth_tokens =
        oauth_tokens_data::fn_get_first_oauth_tokens_by_last_expiry_date().await?;

    match response_oauth_tokens {
        Some(oauth_tokens) => {
            // Consider token expired if it will expire in the next 60 seconds
            let now = chrono::Utc::now();
            let expiry_buffer = chrono::Duration::seconds(60);
            if oauth_tokens.expiry_date > now + expiry_buffer {
                // Token is still valid
                let response = ApiResponse {
                    status_code: 200,
                    data: json!({ "access_token": oauth_tokens.access_token }),
                    errors: None,
                };
                Ok(response)
            } else {
                // Token expired or about to expire, refresh it
                let response = refresh_access_token(RefreshAccessTokenRequest {
                    refresh_token: oauth_tokens.refresh_token,
                    provider: oauth_tokens.provider,
                })
                .await;

                response
            }
        }
        None => {
            let response = ApiResponse {
                status_code: 500,
                data: serde_json::Value::Null,
                errors: Some(vec![String::from(
                    "Não foram encontrados refresh_token disponíveis para geração do access_token",
                )]),
            };

            Ok(response)
        }
    }
}


/// Business logic for generating OAuth authorization URL for default provider (Microsoft)
pub async fn generate_oauth_url() -> Result<ApiResponse, Box<dyn std::error::Error>> {
    generate_oauth_url_for_provider(OAuthProvider::Microsoft).await
}

/// Business logic for generating OAuth authorization URL for specific provider
pub async fn generate_oauth_url_for_provider(
    provider: OAuthProvider,
) -> Result<ApiResponse, Box<dyn std::error::Error>> {
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

    Ok(ApiResponse {
        status_code: 200,
        data: json!({
            "url": auth_url.to_string(),
            "provider": provider.to_string()
        }),
        errors: None,
    })
}
