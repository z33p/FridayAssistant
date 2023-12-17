use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Row};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct OAuthTokens {
    pub id_oauth_tokens: Option<Uuid>,
    pub access_token: String,
    pub refresh_token: String,
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

        let oauth_token = OAuthTokens {
            id_oauth_tokens: Some(id_oauth_tokens),
            access_token,
            refresh_token,
            expiry_date,
        };

        Ok(Some(oauth_token))
    }
}
