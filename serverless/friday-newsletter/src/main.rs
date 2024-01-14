mod linkedin_news_post;
mod message_broker;
mod models;
mod utils;

extern crate dotenv;

use dotenv::dotenv;
use linkedin_news_post::generate_news_post_queue;
use once_cell::sync::Lazy;
use tracing::Level;
use utils::load_env::{load_env_variables, EnvVariables};

static ENV_CONFIG: Lazy<EnvVariables> = Lazy::new(|| load_env_variables());

#[tokio::main]
async fn main() {
    dotenv().ok();
    logging_init();

    generate_news_post_queue::start_consume()
        .await
        .expect("Erro ao iniciar consumidor generate_news_post_queue");

    let forever = futures_util::future::pending::<()>();
    forever.await;
}

fn logging_init() {
    let log_level = if ENV_CONFIG.is_prod {
        Level::INFO
    } else {
        Level::DEBUG
    };

    tracing_subscriber::fmt()
        .with_max_level(log_level)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();
}
