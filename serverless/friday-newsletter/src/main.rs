mod chat_api;
mod lambda_handler;
mod linkedin_news_post;
mod load_env;
mod message_broker_sagas_queue;
mod search_news;

extern crate dotenv;

use dotenv::dotenv;
use lambda_runtime::service_fn;
use load_env::{load_env_variables, EnvVariables};
use once_cell::sync::Lazy;
use tracing::{error, Level};

static ENV_CONFIG: Lazy<EnvVariables> = Lazy::new(|| load_env_variables());

#[tokio::main]
async fn main() {
    dotenv().ok();
    logging_init();

    let func = service_fn(lambda_handler::handler);
    let res = lambda_runtime::run(func).await;

    if res.is_ok() {
        return;
    }

    let err = res.err().unwrap();
    error!("Error: {}", err.to_string());
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
