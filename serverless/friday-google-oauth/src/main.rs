mod generate_oauth_url;
mod tokens_getter;
mod load_env;
mod lambda_handler;

extern crate dotenv;

use std::error::Error;

use dotenv::dotenv;
use lambda_runtime::service_fn;
use load_env::{load_env_variables, EnvVariables};
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use once_cell::sync::Lazy;
use tracing::{Level, error};

static ENV_CONFIG: Lazy<EnvVariables> = Lazy::new(|| load_env_variables());

#[tokio::main]
async fn main() {
    dotenv().ok();
    logging_init();

    let func = service_fn(lambda_handler::handler);
    let res = lambda_runtime::run(func).await;

    if res.is_ok() {
        return;
    }

    let err = res.err().unwrap();
    error!("Error: {}", err.to_string());
}

fn logging_init() {
    let log_level = if ENV_CONFIG.is_prod {
        Level::INFO
    } else {
        Level::DEBUG
    };

    tracing_subscriber::fmt()
        .with_max_level(log_level)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();
}

pub fn get_oauth_client() -> Result<
    oauth2::Client<
        oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType>,
        oauth2::StandardTokenResponse<oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType>,
        oauth2::basic::BasicTokenType,
        oauth2::StandardTokenIntrospectionResponse<
            oauth2::EmptyExtraTokenFields,
            oauth2::basic::BasicTokenType,
        >,
        oauth2::StandardRevocableToken,
        oauth2::StandardErrorResponse<oauth2::RevocationErrorResponseType>,
    >,
    Box<dyn Error>,
> {
    let client_id = ClientId::new(ENV_CONFIG.oauth_client_id.to_string());
    let client_secret = ClientSecret::new(ENV_CONFIG.oauth_client_secret.to_string());
    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())?;
    let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v4/token".to_string())?;
    let redirect_url = RedirectUrl::new("http://localhost/".to_string())?;
    let client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
        .set_redirect_uri(redirect_url);

    Ok(client)
}
