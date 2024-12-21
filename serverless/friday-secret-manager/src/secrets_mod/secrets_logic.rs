use crate::friday_redis_client;

use super::{secret::Secret, secrets_data};

pub async fn get_secret_value(key: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
    if let Some(value) = friday_redis_client::get_value(key).await? {
        return Ok(Some(value));
    }

    if let Some(value) = secrets_data::get_secret_value(key).await? {
        // Set the value in Redis
        friday_redis_client::set_value(key, &value).await?;
        return Ok(Some(value));
    }

    Ok(None)
}

pub async fn get_all_secrets() -> Result<Vec<Option<Secret>>, Box<dyn std::error::Error>> {
    secrets_data::get_all_secrets().await
}

pub async fn insert_secret(secret: Secret) -> Result<(), Box<dyn std::error::Error>> {
    // Insert in Redis
    friday_redis_client::set_value(&secret.key, &secret.value).await?;

    // Insert in database
    secrets_data::insert_secret(secret).await
}

pub async fn update_secret(secret: Secret) -> Result<(), Box<dyn std::error::Error>> {
    // Update in Redis
    friday_redis_client::set_value(&secret.key, &secret.value).await?;

    // Update in database
    secrets_data::update_secret(secret).await
}

pub async fn delete_secret(key: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Delete from Redis
    friday_redis_client::delete_key_value(key).await?;

    // Delete from database
    secrets_data::delete_secret(key).await
}

pub async fn refresh_secrets() -> Result<(), Box<dyn std::error::Error>> {
    let secrets_list = get_all_secrets().await?;
    for secret_option in secrets_list {
        if let Some(secret) = secret_option {
            friday_redis_client::set_value(&secret.key, &secret.value).await?;
        }
    }

    Ok(())
}
