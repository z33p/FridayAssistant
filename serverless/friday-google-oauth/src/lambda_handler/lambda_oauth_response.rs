use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LambdaOAuthResponse {
    pub status_code: u16,
    pub data: serde_json::Value,
    pub errors: Option<Vec<String>>
}
