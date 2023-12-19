use std::time::Duration;

use chrono::Utc;
use oauth2::{AuthorizationCode, RefreshToken, Scope, TokenResponse};

use serde_json::json;
use tracing::{debug, error, info, warn};

use crate::{
    get_gmail_oauth_client, lambda_handler::lambda_oauth_response::LambdaOAuthResponse,
    oauth_tokens_data,
};

use self::{
    get_oauth_tokens_request::GetOAuthTokensRequest, oauth_tokens::OAuthTokens,
    refresh_access_token_request::RefreshAccessTokenRequest,
};

pub mod get_oauth_tokens_request;
pub mod oauth_tokens;
pub mod refresh_access_token_request;

pub async fn get_oauth_tokens(
    request: GetOAuthTokensRequest,
) -> Result<LambdaOAuthResponse, Box<dyn std::error::Error>> {
    let client = get_gmail_oauth_client()?;

    let code = AuthorizationCode::new(extract_code_from_url(&request.url)?);
    let tokens_response = client
        .exchange_code(code)
        .add_extra_param("access_type", "offline")
        .request_async(oauth2::reqwest::async_http_client)
        .await?;

    let oauth_tokens = extract_oauth_tokens(tokens_response);
    oauth_tokens_data::insert_oauth_token(&oauth_tokens).await?;

    Ok(LambdaOAuthResponse {
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

pub async fn refresh_access_token(
    request: RefreshAccessTokenRequest,
) -> Result<LambdaOAuthResponse, Box<dyn std::error::Error>> {
    let client = get_gmail_oauth_client()?;

    match client
        .exchange_refresh_token(&RefreshToken::new(request.refresh_token.to_owned()))
        .add_extra_param("access_type", "offline")
        .add_scope(Scope::new("https://mail.google.com/".to_string()))
        .request_async(oauth2::reqwest::async_http_client)
        .await
    {
        Ok(tokens_response) => {
            let mut oauth_tokens = extract_oauth_tokens(tokens_response);
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

            Ok(LambdaOAuthResponse {
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

pub async fn generate_access_token() -> Result<LambdaOAuthResponse, Box<dyn std::error::Error>> {
    let response_oauth_tokens = oauth_tokens_data::get_first_oauth_token_by_refresh_token().await?;

    match response_oauth_tokens {
        Some(oauth_tokens) => {
            let response = refresh_access_token(RefreshAccessTokenRequest {
                refresh_token: oauth_tokens.refresh_token,
            })
            .await;

            response
        }
        None => {
            let response = LambdaOAuthResponse {
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
