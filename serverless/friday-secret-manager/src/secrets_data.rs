use serde_derive::{Deserialize, Serialize};
use sqlx::{postgres::{PgPoolOptions, PgRow}, PgPool, Row};

use crate::ENV_CONFIG;

async fn create_database_pool() -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .connect(ENV_CONFIG.database_url.as_str())
        .await?;

    Ok(pool)
}

pub async fn get_all_secrets() -> Result<Vec<Option<Secret>>, Box<dyn std::error::Error>> {
    let query = "SELECT * FROM fn_get_all_secrets()";
    
    let pool = create_database_pool().await?;
    let rows = sqlx::query(query).fetch_all(&pool).await?;

    let secret_list: Result<Vec<Option<Secret>>, Box<dyn std::error::Error>> = rows.iter().map(|row| {
        Secret::from_row(row)
    }).collect();

    secret_list
}

pub async fn insert_secret(secret: Secret) -> Result<(), Box<dyn std::error::Error>> {
    let query = "CALL pr_ins_secret($1, $2)";
    
    let pool = create_database_pool().await?;
    sqlx::query(query)
        .bind(&secret.key)
        .bind(&secret.value)
        .execute(&pool)
        .await?;
    
    Ok(())
}

pub async fn update_secret(secret: Secret) -> Result<(), Box<dyn std::error::Error>> {
    let query = "CALL pr_upd_secret($1, $2)";
    
    let pool = create_database_pool().await?;
    sqlx::query(query)
        .bind(&secret.key)
        .bind(&secret.value)
        .execute(&pool)
        .await?;
    
    Ok(())
}

pub async fn delete_secret(key: &str) -> Result<(), Box<dyn std::error::Error>> {
    let query = "CALL pr_del_secret($1)";
    
    let pool = create_database_pool().await?;
    sqlx::query(query)
        .bind(key)
        .execute(&pool)
        .await?;
    
    Ok(())
}

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
