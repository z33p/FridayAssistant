use serde_derive::{Deserialize, Serialize};

use crate::oauth_provider::OAuthProvider;

#[derive(Debug, Deserialize, Serialize)]
pub struct RefreshAccessTokenRequest {
    pub refresh_token: String,
    pub provider: OAuthProvider,
}
