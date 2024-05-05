use std::error::Error;

use redis::AsyncCommands;

use crate::ENV_CONFIG;
use dashmap::DashMap;
use once_cell::sync::Lazy;

/// URL format: `{redis|rediss}://[<username>][:<password>@]<hostname>[:port][/<db>]`
///
/// - Basic: `redis://127.0.0.1:6379`
/// - Username & Password: `redis://user:password@127.0.0.1:6379`
/// - Password only: `redis://:password@127.0.0.1:6379`
/// - Specifying DB: `redis://127.0.0.1:6379/0`
/// - Enabling TLS: `rediss://127.0.0.1:6379`
/// - Enabling Insecure TLS: `rediss://127.0.0.1:6379/#insecure`
fn get_redis_client() -> Result<redis::Client, Box<dyn Error>> {
    let client = redis::Client::open(ENV_CONFIG.redis_url.clone())?;
    Ok(client)
}

static CACHE: Lazy<DashMap<String, String>> = Lazy::new(DashMap::new);

pub async fn get_value_in_memory(key: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
    if let Some(value) = CACHE.get(key) {
        return Ok(Some(value.clone()));
    }

    // If not cached, fetch the value from Redis
    let client = get_redis_client()?;
    let mut conn = client.get_multiplexed_async_connection().await?;
    let result: Option<String> = conn.get(key).await?;

    // Cache the value in memory if it exists
    if let Some(value) = result.as_ref() {
        CACHE.insert(key.to_string(), value.to_string());
    }

    Ok(result)
}

pub async fn get_value(key: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let client = get_redis_client()?;
    let mut conn = client.get_multiplexed_async_connection().await?;

    let result: Option<String> = conn.get(key).await?;

    Ok(result)
}

pub async fn set_value(key: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = get_redis_client()?;
    let mut conn = client.get_multiplexed_async_connection().await?;

    conn.set(key, value).await?;

    Ok(())
}

pub async fn delete_key_value(key: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = get_redis_client()?;
    let mut conn = client.get_multiplexed_async_connection().await?;
    conn.del(key).await?;

    Ok(())
}
