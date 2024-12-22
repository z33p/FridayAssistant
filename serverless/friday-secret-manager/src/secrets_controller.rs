use actix_web::{delete, get, post, put, web, Responder};

extern crate dotenv;

use serde::Deserialize;

use crate::secrets_mod::{secret::Secret, secrets_logic};
#[get("/api/secrets/get_secret_value/{key}")]
pub async fn get_secret_value(key: web::Path<String>) -> impl Responder {
    let result = secrets_logic::get_secret_value(&key).await.unwrap();
    actix_web::web::Json(result)

}

#[get("/api/secrets/get_all_secrets")]
pub async fn get_all_secrets() -> impl Responder {
    let secrets = secrets_logic::get_all_secrets().await.unwrap();
    actix_web::web::Json(secrets)
}

#[post("/api/secrets/insert_secret")]
pub async fn insert_secret(secret: actix_web::web::Json<Secret>) -> impl Responder {
    let result = secrets_logic::insert_secret(secret.into_inner())
        .await
        .unwrap();
    actix_web::web::Json(result)
}

#[put("/api/secrets/update_secret")]
pub async fn update_secret(secret: actix_web::web::Json<Secret>) -> impl Responder {
    let result = secrets_logic::update_secret(secret.into_inner())
        .await
        .unwrap();
    actix_web::web::Json(result)
}

#[delete("/api/secrets/delete_secret")]
pub async fn delete_secret(secret: actix_web::web::Json<DeleteSecretRequest>) -> impl Responder {
    let result = secrets_logic::delete_secret(&secret.key).await.unwrap();
    actix_web::web::Json(result)
}

#[post("/api/secrets/refresh_secrets")]
pub async fn refresh_secrets() -> impl Responder {
    let result = secrets_logic::refresh_secrets().await.unwrap();
    actix_web::web::Json(result)

}

#[derive(Deserialize)]
struct DeleteSecretRequest {
    key: String,
}
