mod chat_api;
mod load_env;
mod search_news;

use load_env::{load_env_variables, EnvVariables};
use once_cell::sync::Lazy;

static ENV_CONFIG: Lazy<EnvVariables> = Lazy::new(|| load_env_variables());

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let news_response = search_news::handle_get_news().await?;

    for article in news_response.articles.iter() {
        println!("{}", article.title);
    }

    println!("Finished");
    Ok(())
}
