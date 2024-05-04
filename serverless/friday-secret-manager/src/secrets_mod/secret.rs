use serde_derive::{Deserialize, Serialize};
use sqlx::Row;
use sqlx::postgres::PgRow;

#[derive(Debug, Deserialize, Serialize)]
pub struct Secret {
    pub key: String,
    pub value: String,
}

impl Secret {
    pub fn from_row(row: &PgRow) -> Result<Option<Secret>, Box<dyn std::error::Error>> {
        let secret_key: String = row
            .try_get("key")
            .expect("Failed to parse secret.key");

        let secret_value: String = row
            .try_get("value")
            .expect("Failed to parse secret.value");

        let secret = Secret {
            key: secret_key,
            value: secret_value
        };

        Ok(Some(secret))
    }
}