use chrono::{DateTime, Utc};
use chrono_tz::America::Sao_Paulo;
use serde_derive::{Deserialize, Serialize};
use sqlx::Row;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct OAuthTokens {
    pub id_oauth_tokens: Option<Uuid>,
    pub access_token: String,
    pub refresh_token: String,
    pub expiry_date: DateTime<Utc>,
}

impl OAuthTokens {
    pub fn get_expiry_date_str(&self) -> String {
        self.expiry_date
            .with_timezone(&Sao_Paulo)
            .to_rfc3339_opts(chrono::SecondsFormat::Secs, false)
    }

    pub(crate) fn parse_expiry_date(
        expiry_date_str: &str,
    ) -> Result<DateTime<Utc>, chrono::ParseError> {
        DateTime::parse_from_rfc3339(expiry_date_str).map(|dt| dt.with_timezone(&Utc))
    }

    pub fn from_row(
        row: &sqlx_postgres::PgRow,
    ) -> Result<Option<OAuthTokens>, Box<dyn std::error::Error>> {
        let id_oauth_tokens: Uuid = match row.try_get::<String, _>("id_oauth_tokens") {
            Ok(id_oauth_tokens_str) => {
                Uuid::parse_str(&id_oauth_tokens_str).expect("Failed to parse uuid id_oauth_tokens")
            }
            Err(e) => return Err(Box::new(e)),
        };

        let access_token: String = row
            .try_get("access_token")
            .expect("Failed to parse access_token");

        let refresh_token: String = row
            .try_get("refresh_token")
            .expect("Failed to parse refresh_token");

        let expiry_date = match row.try_get::<String, _>("expiry_date") {
            Ok(expiry_date_str) => OAuthTokens::parse_expiry_date(expiry_date_str.as_str())
                .expect("Failed to parse expiry_date"), // ou seu método de conversão
            Err(e) => return Err(Box::new(e)),
        };

        let oauth_token = OAuthTokens {
            id_oauth_tokens: Some(id_oauth_tokens),
            access_token,
            refresh_token,
            expiry_date,
        };

        Ok(Some(oauth_token))
    }
}
