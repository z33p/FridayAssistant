use serde_derive::{Deserialize, Serialize};

use crate::oauth_provider::OAuthProvider;

#[derive(Debug, Deserialize, Serialize)]
pub struct GetOAuthTokensRequest {
    pub url: String,
    pub provider: OAuthProvider,
}
