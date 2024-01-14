use serde_derive::{Deserialize, Serialize};

use super::chat_api_contracts::{Choice, Usage};

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}
