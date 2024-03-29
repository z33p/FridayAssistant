use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LambdaRequest {
    pub action: String,
    pub data: serde_json::Value,
}
