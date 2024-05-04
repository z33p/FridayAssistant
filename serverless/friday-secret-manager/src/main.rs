use actix_web::{get, App, HttpServer, Responder};
use dotenv::dotenv;
use load_env::{load_env_variables, EnvVariables};
use once_cell::sync::Lazy;
use tracing::Level;

mod load_env;
mod secrets_mod;
mod secrets_controller;

extern crate dotenv;

static ENV_CONFIG: Lazy<EnvVariables> = Lazy::new(|| load_env_variables());

#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    logging_init();

    HttpServer::new(|| App::new()
        .service(index)
        .service(secrets_controller::get_all_secrets)
        .service(secrets_controller::insert_secret)
        .service(secrets_controller::update_secret)
        .service(secrets_controller::delete_secret))
        .bind(("127.0.0.1", 5000))?
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