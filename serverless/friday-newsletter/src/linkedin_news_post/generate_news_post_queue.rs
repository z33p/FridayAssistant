use std::{error::Error, pin::Pin};

use core_infra::message_broker;
use futures_util::Future;

use crate::{
    linkedin_news_post, message_broker::sagas_queue,
    models::message_broker_dto::queue_request::QueueRequest, ENV_CONFIG,
};

pub async fn start_consume() -> Result<(), Box<dyn Error>> {
    let channel = message_broker::get_channel(
        ENV_CONFIG.rabbit_user.as_str(),
        ENV_CONFIG.rabbit_password.as_str(),
        ENV_CONFIG.rabbit_host.as_str(),
    )
    .await
    .unwrap();

    // Declare a queue
    let queue_name = "NEWSLETTER.GENERATE.NEWS.POST";
    message_broker::declare_queue(&channel, queue_name)
        .await
        .unwrap();

    let prefetch_count = 2;
    let max_concurrent_messages = 2;

    let _ = message_broker::multi_thread_consume(
        channel,
        queue_name,
        handler,
        prefetch_count,
        max_concurrent_messages,
    )
    .await;

    Ok(())
}

// Example handler function
fn handler(data: QueueRequest) -> Pin<Box<dyn Future<Output = ()> + Send>> {
    Box::pin(async move {
        let business_response = linkedin_news_post::generate_post().await;
        sagas_queue::handle_sagas_response(business_response.unwrap(), data.correlation_id).await
    })
}
