use lambda_runtime::LambdaEvent;

use crate::linkedin_news_post;

use self::{lambda_request::LambdaRequest, lambda_response::LambdaResponse};

pub mod lambda_request;
pub mod lambda_response;

pub async fn handler(
    event: LambdaEvent<LambdaRequest>,
) -> Result<LambdaResponse, lambda_runtime::Error> {
    let lambda_response = match event.payload.action.as_ref() {
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

    Ok(lambda_response.unwrap())
}
