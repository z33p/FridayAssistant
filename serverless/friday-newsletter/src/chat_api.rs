use reqwest::Client;
use serde_derive::{Deserialize, Serialize};
use serde_json::json;

use crate::load_env;

pub async fn send_request_to_openai(text: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();

    let env_variables = load_env::load_env_variables();

    let request_body = json!({
        "model": "text-davinci-003",
        "prompt": text,
        "max_tokens": env_variables.request_max_tokens,
        "temperature": 0.3,
        "max_tokens": 64,
        "top_p": 1.0,
        "frequency_penalty": 0.2,
        "presence_penalty": 0.2,
    });

    let request_body = serde_json::to_string(&request_body).unwrap();

    let response = client
        .post("https://api.openai.com/v1/completions")
        .header("Content-Type", "application/json")
        .header(
            "Authorization",
            format!("Bearer {}", env_variables.open_ai_api_key),
        )
        .body(request_body)
        .send()
        .await?;

    let response_body = response.text().await?;
    let response: AiResponse = serde_json::from_str(&response_body)?;

    let first_choice = response
        .choices
        .first()
        .ok_or("No choices found in response")?;

    let summary = first_choice.text.clone();

    Ok(summary)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Choice {
    pub text: String,
    // index: u8,
    // logprobs: Option<()>,
    pub finish_reason: String,
}

// #[derive(Debug, Deserialize, Serialize)]
// pub struct Usage {
//     prompt_tokens: u8,
//     completion_tokens: u8,
//     total_tokens: u8,
// }

#[derive(Debug, Deserialize, Serialize)]
pub struct AiResponse {
    // id: String,
    // object: String,
    // created: i64,
    // model: String,
    pub choices: Vec<Choice>,
    // usage: Usage,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    #[serde(rename = "Summary")]
    summary: AiResponse,
}
