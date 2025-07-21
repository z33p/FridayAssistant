use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Row};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::oauth_provider::OAuthProvider;

/// OAuth tokens with expiry information
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct OAuthTokens {
    /// Unique identifier for the token record
    pub id_oauth_tokens: Option<Uuid>,
    /// OAuth access token
    #[schema(example = "ya29.a0AfH6SMC...")]
    pub access_token: String,
    /// OAuth refresh token
    #[schema(example = "1//0GWthWtnM1YzzCgYIARAAGAwSNwF-L9IrqcH...")]
    pub refresh_token: String,
    /// Token expiry date and time in UTC
    pub expiry_date: DateTime<Utc>,
    /// OAuth provider that issued the tokens
    pub provider: OAuthProvider,
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

        let provider_str: String = row
            .try_get("provider")
            .unwrap_or_else(|_| "google".to_string()); // Default to google for backward compatibility

        let provider = match provider_str.as_str() {
            "microsoft" => OAuthProvider::Microsoft,
            _ => OAuthProvider::Google,
        };

        let oauth_token = OAuthTokens {
            id_oauth_tokens: Some(id_oauth_tokens),
            access_token,
            refresh_token,
            expiry_date,
            provider,
        };

        Ok(Some(oauth_token))
    }
}
