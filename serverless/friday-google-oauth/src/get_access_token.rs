use oauth2::{AuthorizationCode, TokenResponse};

use serde_json::json;
use tracing::debug;

use crate::{get_oauth_client, lambda_handler::lambda_oauth_response::LambdaOAuthResponse};

use self::get_access_token_request::GetAccessTokenRequest;

pub mod get_access_token_request;

pub async fn get_access_token(
    request: GetAccessTokenRequest,
) -> Result<LambdaOAuthResponse, Box<dyn std::error::Error>> {
    let client = get_oauth_client()?;

    let code = AuthorizationCode::new(extract_code_from_url(&request.url)?);
    let token_result = client
        .exchange_code(code)
        .request_async(oauth2::reqwest::async_http_client)
        .await?;

    let access_token = token_result.access_token().secret();

    debug!("{}", access_token);

    Ok(LambdaOAuthResponse {
        status_code: 200,
        data: json!({ "tokens": access_token }),
    })
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
