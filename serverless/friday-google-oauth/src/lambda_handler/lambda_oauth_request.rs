use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LambdaOAuthRequest {
    pub action: String,
    pub data: serde_json::Value,
}
