use sqlx::{postgres::PgPoolOptions, PgPool, Row};

use crate::ENV_CONFIG;

use super::secret::Secret;

async fn create_database_pool() -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .connect(ENV_CONFIG.database_url.as_str())
        .await?;

    Ok(pool)
}

pub async fn get_secret_value(key: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let query = "SELECT fn_get_secret_value($1)";

    let pool = create_database_pool().await?;
    let row = sqlx::query(query).bind(key).fetch_optional(&pool).await?;

    match row {
        Some(row) => {
            let value: String = row.get(0);
            Ok(Some(value))
        }
        None => Ok(None),
    }
}

pub async fn get_all_secrets() -> Result<Vec<Option<Secret>>, Box<dyn std::error::Error>> {
    let query = "SELECT * FROM fn_get_all_secrets()";

    let pool = create_database_pool().await?;
    let rows = sqlx::query(query).fetch_all(&pool).await?;

    let secret_list: Result<Vec<Option<Secret>>, Box<dyn std::error::Error>> =
        rows.iter().map(|row| Secret::from_row(row)).collect();

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
    sqlx::query(query).bind(key).execute(&pool).await?;

    Ok(())
}
