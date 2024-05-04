
use super::{secret::Secret, secrets_data};


pub async fn get_all_secrets() -> Result<Vec<Option<Secret>>, Box<dyn std::error::Error>> {
    secrets_data::get_all_secrets().await
}

pub async fn insert_secret(secret: Secret) -> Result<(), Box<dyn std::error::Error>> {
    secrets_data::insert_secret(secret).await
}

pub async fn update_secret(secret: Secret) -> Result<(), Box<dyn std::error::Error>> {
    secrets_data::update_secret(secret).await
}

pub async fn delete_secret(key: &str) -> Result<(), Box<dyn std::error::Error>> {
    secrets_data::delete_secret(key).await
}

