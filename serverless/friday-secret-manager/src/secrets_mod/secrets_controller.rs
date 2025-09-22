use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

extern crate dotenv;

use serde::Deserialize;
use utoipa::ToSchema;

use crate::business_response;
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
    match secrets_logic::get_secret_value(&key).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            let error_response = business_response::Response::<String>::new(
                false,
                None,
                vec![format!("Failed to get secret: {}", e)],
            );
            HttpResponse::InternalServerError().json(error_response)
        }
    }
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
    match secrets_logic::get_all_secrets().await {
        Ok(secrets) => HttpResponse::Ok().json(secrets),
        Err(e) => {
            let error_response = business_response::Response::<Vec<Option<Secret>>>::new(
                false,
                None,
                vec![format!("Failed to get all secrets: {}", e)],
            );
            HttpResponse::InternalServerError().json(error_response)
        }
    }
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
    match secrets_logic::insert_secret(secret.into_inner()).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            let error_response = business_response::Response::<String>::new(
                false,
                None,
                vec![format!("Failed to insert secret: {}", e)],
            );
            HttpResponse::InternalServerError().json(error_response)
        }
    }
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
    match secrets_logic::update_secret(secret.into_inner()).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            let error_response = business_response::Response::<String>::new(
                false,
                None,
                vec![format!("Failed to update secret: {}", e)],
            );
            HttpResponse::InternalServerError().json(error_response)
        }
    }
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
    match secrets_logic::delete_secret(&secret.key).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            let error_response = business_response::Response::<String>::new(
                false,
                None,
                vec![format!("Failed to delete secret: {}", e)],
            );
            HttpResponse::InternalServerError().json(error_response)
        }
    }
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
    match secrets_logic::refresh_secrets().await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            let error_response = business_response::Response::<String>::new(
                false,
                None,
                vec![format!("Failed to refresh secrets: {}", e)],
            );
            HttpResponse::InternalServerError().json(error_response)
        }
    }
}

#[derive(Deserialize, ToSchema)]
pub struct DeleteSecretRequest {
    pub key: String,
}
