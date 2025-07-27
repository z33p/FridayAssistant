use serde_derive::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::oauth_provider::OAuthProvider;

/// Request to refresh an OAuth access token
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct RefreshAccessTokenRequest {
    /// The refresh token to use for generating a new access token
    #[schema(example = "1//0GWthWtnM1YzzCgYIARAAGAwSNwF-L9IrqcH...")]
    pub refresh_token: String,
    /// OAuth provider that issued the refresh token
    pub provider: OAuthProvider,
}
