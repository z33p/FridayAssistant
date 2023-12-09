mod chat_api;
mod load_env;
mod search_news;

use load_env::{load_env_variables, EnvVariables};
use once_cell::sync::Lazy;

static ENV_CONFIG: Lazy<EnvVariables> = Lazy::new(|| load_env_variables());

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let news_response = search_news::handle_get_news().await?;

    let article = news_response
        .articles
        .first()
        .unwrap();

    println!("{}", article.title);

    let summary = chat_api::send_request_to_openai(article.title.as_str()).await.unwrap();

    println!("{}", summary);

    println!("Finished");
    Ok(())
}
