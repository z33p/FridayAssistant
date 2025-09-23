use reqwest::Client;
use tracing::{error, info, instrument};

use super::oauth_response::AccessTokenResponse;
use crate::ENV_CONFIG;

#[derive(Debug)]
pub struct OAuthApi {
    client: Client,
    base_url: String,
}

impl OAuthApi {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: ENV_CONFIG.oauth_manager_url.clone(),
        }
    }


    pub async fn generate_access_token(&self) -> Result<String, Box<dyn std::error::Error>> {
        info!("OAuth layer: Generating Microsoft access token");

        let url = format!("{}/api/oauth/generate-access-token", self.base_url);
        info!("OAuth layer: Making GET request to: {}", url);

        let response = match self
            .client
            .get(&url)
            .header("Content-Type", "application/json")
            .send()
            .await
        {
            Ok(response) => response,
            Err(e) => {
                error!("OAuth layer: HTTP request failed: {}", e);
                return Err(format!("Failed to connect to OAuth manager: {}", e).into());
            }
        };

        let status = response.status();
        info!("OAuth layer: Received response with status: {}", status);

        if response.status().is_success() {
            match response.json::<AccessTokenResponse>().await {
                Ok(oauth_response) => {
                    if oauth_response.success {
                        if let Some(access_token) = oauth_response.data {
                            info!("OAuth layer: Successfully generated access token");
                            Ok(access_token)
                        } else {
                            error!("OAuth layer: OAuth response contained no access token");
                            Err("No access token in OAuth response".into())
                        }
                    } else {
                        let error_msg = oauth_response.errors.join(", ");
                        error!("OAuth layer: OAuth service error: {}", error_msg);
                        Err(format!("OAuth service error: {}", error_msg).into())
                    }
                }
                Err(e) => {
                    error!("OAuth layer: Failed to parse OAuth response: {}", e);
                    Err(format!("Failed to parse OAuth response: {}", e).into())
                }
            }
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            error!("OAuth layer: HTTP error ({}): {}", status, error_text);
            Err(format!("OAuth service HTTP error: {} - {}", status, error_text).into())
        }
    }
}
