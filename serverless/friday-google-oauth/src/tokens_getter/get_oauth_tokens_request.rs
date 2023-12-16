use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GetOAuthTokensRequest {
    pub url: String,
}
