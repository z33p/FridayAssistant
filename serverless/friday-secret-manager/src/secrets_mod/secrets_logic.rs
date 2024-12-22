use crate::business_response;
use crate::friday_redis_client;

use super::{secret::Secret, secrets_data};

pub async fn get_secret_value(
    key: &str,
) -> Result<business_response::Response<String>, Box<dyn std::error::Error>> {
    if let Some(value) = friday_redis_client::get_value(key).await? {
        return Ok(business_response::Response::new(true, Some(value), vec![]));
    }

    if let Some(value) = secrets_data::get_secret_value(key).await? {
        // Set the value in Redis
        friday_redis_client::set_value(key, &value).await?;
        return Ok(business_response::Response::new(true, Some(value), vec![]));
    }

    Ok(business_response::Response::new(
        false,
        None,
        vec!["Secret not found".to_string()],
    ))
}

pub async fn get_all_secrets(
) -> Result<business_response::Response<Vec<Option<Secret>>>, Box<dyn std::error::Error>> {
    let secrets = secrets_data::get_all_secrets().await?;
    Ok(business_response::Response::new(
        true,
        Some(secrets),
        vec![],
    ))
}

pub async fn insert_secret(
    secret: Secret,
) -> Result<business_response::Response<String>, Box<dyn std::error::Error>> {
    // Insert in Redis
    friday_redis_client::set_value(&secret.key, &secret.value).await?;

    // Insert in database
    secrets_data::insert_secret(secret).await?;
    Ok(business_response::Response::new(
        true,
        Some("Secret inserted successfully".to_string()),
        vec![],
    ))
}

pub async fn update_secret(
    secret: Secret,
) -> Result<business_response::Response<String>, Box<dyn std::error::Error>> {
    // Update in Redis
    friday_redis_client::set_value(&secret.key, &secret.value).await?;

    // Update in database
    secrets_data::update_secret(secret).await?;
    Ok(business_response::Response::new(
        true,
        Some("Secret updated successfully".to_string()),
        vec![],
    ))
}

pub async fn delete_secret(
    key: &str,
) -> Result<business_response::Response<String>, Box<dyn std::error::Error>> {
    // Delete from Redis
    friday_redis_client::delete_key_value(key).await?;

    // Delete from database
    secrets_data::delete_secret(key).await?;
    Ok(business_response::Response::new(
        true,
        Some("Secret deleted successfully".to_string()),
        vec![],
    ))
}

pub async fn refresh_secrets() -> Result<business_response::Response<String>, Box<dyn std::error::Error>>
{
    let secrets_list = get_all_secrets().await?.data.unwrap_or_default();
    for secret_option in secrets_list {
        if let Some(secret) = secret_option {
            friday_redis_client::set_value(&secret.key, &secret.value).await?;
        }
    }

    Ok(business_response::Response::new(
        true,
        Some("Secrets refreshed successfully".to_string()),
        vec![],
    ))
}
