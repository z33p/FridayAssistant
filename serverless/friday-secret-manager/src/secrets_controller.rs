use actix_web::{delete, get, post, put, web, Responder};

extern crate dotenv;

use serde::Deserialize;
use utoipa::ToSchema;

use crate::secrets_mod::{secret::Secret, secrets_logic};

#[utoipa::path(
    get,
    path = "/api/secrets/get_secret_value/{key}",
    tag = "Secrets",
    params(
        ("key", Path, description = "Secret key to retrieve")
    ),
    responses(
        (status = 200, description = "Secret retrieved successfully", body = Secret),
        (status = 404, description = "Secret not found")
    )
)]
#[get("/api/secrets/get_secret_value/{key}")]
pub async fn get_secret_value(key: web::Path<String>) -> impl Responder {
    let result = secrets_logic::get_secret_value(&key).await.unwrap();
    actix_web::web::Json(result)
}

#[utoipa::path(
    get,
    path = "/api/secrets/get_all_secrets",
    tag = "Secrets",
    responses(
        (status = 200, description = "All secrets retrieved successfully", body = Vec<Secret>),
    )
)]
#[get("/api/secrets/get_all_secrets")]
pub async fn get_all_secrets() -> impl Responder {
    let secrets = secrets_logic::get_all_secrets().await.unwrap();
    actix_web::web::Json(secrets)
}

#[utoipa::path(
    post,
    path = "/api/secrets/insert_secret",
    tag = "Secrets",
    request_body = Secret,
    responses(
        (status = 200, description = "Secret inserted successfully", body = String),
        (status = 400, description = "Invalid secret data")
    )
)]
#[post("/api/secrets/insert_secret")]
pub async fn insert_secret(secret: actix_web::web::Json<Secret>) -> impl Responder {
    let result = secrets_logic::insert_secret(secret.into_inner())
        .await
        .unwrap();
    actix_web::web::Json(result)
}

#[utoipa::path(
    put,
    path = "/api/secrets/update_secret",
    tag = "Secrets",
    request_body = Secret,
    responses(
        (status = 200, description = "Secret updated successfully", body = String),
        (status = 404, description = "Secret not found"),
        (status = 400, description = "Invalid secret data")
    )
)]
#[put("/api/secrets/update_secret")]
pub async fn update_secret(secret: actix_web::web::Json<Secret>) -> impl Responder {
    let result = secrets_logic::update_secret(secret.into_inner())
        .await
        .unwrap();
    actix_web::web::Json(result)
}

#[utoipa::path(
    delete,
    path = "/api/secrets/delete_secret",
    tag = "Secrets",
    request_body = DeleteSecretRequest,
    responses(
        (status = 200, description = "Secret deleted successfully", body = String),
        (status = 404, description = "Secret not found")
    )
)]
#[delete("/api/secrets/delete_secret")]
pub async fn delete_secret(secret: actix_web::web::Json<DeleteSecretRequest>) -> impl Responder {
    let result = secrets_logic::delete_secret(&secret.key).await.unwrap();
    actix_web::web::Json(result)
}

#[utoipa::path(
    post,
    path = "/api/secrets/refresh_secrets",
    tag = "Operations",
    responses(
        (status = 200, description = "Secrets refreshed successfully", body = String),
    )
)]
#[post("/api/secrets/refresh_secrets")]
pub async fn refresh_secrets() -> impl Responder {
    let result = secrets_logic::refresh_secrets().await.unwrap();
    actix_web::web::Json(result)
}

#[derive(Deserialize, ToSchema)]
pub struct DeleteSecretRequest {
    pub key: String,
}
