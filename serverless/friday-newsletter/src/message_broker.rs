use std::{error::Error, pin::Pin, sync::Arc};

use futures_util::{Future, StreamExt};
use lapin::{
    options::{BasicConsumeOptions, BasicPublishOptions, BasicQosOptions, QueueDeclareOptions},
    types::FieldTable,
    BasicProperties, Channel, Connection, ConnectionProperties,
};
use serde::de::DeserializeOwned;
use tokio::{sync::Semaphore, task};
use urlencoding::encode;

use crate::ENV_CONFIG;

use self::queue_response::{BusinessResponse, QueueResponse};

pub mod generate_news_post_queue;
pub mod queue_request;
pub mod queue_response;
pub mod sagas_queue;

pub async fn declare_queue(
    channel: &lapin::Channel,
    queue_name: &str,
) -> Result<(), Box<dyn Error>> {
    let _queue = channel
        .queue_declare(
            queue_name,
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;
    Ok(())
}

pub async fn get_channel() -> Result<lapin::Channel, Box<dyn Error>> {
    let addr = format!(
        "amqp://{}:{}@{}:5672/%2f",
        ENV_CONFIG.rabbit_user,
        encode(&ENV_CONFIG.rabbit_password),
        ENV_CONFIG.rabbit_host,
    );

    let conn = Connection::connect(&addr, ConnectionProperties::default()).await;

    match conn {
        Ok(conn) => {
            let channel = conn.create_channel().await?;
            Ok(channel)
        }
        Err(err) => {
            tracing::error!("Falha ao se conectar ao RabbitMq error: {}", err);
            Err(err.into())
        }
    }
}

pub async fn queue_publish(
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

pub async fn multi_thread_consume<T, F>(
    channel: Channel,
    queue_name: &str,
    handler: F,
    prefetch_count: u16,
    max_concurrent_messages: u16,
) -> Result<(), Box<dyn Error>>
where
    T: DeserializeOwned + Send,
    F: FnMut(T) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + 'static + Clone,
{
    for _ in 0..max_concurrent_messages {
        _ = single_thread_consume(&channel, queue_name, handler.clone(), prefetch_count).await;
    }

    Ok(())
}

pub async fn single_thread_consume<T, F>(
    channel: &Channel,
    queue_name: &str,
    mut handler: F,
    prefetch_count: u16,
) -> Result<(), Box<dyn Error>>
where
    T: DeserializeOwned + Send,
    F: FnMut(T) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + 'static,
{
    channel
        .basic_qos(prefetch_count, BasicQosOptions::default())
        .await?;

    let mut consumer = channel
        .basic_consume(
            queue_name,
            "",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    let queue_name = String::from(queue_name);

    task::spawn(async move {
        while let Some(delivery_result) = consumer.next().await {
            if let Ok(delivery) = delivery_result {
                match serde_json::from_slice::<T>(&delivery.data) {
                    Ok(data) => handler(data).await,
                    Err(e) => {
                        tracing::error!(
                            "Erro ao deserializar mensagem da fila '{}': {:?}",
                            queue_name,
                            e
                        );
                        continue;
                    }
                }

                if let Err(e) = delivery.ack(Default::default()).await {
                    tracing::error!(
                        "Erro ao acknowledge mensagem da fila '{}': {:?}",
                        queue_name,
                        e
                    );
                }
            } else {
                tracing::error!("Erro ao receber mensagem da fila: {}", queue_name);
            }
        }
    });

    Ok(())
}

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
