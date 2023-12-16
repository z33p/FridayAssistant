use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RefreshAccessTokenRequest {
    pub refresh_token: String,
}
