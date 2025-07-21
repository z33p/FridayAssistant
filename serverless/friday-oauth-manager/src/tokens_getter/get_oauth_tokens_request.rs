use serde_derive::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::oauth_provider::OAuthProvider;

/// Request to exchange OAuth authorization code for tokens
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct GetOAuthTokensRequest {
    /// OAuth callback URL containing the authorization code
    #[schema(example = "https://your-app.com/callback?code=auth_code_here")]
    pub url: String,
    /// OAuth provider to use for token exchange
    pub provider: OAuthProvider,
}
