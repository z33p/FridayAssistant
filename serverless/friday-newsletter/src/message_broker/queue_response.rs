use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct BusinessResponse {
    pub status_code: u16,
    pub data: serde_json::Value,
    pub errors: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QueueResponse {
    pub status_code: u16,
    pub data: serde_json::Value,
    pub errors: Option<Vec<String>>,
    #[serde(rename = "correlationId")]
    pub correlation_id: Option<String>,
}
