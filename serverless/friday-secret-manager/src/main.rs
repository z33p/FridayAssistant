use actix_web::{delete, get, post, put, App, HttpServer, Responder};

mod load_env;
mod secrets_data;

extern crate dotenv;

use dotenv::dotenv;
use load_env::{load_env_variables, EnvVariables};
use once_cell::sync::Lazy;
use secrets_data::Secret;
use serde::Deserialize;
use tracing::Level;

static ENV_CONFIG: Lazy<EnvVariables> = Lazy::new(|| load_env_variables());

#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[get("/get_all_secrets")]
async fn get_all_secrets() -> impl Responder {
    let secrets = secrets_data::get_all_secrets().await.unwrap();
    actix_web::web::Json(secrets)
}

#[post("/insert_secret")]
async fn insert_secret(secret: actix_web::web::Json<Secret>) -> impl Responder {
    let result = secrets_data::insert_secret(secret.into_inner()).await.unwrap();
    actix_web::web::Json(result)
}

#[put("/update_secret")]
async fn update_secret(secret: actix_web::web::Json<Secret>) -> impl Responder {
    let result = secrets_data::update_secret(secret.into_inner()).await.unwrap();
    actix_web::web::Json(result)
}

#[delete("/delete_secret")]
async fn delete_secret(secret: actix_web::web::Json<DeleteSecretRequest>) -> impl Responder {
    let result = secrets_data::delete_secret(&secret.key).await.unwrap();
    actix_web::web::Json(result)
}

#[derive(Deserialize)]
struct DeleteSecretRequest {
    key: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    logging_init();

    HttpServer::new(|| App::new()
        .service(index)
        .service(get_all_secrets)
        .service(insert_secret)
        .service(update_secret)
        .service(delete_secret))
        .bind(("127.0.0.1", 8080))?
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