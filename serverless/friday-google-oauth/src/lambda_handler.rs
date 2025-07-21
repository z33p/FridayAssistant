use lambda_runtime::LambdaEvent;

use crate::{
    generate_oauth_url::{self, generate_oauth_url_for_provider},
    oauth_provider::OAuthProvider,
    tokens_getter::{
        self, get_oauth_tokens_request::GetOAuthTokensRequest,
        refresh_access_token_request::RefreshAccessTokenRequest,
    },
};

use self::{lambda_request::LambdaRequest, lambda_response::LambdaResponse};

pub mod lambda_request;
pub mod lambda_response;

pub async fn handler(
    event: LambdaEvent<LambdaRequest>,
) -> Result<LambdaResponse, lambda_runtime::Error> {
    let lambda_response = match event.payload.action.as_ref() {
        "GENERATE_ACCESS_TOKEN" => {
            let response = tokens_getter::generate_access_token().await;
            response
        }
        "REFRESH_ACCESS_TOKEN" => {
            let request_refresh_access_token: RefreshAccessTokenRequest =
                serde_json::from_value(event.payload.data)
                    .expect("Falha ao deserializar RefreshAccessTokenRequest");
            let response = tokens_getter::refresh_access_token(request_refresh_access_token).await;
            response
        }
        "GENERATE_OAUTH_URL" => {
            let response = generate_oauth_url::generate_oauth_url().await;
            response
        }
        "GENERATE_GOOGLE_OAUTH_URL" => {
            let response = generate_oauth_url_for_provider(OAuthProvider::Google).await;
            response
        }
        "GENERATE_MICROSOFT_OAUTH_URL" => {
            let response = generate_oauth_url_for_provider(OAuthProvider::Microsoft).await;
            response
        }
        "GET_OAUTH_TOKENS" => {
            let request_get_oauth_token: GetOAuthTokensRequest =
                serde_json::from_value(event.payload.data)
                    .expect("Falha ao deserializar GetOAuthTokensRequest");
            let response = tokens_getter::get_oauth_tokens(request_get_oauth_token).await;
            response
        }
        _ => {
            panic!(
                "Ação desconhecida não implementada: {}",
                event.payload.action
            )
        }
    };

    Ok(lambda_response.unwrap())
}
