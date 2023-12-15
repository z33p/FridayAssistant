use oauth2::{CsrfToken, Scope};
use serde_json::json;
use tracing::debug;

use crate::{get_oauth_client, lambda_handler::lambda_oauth_response::LambdaOAuthResponse};

pub async fn generate_oauth_url() -> Result<LambdaOAuthResponse, Box<dyn std::error::Error>> {
    let client = get_oauth_client()?;

    let (auth_url, _) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/gmail.send".to_string(),
        ))
        .url();

    debug!("{}", auth_url.to_string());

    Ok(LambdaOAuthResponse {
        status_code: 200,
        data: json!({ "url": auth_url.to_string() }),
    })
}
