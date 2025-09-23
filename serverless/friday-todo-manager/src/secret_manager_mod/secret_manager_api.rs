use reqwest;
use tracing::{error, info};

use super::secret_response::SecretResponse;
use crate::ENV_CONFIG;

pub struct SecretManagerApi {
    client: reqwest::Client,
    base_url: String,
}

impl SecretManagerApi {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: ENV_CONFIG.secret_manager_url.clone(),
        }
    }

    fn get_secret_url(&self, key: &str) -> String {
        format!("{}/secrets/get_secret_value/{}", self.base_url, key)
    }

    pub async fn get_secret_value(
        &self,
        key: &str,
    ) -> Result<Option<String>, Box<dyn std::error::Error>> {
        info!("Fetching secret value for key: {}", key);

        let url = self.get_secret_url(key);

        match self.client.get(&url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<SecretResponse>().await {
                        Ok(secret_response) => {
                            if secret_response.success {
                                info!("Successfully retrieved secret for key: {}", key);
                                Ok(secret_response.data)
                            } else {
                                error!(
                                    "Secret manager returned error for key {}: {:?}",
                                    key, secret_response.errors
                                );
                                Ok(None)
                            }
                        }
                        Err(e) => {
                            error!("Failed to parse secret manager response: {}", e);
                            Err(Box::new(e))
                        }
                    }
                } else if response.status() == 404 {
                    info!("Secret not found for key: {}", key);
                    Ok(None)
                } else {
                    error!(
                        "Secret manager request failed with status: {}",
                        response.status()
                    );
                    Err(format!(
                        "Secret manager request failed with status: {}",
                        response.status()
                    )
                    .into())
                }
            }
            Err(e) => {
                error!("Network error when contacting secret manager: {}", e);
                Err(Box::new(e))
            }
        }
    }
}
