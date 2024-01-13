use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct BusinessResponse {
    pub status_code: u16,
    pub data: serde_json::Value,
    pub errors: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LambdaResponse {
    pub status_code: u16,
    pub data: serde_json::Value,
    pub errors: Option<Vec<String>>,
    pub correlation_id: Option<String>,
}
