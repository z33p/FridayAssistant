use serde_json::json;
use tracing::debug;
mod chat_api;
mod search_news;

use crate::models::business_response::BusinessResponse;

pub mod generate_news_post_queue;

pub async fn generate_post() -> Result<BusinessResponse, Box<dyn std::error::Error>> {
    let news_response = search_news::handle_get_news().await.unwrap();

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

    let mut post_content_list = Vec::new();

    let relevant_articles_indexes = chat_api::rank_most_relevant_text(concatenated_titles.as_str())
        .await
        .unwrap();

    for article_index in relevant_articles_indexes {
        let article = news_response.articles.get(article_index).unwrap();

        let post_content = chat_api::write_relevant_post_about(&article.title)
            .await
            .unwrap();

        debug!("Relevant post written: \n\n{}\n", post_content);

        post_content_list.push(post_content);
    }

    debug!("Finished");

    let response = BusinessResponse {
        status_code: 200,
        data: json!(post_content_list),
        errors: None,
    };

    Ok(response)
}
