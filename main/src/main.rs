use lambda_runtime::service_fn;
use std::error::Error;
use tracing::log::error;

mod alexa_handler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let func = service_fn(alexa_handler::handler);
    let res = lambda_runtime::run(func).await;

    if res.is_ok() {
        return Ok(());
    }

    let err = res.err().unwrap();
    error!("Error: {}", err.to_string());

    Err(err)
}
