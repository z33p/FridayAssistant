use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;
use tracing::debug;

use crate::{models::chat_api_dto::chat_response::ChatResponse, ENV_CONFIG};

pub async fn rank_most_relevant_text(
    user_prompt: &str,
) -> Result<Vec<usize>, Box<dyn std::error::Error>> {
    let system_prompt = format!(
        "{} {} {}",
        "Classifique as manchetes a seguir nessa ordem: Tecnologias de ponta, AI ou Crypto, polêmicas Tech e mais interessante no geral",
        "Retorne até 3 resultados em um formato de lista similar ao exemplo:",
        "\"7,0,11\" onde o número representa a ordem da notícia iniciando de 0"
    );

    let max_tokens = 32;
    let text_result =
        send_request_to_openai(system_prompt.as_str(), user_prompt, max_tokens).await?;

    let numbers: Vec<usize> = text_result
        .split(',')
        .map(|s| s.trim().parse::<usize>())
        .filter_map(Result::ok)
        .collect();

    return Ok(numbers);
}

pub async fn write_relevant_post_about(
    user_prompt: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let system_prompt = format!(
        "{} {} {} {} {} {}",
        "Em formato educacional de post para o LinkedIn.",
        "Escreva de forma pessoal você não está somente compartilhando a notícia. Você entende do tema e sua importância.",
        "Sempre responda o texto nessa estrutura:",
        "1 paragrafo frases sobre a relevância do tópico,",
        "1 paragrafo sobre a tecnologia e a relevância da tecnologia em si,",
        "1 frase finalize o texto.",
    );

    let formated_user_prompt = format!("Escreva sobre esse tema: \"{}\"", user_prompt);

    debug!("System Prompt: {}", system_prompt);
    debug!("User Prompt: {}", formated_user_prompt);

    return send_request_to_openai(system_prompt.as_str(), formated_user_prompt.as_str(), 2048)
        .await;
}

async fn send_request_to_openai(
    system_prompt: &str,
    user_prompt: &str,
    max_tokens: u16,
) -> Result<String, Box<dyn std::error::Error>> {
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
            {"role": "system", "content": system_prompt},
            {"role": "user", "content": user_prompt}
        ],
        "model": "gpt-3.5-turbo",  // specify the model
        "frequency_penalty": 0.0,  // decreases the model's likelihood to repeat the same line verbatim
        "logprobs": null,  // include the log probabilities on the logprobs most likely tokens, up to a maximum of 5. Set to null if not used
        "max_tokens": max_tokens,  // maximum number of tokens to generate
        "n": 1,  // number of completions to generate
        "presence_penalty": 0.0,  // increases the model's likelihood to talk about new topics
        "stop": ["\n", "<|endoftext|>"],  // stop sequence to end generation
        "stream": false,  // whether to stream back partial progress
        "temperature": 0.7,  // controls randomness
        "top_p": 1.0,  // controls diversity
    });

    // URL for the OpenAI completion endpoint
    let url = "https://api.openai.com/v1/chat/completions";
    debug!("OpenAi completions url: {}", url);

    // Create a client and post the request
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .headers(headers)
        .body(body.to_string())
        .send()
        .await?;

    let response_body = response.text().await?;
    debug!("OpenAi completions success response: {}", response_body);

    let response: ChatResponse = serde_json::from_str(&response_body)?;

    let first_choice = response
        .choices
        .first()
        .ok_or("No choices found in response")?;

    let summary = first_choice.message.content.clone().unwrap_or_default();

    Ok(summary)
}
