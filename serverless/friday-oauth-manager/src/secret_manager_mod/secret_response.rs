use serde_derive::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Response structure from Friday Secret Manager API
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct SecretResponse {
    pub success: bool,
    pub data: Option<String>,
    pub errors: Vec<String>,
}
