use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;

use crate::ENV_CONFIG;

use self::chat_response::ChatResponse;

mod chat_api_contracts;
mod chat_response;

pub async fn send_request_to_openai(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Set up the headers
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", ENV_CONFIG.open_ai_api_key).parse()?,
    );
    headers.insert(CONTENT_TYPE, "application/json".parse()?);

    // Define the request body with all available parameters
    let body = json!({
        "messages": [
            {"role": "system", "content": "Em uma frase fale sobre esse tópico"},
            {"role": "user", "content": prompt}
        ],
        "model": "gpt-3.5-turbo",  // specify the model
        "frequency_penalty": 0.0,  // decreases the model's likelihood to repeat the same line verbatim
        "logprobs": null,  // include the log probabilities on the logprobs most likely tokens, up to a maximum of 5. Set to null if not used
        "max_tokens": ENV_CONFIG.request_max_tokens,  // maximum number of tokens to generate
        "n": 1,  // number of completions to generate
        "presence_penalty": 0.0,  // increases the model's likelihood to talk about new topics
        "stop": ["\n", "<|endoftext|>"],  // stop sequence to end generation
        "stream": false,  // whether to stream back partial progress
        "temperature": 0.7,  // controls randomness
        "top_p": 1.0,  // controls diversity
    });

    // URL for the OpenAI completion endpoint
    let url = "https://api.openai.com/v1/chat/completions";
    print!("{}", url);

    // Create a client and post the request
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .headers(headers)
        .body(body.to_string())
        .send()
        .await?;

    let response_body = response.text().await?;
    print!("{}", response_body);

    let response: ChatResponse = serde_json::from_str(&response_body)?;

    let first_choice = response
        .choices
        .first()
        .ok_or("No choices found in response")?;

    let summary = first_choice.message.content.clone().unwrap_or_default();

    Ok(summary)
}
