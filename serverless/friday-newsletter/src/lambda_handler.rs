// use std::error::Error;

use std::error::Error;

use lambda_runtime::LambdaEvent;

use crate::{linkedin_news_post, message_broker_sagas_queue};

use self::{
    lambda_request::LambdaRequest,
    responses::{BusinessResponse, LambdaResponse},
};

pub mod lambda_request;
pub mod responses;

pub async fn handler(
    event: LambdaEvent<LambdaRequest>,
) -> Result<LambdaResponse, lambda_runtime::Error> {
    let business_response = match event.payload.action.as_ref() {
        "GENERATE_LINKEDIN_NEWS_POST" => {
            let response = linkedin_news_post::generate_post().await;
            response
        }
        _ => {
            panic!(
                "Ação desconhecida não implementada: {}",
                event.payload.action
            )
        }
    };

    let lambda_response =
        map_lambda_response(business_response, event.payload.correlation_id.clone());

    if event.payload.correlation_id.is_some() {
        _ = message_broker_sagas_queue::respond_to_sagas(&lambda_response).await;
    }

    Ok(lambda_response)
}

fn map_lambda_response(
    business_response: Result<BusinessResponse, Box<dyn Error>>,
    correlation_id: Option<String>,
) -> LambdaResponse {
    let response = business_response.unwrap();

    let lambda_response = LambdaResponse {
        status_code: response.status_code,
        data: response.data,
        errors: response.errors,
        correlation_id,
    };

    lambda_response
}
