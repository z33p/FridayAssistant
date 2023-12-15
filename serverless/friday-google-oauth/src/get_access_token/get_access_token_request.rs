use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GetAccessTokenRequest {
    pub url: String,
}
