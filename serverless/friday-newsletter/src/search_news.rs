extern crate reqwest;

use crate::ENV_CONFIG;
use reqwest::header;
use tracing::debug;
use std::fs::File;
use std::io::Read;

use self::news_contracts::NewsResponse;

mod news_contracts;

pub async fn handle_get_news() -> Result<NewsResponse, Box<dyn std::error::Error>> {
    let response_body: String;

    if ENV_CONFIG.is_prod {
        response_body = fetch_news().await?;
    } else {
        response_body = read_local_file()?;
    }

    let news_response: NewsResponse = serde_json::from_str(&response_body)?;

    Ok(news_response)
}

pub async fn fetch_news() -> Result<String, Box<dyn std::error::Error>> {
    let user_agent = "FridayNewsletter/1.0";

    let base_url = "https://newsapi.org/v2/top-headlines";
    let country = "country=br";
    let category = "category=technology";
    let api_key = format!("apiKey={}", ENV_CONFIG.news_api_key);

    let url = format!("{}?{}&{}&{}", base_url, country, category, api_key);
    debug!("OpenNews search headlines url: {}", url);

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(header::USER_AGENT, user_agent)
        .send()
        .await?;

    if response.status().is_success() {
        let response_body = response.text().await?;
        debug!(
            "OpenNews search headlines success response: {}",
            response_body
        );

        return Ok(response_body);
    }

    let response_body = response.text().await?;
    panic!(
        "OpenNews search headlines error response: {}",
        response_body
    );
}

pub fn read_local_file() -> Result<String, Box<dyn std::error::Error>> {
    let mut file = File::open("top-headlines.json")?;
    let mut json_str = String::new();
    file.read_to_string(&mut json_str)?;

    Ok(json_str)
}
