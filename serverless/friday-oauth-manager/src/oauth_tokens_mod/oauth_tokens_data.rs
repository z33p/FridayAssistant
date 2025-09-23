use sqlx::{postgres::PgPoolOptions, PgPool};
use tracing::{debug, error};

use crate::{oauth_tokens_mod::oauth_tokens::OAuthTokens, secret_manager_mod};

async fn create_database_pool() -> Result<PgPool, Box<dyn std::error::Error>> {
    let database_url = secret_manager_mod::get_database_url().await?;
    let pool = PgPoolOptions::new().connect(&database_url).await?;

    Ok(pool)
}

pub async fn insert_oauth_token(
    oauth_tokens: &OAuthTokens,
) -> Result<(), Box<dyn std::error::Error>> {
    let pool = create_database_pool().await?;

    let result = sqlx::query("CALL pr_ins_oauth_tokens($1, $2, $3)")
        .bind(&oauth_tokens.access_token)
        .bind(&oauth_tokens.refresh_token)
        .bind(oauth_tokens.expiry_date)
        .execute(&pool)
        .await;

    if let Err(e) = result {
        error!("Erro ao inserir oauth_tokens: {:?}", e);
        return Err(Box::new(e));
    }

    debug!("Registro inserido com sucesso");

    Ok(())
}

pub async fn update_oauth_token_by_refresh_token(
    oauth_tokens: &OAuthTokens,
) -> Result<(), Box<dyn std::error::Error>> {
    let pool = create_database_pool().await?;

    sqlx::query("CALL pr_upd_oauth_tokens_by_refresh_token($1, $2, $3)")
        .bind(&oauth_tokens.access_token)
        .bind(&oauth_tokens.refresh_token)
        .bind(oauth_tokens.expiry_date)
        .execute(&pool)
        .await?;

    debug!("Registro atualizado com sucesso");

    Ok(())
}

pub async fn fn_get_first_oauth_tokens_by_last_expiry_date(
) -> Result<Option<OAuthTokens>, Box<dyn std::error::Error>> {
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
