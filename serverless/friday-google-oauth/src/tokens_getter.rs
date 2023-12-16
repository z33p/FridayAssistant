use std::time::Duration;

use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::types::AttributeValue;
use chrono::Utc;
use chrono_tz::America::Sao_Paulo;
use oauth2::{AuthorizationCode, RefreshToken, TokenResponse};

use serde_json::json;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use crate::{get_oauth_client, lambda_handler::lambda_oauth_response::LambdaOAuthResponse};

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
    let client = get_oauth_client()?;

    let code = AuthorizationCode::new(extract_code_from_url(&request.url)?);
    let tokens_response = client
        .exchange_code(code)
        .request_async(oauth2::reqwest::async_http_client)
        .await?;

    let oauth_tokens = extract_oauth_tokens(tokens_response);
    db_insert_oauth(&oauth_tokens).await;

    Ok(LambdaOAuthResponse {
        status_code: 200,
        data: json!({ "oauth_tokens": oauth_tokens }),
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
    let expiry_date_utc = expiry_date
        .with_timezone(&Sao_Paulo)
        .to_rfc3339_opts(chrono::SecondsFormat::Secs, false);

    let oauth_tokens = OAuthTokens {
        access_token,
        refresh_token,
        expiry_date: expiry_date.timestamp_millis(),
        expiry_date_utc: expiry_date_utc,
    };

    oauth_tokens
}

async fn db_insert_oauth(oauth_tokens: &OAuthTokens) {
    let client = get_aws_client().await;

    let tb_oauth_tokens = "tb_oauth_tokens";
    let id_oauth_tokens = Uuid::new_v4().to_string();

    let request = client
        .put_item()
        .table_name(tb_oauth_tokens)
        .item(
            "id_oauth_tokens",
            AttributeValue::S(id_oauth_tokens.to_owned()),
        )
        .item(
            "access_token",
            AttributeValue::S(oauth_tokens.access_token.to_owned()),
        )
        .item(
            "refresh_token",
            AttributeValue::S(oauth_tokens.refresh_token.to_owned()),
        )
        .item(
            "expiry_date",
            AttributeValue::N(oauth_tokens.expiry_date.to_string()),
        )
        .item(
            "expiry_date_utc",
            AttributeValue::S(oauth_tokens.expiry_date_utc.to_owned()),
        );
    info!(
        "Iniciando inserção na tabela {} uuid {}",
        tb_oauth_tokens, id_oauth_tokens
    );

    _ = request.send().await.unwrap();
    info!("Tokens inseridos com sucesso uuid {}", id_oauth_tokens);
}

async fn get_aws_client() -> aws_sdk_dynamodb::Client {
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = aws_sdk_dynamodb::Client::new(&config);
    debug!("Client AWS criado com sucesso");
    client
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
    let client = get_oauth_client()?;

    match client
        .exchange_refresh_token(&RefreshToken::new(request.refresh_token))
        .request_async(oauth2::reqwest::async_http_client)
        .await
    {
        Ok(tokens_response) => {
            let oauth_tokens = extract_oauth_tokens(tokens_response);

            Ok(LambdaOAuthResponse {
                status_code: 200,
                data: json!({ "oauth_tokens": oauth_tokens }),
            })
        }
        Err(e) => {
            error!("Failed to refresh access token: {}", e);
            Err(Box::new(e))
        }
    }
}
