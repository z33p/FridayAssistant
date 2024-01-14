use std::{error::Error, pin::Pin, time::Duration};

use futures_util::Future;
use tokio::time::sleep;
use tracing::info;

use crate::{linkedin_news_post, message_broker};

use super::{queue_request::QueueRequest, sagas_queue};

pub async fn start_consume() -> Result<(), Box<dyn Error>> {
    let channel = message_broker::get_channel().await.unwrap();

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
        max_concurrent_messages
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
