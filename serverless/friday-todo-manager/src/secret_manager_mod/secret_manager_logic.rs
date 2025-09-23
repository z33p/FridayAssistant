use super::secret_manager_api::SecretManagerApi;
use once_cell::sync::OnceCell;

static DATABASE_URL: OnceCell<String> = OnceCell::new();

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
