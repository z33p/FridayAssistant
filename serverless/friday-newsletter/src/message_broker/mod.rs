use crate::models::{
    business_response::BusinessResponse, message_broker_dto::queue_response::QueueResponse,
};

pub mod sagas_queue;

pub fn map_queue_response(
    business_response: BusinessResponse,
    correlation_id: Option<String>,
) -> QueueResponse {
    let queue_response = QueueResponse {
        status_code: business_response.status_code,
        data: business_response.data,
        errors: business_response.errors,
        correlation_id,
    };

    queue_response
}
