mod api_response;
mod generate_oauth_url;
mod load_env;
mod oauth_controller;
mod oauth_provider;
mod oauth_tokens_data;
mod openapi;
mod tokens_getter;

extern crate dotenv;

use actix_web::{get, App, HttpServer, Responder};
use dotenv::dotenv;
use load_env::{load_env_variables, EnvVariables};
use oauth2::basic::BasicClient;
use oauth_provider::{OAuthProvider, OAuthProviderFactory};
use once_cell::sync::Lazy;
use openapi::swagger_config;
use std::error::Error;
use tracing::{info, Level};

static ENV_CONFIG: Lazy<EnvVariables> = Lazy::new(|| load_env_variables());

#[get("/")]
async fn index() -> impl Responder {
    "Friday OAuth API - OK"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    logging_init();

    info!("Iniciando servidor OAuth HTTP API na porta 3000");

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(oauth_controller::generate_access_token)
            .service(oauth_controller::refresh_access_token)
            .service(oauth_controller::generate_oauth_url_endpoint)
            .service(oauth_controller::generate_google_oauth_url)
            .service(oauth_controller::generate_microsoft_oauth_url)
            .service(oauth_controller::get_oauth_tokens)
            .service(oauth_controller::health_check)
            .service(swagger_config())
    })
    .workers(4)
    .bind(("0.0.0.0", 3000))?
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

pub fn get_oauth_client(provider: OAuthProvider) -> Result<BasicClient, Box<dyn Error>> {
    let oauth_provider = OAuthProviderFactory::create_provider(
        &provider,
        ENV_CONFIG.oauth_client_id.clone(),
        ENV_CONFIG.oauth_client_secret.clone(),
        "http://localhost:5000/callback".to_string(),
    );

    oauth_provider.create_client()
}
