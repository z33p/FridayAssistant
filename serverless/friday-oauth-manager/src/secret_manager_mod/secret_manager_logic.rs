use super::secret_manager_api::SecretManagerApi;
use once_cell::sync::OnceCell;

static OAUTH_CREDENTIALS: OnceCell<(String, String)> = OnceCell::new();
static DATABASE_URL: OnceCell<String> = OnceCell::new();

pub async fn get_oauth_credentials() -> Result<(String, String), Box<dyn std::error::Error>> {
    if let Some(credentials) = OAUTH_CREDENTIALS.get() {
        return Ok(credentials.clone());
    }

    let client = SecretManagerApi::new();

    let client_id = client
        .get_secret_value("OAUTH_CLIENT_ID")
        .await?
        .ok_or("OAUTH_CLIENT_ID not found in secret manager")?;

    let secret_value = client
        .get_secret_value("OAUTH_SECRET_VALUE")
        .await?
        .ok_or("OAUTH_SECRET_VALUE not found in secret manager")?;

    OAUTH_CREDENTIALS.set((client_id.clone(), secret_value.clone())).ok();
    Ok((client_id, secret_value))
}

pub async fn get_database_url() -> Result<String, Box<dyn std::error::Error>> {
    if let Some(url) = DATABASE_URL.get() {
        return Ok(url.clone());
    }

    let client = SecretManagerApi::new();

    let database_url = client
        .get_secret_value("ConnectionStrings:Postgres")
        .await?
        .ok_or("ConnectionStrings:Postgres not found in secret manager")?;

    DATABASE_URL.set(database_url.clone()).ok();
    Ok(database_url)
}
