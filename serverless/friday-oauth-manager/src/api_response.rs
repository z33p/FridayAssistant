use serde_derive::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Standard API response wrapper
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ApiResponse {
    /// HTTP status code
    #[schema(example = 200)]
    pub status_code: u16,
    /// Response data payload
    pub data: serde_json::Value,
    /// Error messages if any
    pub errors: Option<Vec<String>>,
}
