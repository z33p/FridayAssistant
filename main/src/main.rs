use lambda_runtime::{service_fn, Error};

mod alexa_handler;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(alexa_handler::handler);
    lambda_runtime::run(func).await?;

    Ok(())
}