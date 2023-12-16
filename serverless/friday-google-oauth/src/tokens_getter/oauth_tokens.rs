use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct OAuthTokens {
    pub access_token: String,
    pub refresh_token: String,
    pub expiry_date: i64,
    pub expiry_date_utc: String
}