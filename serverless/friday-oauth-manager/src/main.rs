mod business_response;
mod load_env;
mod oauth_provider;
mod oauth_tokens_mod;
mod openapi;
mod secret_manager_mod;

extern crate dotenv;

use actix_web::{get, App, HttpServer, Responder};
use dotenv::dotenv;
use load_env::{load_env_variables, EnvVariables};
use oauth2::basic::BasicClient;
use oauth_provider::{OAuthProvider, OAuthProviderFactory};
use once_cell::sync::Lazy;
use std::error::Error;
use tracing::{info, Level};

use crate::oauth_tokens_mod::oauth_tokens_controller;

static ENV_CONFIG: Lazy<EnvVariables> = Lazy::new(|| load_env_variables());

#[get("/")]
async fn index() -> impl Responder {
    "Friday OAuth API - OK"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    logging_init();

    info!("Iniciando servidor OAuth HTTP API na porta 5000");

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(oauth_tokens_controller::generate_access_token)
            .service(oauth_tokens_controller::refresh_access_token)
            .service(oauth_tokens_controller::generate_oauth_url_endpoint)
            .service(oauth_tokens_controller::generate_google_oauth_url)
            .service(oauth_tokens_controller::generate_microsoft_oauth_url)
            .service(oauth_tokens_controller::get_oauth_tokens)
            .service(oauth_tokens_controller::health_check)
            .service(openapi::swagger_config())
    })
    .workers(4)
    .bind(("0.0.0.0", 5000))?
    .run()
    .await
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

pub async fn get_oauth_client(provider: OAuthProvider) -> Result<BasicClient, Box<dyn Error>> {
    let (client_id, secret_value) = secret_manager_mod::get_oauth_credentials().await?;

    let oauth_provider = OAuthProviderFactory::create_provider(
        &provider,
        client_id,
        secret_value,
        "http://localhost:5000/callback".to_string(),
    );

    oauth_provider.create_client()
}
