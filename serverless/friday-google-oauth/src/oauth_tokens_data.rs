use sqlx::{PgPool, postgres::PgPoolOptions};
use tracing::debug;

use crate::{tokens_getter::oauth_tokens::OAuthTokens, ENV_CONFIG};

async fn create_database_pool() -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .connect(ENV_CONFIG.database_url.as_str())
        .await?;

    Ok(pool)
}

pub async fn insert_oauth_token(oauth_tokens: &OAuthTokens) -> Result<(), sqlx::Error> {
    let pool = create_database_pool().await?;

    sqlx::query("CALL pr_ins_oauth_tokens($1, $2, $3)")
        .bind(&oauth_tokens.access_token)
        .bind(&oauth_tokens.refresh_token)
        .bind(oauth_tokens.expiry_date)
        .execute(&pool)
        .await?;

    debug!("Registro inserido com sucesso");

    Ok(())
}

pub async fn update_oauth_token(oauth_tokens: &OAuthTokens) -> Result<(), sqlx::Error> {
    let pool = create_database_pool().await?;

    sqlx::query("CALL pr_upd_oauth_tokens($1, $2, $3)")
        .bind(&oauth_tokens.access_token)
        .bind(&oauth_tokens.refresh_token)
        .bind(oauth_tokens.expiry_date)
        .execute(&pool)
        .await?;

    debug!("Registro atualizado com sucesso");

    Ok(())
}

pub async fn get_first_oauth_token_by_refresh_token() -> Result<Option<OAuthTokens>, sqlx::Error> {
    let pool = create_database_pool().await?;

    let query = "SELECT * FROM fn_get_first_oauth_tokens_by_last_expiry_date()";

    let row = sqlx::query(query).fetch_optional(&pool).await?;

    match row {
        Some(row) => {
            let oauth_tokens = OAuthTokens::from_row(&row).unwrap();
            Ok(oauth_tokens)
        }
        None => Ok(None),
    }
}
