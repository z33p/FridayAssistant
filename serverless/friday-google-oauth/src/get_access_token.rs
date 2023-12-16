use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::types::AttributeValue;
use chrono::{TimeZone, Utc};
use oauth2::{AuthorizationCode, TokenResponse};

use serde_json::json;
use tracing::{debug, info, warn};
use uuid::Uuid;

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

    let access_token = token_result.access_token().secret().to_string();
    debug!("Access Token: {}", access_token);

    let refresh_token = handle_get_refresh_token(&token_result);
    let expiry_date = token_result.expires_in().unwrap().as_millis();

    db_insert_tokens(&access_token, refresh_token, expiry_date).await;

    Ok(LambdaOAuthResponse {
        status_code: 200,
        data: json!({ "tokens": access_token }),
    })
}

fn handle_get_refresh_token(
    token_result: &oauth2::StandardTokenResponse<
        oauth2::EmptyExtraTokenFields,
        oauth2::basic::BasicTokenType,
    >
) -> String {
    let refresh_token = match token_result.refresh_token() {
        Some(token) => token.secret().to_string(),
        None => {
            warn!("refresh_token não estava presente na resposta");
            String::new()
        }
    };

    refresh_token
}

async fn db_insert_tokens(access_token: &String, refresh_token: String, expiry_date: u128) {
    let client = get_aws_client().await;

    let expiry_date_utc = Utc
        .timestamp_millis_opt(cast_u128_into_i64(expiry_date))
        .unwrap()
        .to_rfc3339_opts(chrono::SecondsFormat::Secs, false);

    let tb_oauth_tokens = "tb_oauth_tokens";
    let id_oauth_tokens = Uuid::new_v4().to_string();

    let request = client
        .put_item()
        .table_name(tb_oauth_tokens)
        .item(
            "id_oauth_tokens",
            AttributeValue::S(id_oauth_tokens.to_owned()),
        )
        .item("access_token", AttributeValue::S(access_token.to_owned()))
        .item("refresh_token", AttributeValue::N(refresh_token))
        .item("expiry_date", AttributeValue::S(expiry_date.to_string()))
        .item("expiry_date_utc", AttributeValue::S(expiry_date_utc));

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

fn cast_u128_into_i64(expiry_date: u128) -> i64 {
    let expiry_date_i64: i64 = if expiry_date <= i64::MAX as u128 {
        expiry_date as i64
    } else {
        panic!("Não foi possível realizar o cast de u128 em i64")
    };
    expiry_date_i64
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
