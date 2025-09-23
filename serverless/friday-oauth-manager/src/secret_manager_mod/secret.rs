use serde_derive::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Secret data structure for Friday Secret Manager
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct Secret {
    pub key: String,
    pub value: String,
}
