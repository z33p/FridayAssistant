use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Row};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::oauth_provider::OAuthProvider;

impl From<i32> for OAuthProvider {
    fn from(id: i32) -> Self {
        match id {
            1 => OAuthProvider::Microsoft,
            2 => OAuthProvider::Google,
            _ => panic!("Unknown OAuthProvider id: {}", id),
        }
    }
}

/// OAuth tokens with expiry information
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct OAuthTokens {
    /// Unique identifier for the token record
    pub id_oauth_tokens: Option<Uuid>,
    /// OAuth provider that issued the tokens
    pub id_provider: OAuthProvider,
    /// OAuth access token
    #[schema(example = "ya29.a0AfH6SMC...")]
    pub access_token: String,
    /// OAuth refresh token
    #[schema(example = "1//0GWthWtnM1YzzCgYIARAAGAwSNwF-L9IrqcH...")]
    pub refresh_token: String,
    /// Token expiry date and time in UTC
    pub expiry_date: DateTime<Utc>,
}

impl OAuthTokens {
    pub fn from_row(row: &PgRow) -> Result<Option<OAuthTokens>, Box<dyn std::error::Error>> {
        let id_oauth_tokens: Uuid = row
            .try_get("id_oauth_tokens")
            .expect("Failed to parse id_oauth_tokens");

        let access_token: String = row
            .try_get("access_token")
            .expect("Failed to parse access_token");

        let refresh_token: String = row
            .try_get("refresh_token")
            .expect("Failed to parse refresh_token");

        let expiry_date: DateTime<Utc> = row
            .try_get("expiry_date")
            .expect("Failed to parse expiry_date");

        let id_provider = row
            .try_get::<i32, _>("id_provider")
            .map(OAuthProvider::from)
            .expect("Failed to parse id_provider");

        let oauth_token = OAuthTokens {
            id_oauth_tokens: Some(id_oauth_tokens),
            access_token,
            refresh_token,
            expiry_date,
            id_provider,
        };

        Ok(Some(oauth_token))
    }
}
