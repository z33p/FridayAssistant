use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{error, info, instrument};

#[derive(Debug, Deserialize, Serialize)]
pub struct AccessTokenResponse {
    pub status_code: i32,
    pub data: Option<String>, // The access token
    pub errors: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct OAuthClient {
    client: Client,
    oauth_base_url: String,
}

impl OAuthClient {
    pub fn new() -> Self {
        // You can configure this URL via environment variable or config
        let oauth_base_url = std::env::var("OAUTH_MANAGER_URL")
            .unwrap_or_else(|_| "http://localhost:5000".to_string());

        Self {
            client: Client::new(),
            oauth_base_url,
        }
    }

    #[instrument(name = "oauth_generate_access_token")]
    pub async fn generate_access_token(&self) -> Result<String, Box<dyn std::error::Error>> {
        info!("OAuth layer: Generating Microsoft access token");

        let url = format!("{}/api/oauth/generate-access-token", self.oauth_base_url);
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
                    if oauth_response.status_code == 200 {
                        if let Some(access_token) = oauth_response.data {
                            info!("OAuth layer: Successfully generated access token");
                            Ok(access_token)
                        } else {
                            error!("OAuth layer: OAuth response contained no access token");
                            Err("No access token in OAuth response".into())
                        }
                    } else {
                        let error_msg = oauth_response
                            .errors
                            .unwrap_or_else(|| vec!["Unknown OAuth error".to_string()])
                            .join(", ");
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
