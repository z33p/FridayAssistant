use crate::{
    generate_oauth_url,
    tokens_getter::{
        self, get_oauth_tokens_request::GetOAuthTokensRequest,
        refresh_access_token_request::RefreshAccessTokenRequest,
    },
};

use self::{lambda_oauth_request::LambdaOAuthRequest, lambda_oauth_response::LambdaOAuthResponse};
use lambda_http::{IntoResponse, Request, RequestPayloadExt};

pub mod lambda_oauth_request;
pub mod lambda_oauth_response;

pub async fn handler(
    lambda_request: Request,
) -> Result<impl IntoResponse, Box<dyn std::error::Error>> {
    let payload = lambda_request
        .payload::<LambdaOAuthRequest>()
        .expect("Erro ao deserializar request payload")
        .expect("Erro ao deserializar o payload estava vazio");

    let logic_response = match payload.action.as_ref() {
        "GENERATE_ACCESS_TOKEN" => {
            let response = tokens_getter::generate_access_token().await;
            response
        }
        "REFRESH_ACCESS_TOKEN" => {
            let request_refresh_access_token: RefreshAccessTokenRequest =
                serde_json::from_value(payload.data)
                    .expect("Falha ao deserializar GetAccessTokenRequest");
            let response = tokens_getter::refresh_access_token(request_refresh_access_token).await;
            response
        }
        "GENERATE_OAUTH_URL" => {
            let response = generate_oauth_url::generate_oauth_url().await;
            response
        }
        "GET_OAUTH_TOKENS" => {
            let request_get_oauth_token: GetOAuthTokensRequest =
                serde_json::from_value(payload.data)
                    .expect("Falha ao deserializar GetAccessTokenRequest");
            let response = tokens_getter::get_oauth_tokens(request_get_oauth_token).await;
            response
        }
        _ => {
            panic!("Ação desconhecida não implementada: {}", payload.action)
        }
    };

    let http_response = match logic_response {
        Ok(response) => parse_http_response(response),
        _ => {
            let response = LambdaOAuthResponse {
                status_code: 500,
                data: serde_json::Value::Null,
                errors: Some(vec![String::from("Ocorreu um erro inesperado")]),
            };

            parse_http_response(response)
        }
    };

    Ok(http_response)
}

fn parse_http_response(response: LambdaOAuthResponse) -> String {
    serde_json::to_string(&response).unwrap_or_else(|_| "Erro na conversão para JSON".to_string())
}
