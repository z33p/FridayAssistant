mod chat_api;
mod load_env;
use std::convert::Infallible;

#[tokio::main]
async fn main() -> Result<(), Infallible> {
    let text = "Escreva um pequeno resumo sobre fundos investimento FMP";
    let summary = chat_api::send_request_to_openai(&text).await.unwrap();

    println!("Summary: {}", summary);
    Ok(())
}
