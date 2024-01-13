use std::error::Error;

use lapin::{options::*, types::FieldTable, BasicProperties, Connection, ConnectionProperties};
use tracing::debug;

use crate::{lambda_handler::responses::LambdaResponse, ENV_CONFIG};

pub async fn respond_to_sagas(payload: &LambdaResponse) -> Result<(), Box<dyn std::error::Error>> {
    let channel = get_channel().await?;

    // Declare a queue
    let queue_name = &ENV_CONFIG.queue_newsletter_message_broker_response;
    declare_queue(&channel, queue_name).await?;

    // Publish a message
    let json_payload = serde_json::to_string(&payload)?;

    // Attempt to publish the message
    match queue_publish(channel, queue_name, &json_payload).await {
        Ok(_) => debug!("Sent: {}", json_payload),
        Err(e) => {
            tracing::error!("Erro ao publicar na fila {} mensagem: {}", queue_name, e);
            return Err(e.into());
        }
    }

    Ok(())
}

async fn queue_publish(
    channel: lapin::Channel,
    queue_name: &str,
    payload: &str,
) -> Result<(), Box<dyn Error>> {
    channel
        .basic_publish(
            "",
            queue_name,
            BasicPublishOptions::default(),
            payload.as_bytes(),
            BasicProperties::default(),
        )
        .await?;
    Ok(())
}

async fn declare_queue(channel: &lapin::Channel, queue_name: &str) -> Result<(), Box<dyn Error>> {
    let _queue = channel
        .queue_declare(
            queue_name,
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;
    Ok(())
}

async fn get_channel() -> Result<lapin::Channel, Box<dyn Error>> {
    let addr = format!(
        "amqp://{}:{}@{}:{}/%2f",
        ENV_CONFIG.rabbit_user,
        ENV_CONFIG.rabbit_password,
        ENV_CONFIG.rabbit_host,
        ENV_CONFIG.rabbit_port,
    );
    let conn = Connection::connect(&addr, ConnectionProperties::default()).await?;
    let channel = conn.create_channel().await?;
    Ok(channel)
}
