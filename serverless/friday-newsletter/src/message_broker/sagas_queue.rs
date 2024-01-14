use core_infra::message_broker;
use tracing::debug;

use crate::{
    models::{
        business_response::BusinessResponse, message_broker_dto::queue_response::QueueResponse,
    },
    ENV_CONFIG,
};

use super::map_queue_response;

pub async fn handle_sagas_response(
    business_response: BusinessResponse,
    correlation_id: Option<String>,
) {
    let queue_response = map_queue_response(business_response, correlation_id.clone());

    if correlation_id.is_some() {
        _ = respond_to_sagas(&queue_response).await;
    }
}

pub async fn respond_to_sagas(payload: &QueueResponse) -> Result<(), Box<dyn std::error::Error>> {
    let channel = message_broker::get_channel(
        ENV_CONFIG.rabbit_user.as_str(),
        ENV_CONFIG.rabbit_password.as_str(),
        ENV_CONFIG.rabbit_host.as_str(),
    )
    .await?;

    // Declare a queue
    let queue_name = "NEWSLETTER.MESSAGE.BROKER.RESPONSE";
    message_broker::declare_queue(&channel, queue_name).await?;

    // Publish a message
    let json_payload = serde_json::to_string(&payload)?;

    // Attempt to publish the message
    match message_broker::queue_publish(channel, queue_name, &json_payload).await {
        Ok(_) => debug!("Sent: {}", json_payload),
        Err(e) => {
            tracing::error!("Erro ao publicar na fila {} mensagem: {}", queue_name, e);
            return Err(e.into());
        }
    }

    Ok(())
}
