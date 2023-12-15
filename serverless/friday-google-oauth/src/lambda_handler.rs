use lambda_runtime::{Error, LambdaEvent};

use crate::{
    generate_oauth_url,
    get_access_token::{self, get_access_token_request::GetAccessTokenRequest},
};

use self::{lambda_oauth_request::LambdaOAuthRequest, lambda_oauth_response::LambdaOAuthResponse};

pub mod lambda_oauth_request;
pub mod lambda_oauth_response;

pub async fn handler(event: LambdaEvent<LambdaOAuthRequest>) -> Result<LambdaOAuthResponse, Error> {
    let lambda_response = match event.payload.action.as_ref() {
        "GENERATE_OAUTH_URL" => {
            let response = generate_oauth_url::generate_oauth_url().await;
            response
        }
        "GET_ACCESS_TOKEN" => {
            let request_get_access_token: GetAccessTokenRequest = serde_json::from_value(event.payload.data)
                .expect("Falha ao deserializar GetAccessTokenRequest");
            let response = get_access_token::get_access_token(request_get_access_token).await;
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
