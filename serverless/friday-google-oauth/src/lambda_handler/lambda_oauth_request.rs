use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LambdaOAuthRequest {
    #[serde(rename = "action")]
    pub action: String,
    #[serde(rename = "data")]
    pub data: serde_json::Value,
}
