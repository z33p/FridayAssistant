use serde::{Deserialize, Serialize};

/// Response structure for OAuth access token requests
#[derive(Debug, Deserialize, Serialize)]
pub struct AccessTokenResponse {
    pub success: bool,
    pub data: Option<String>, // The access token
    pub errors: Vec<String>,
}
