mod chat_api;
mod load_env;
mod search_news;

use load_env::{load_env_variables, EnvVariables};
use once_cell::sync::Lazy;
use tracing::{debug, Level};

static ENV_CONFIG: Lazy<EnvVariables> = Lazy::new(|| load_env_variables());

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    logging_init();

    let news_response = search_news::handle_get_news().await?;

    // Initialize an empty string to store concatenated titles
    let mut concatenated_titles = String::new();

    // Iterate through the list of objects and concatenate title with ordinal number
    for (index, article) in news_response.articles.iter().enumerate() {
        let ordinal_number = index;
        let concatenated_title = format!("{}. {}\n", ordinal_number, &article.title);

        // Append the concatenated title to the existing string
        concatenated_titles.push_str(&concatenated_title);
    }

    debug!("Top Headlines:\n\n{}", concatenated_titles);

    let relevant_articles_indexes = chat_api::rank_most_relevant_text(concatenated_titles.as_str())
        .await
        .unwrap();

    for article_index in relevant_articles_indexes {
        let article = news_response.articles.get(article_index).unwrap();

        let post_content = chat_api::write_relevant_post_about(&article.title).await?;

        debug!("Relevant post written: \n\n{}\n", post_content);
    }

    debug!("Finished");
    Ok(())
}

fn logging_init() {
    let log_level = if ENV_CONFIG.is_prod { Level::INFO } else { Level::DEBUG };

    tracing_subscriber::fmt()
        .with_max_level(log_level)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();
}
