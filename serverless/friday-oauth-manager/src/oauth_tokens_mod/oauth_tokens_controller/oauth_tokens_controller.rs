use actix_web::{get, post, Responder};
use serde_json::json;
use tracing::{error, info};
use utoipa;

use crate::{
    api_response::ApiResponse, 
    oauth_provider::OAuthProvider, 
    oauth_tokens_mod::{
        oauth_tokens_logic,
        oauth_tokens_controller::{
            refresh_access_token_request::RefreshAccessTokenRequest,
            get_oauth_tokens_request::GetOAuthTokensRequest
        }
    }
};

extern crate dotenv;

/// Generate a new access token (Microsoft only - legacy endpoint)
///
/// This endpoint generates an access token for Microsoft OAuth.
/// This is a legacy endpoint maintained for backward compatibility.
#[utoipa::path(
    get,
    path = "/api/oauth/generate-access-token",
    responses(
        (status = 200, description = "Access token generated successfully", body = ApiResponse),
        (status = 500, description = "Internal server error", body = ApiResponse)
    ),
    tag = "OAuth"
)]
#[get("/api/oauth/generate-access-token")]
pub async fn generate_access_token() -> impl Responder {
    info!("Gerando access token");

    match oauth_tokens_logic::generate_access_token().await {
        Ok(response) => actix_web::web::Json(response),
        Err(e) => {
            error!("Erro ao gerar access token: {}", e);
            actix_web::web::Json(ApiResponse {
                status_code: 500,
                data: serde_json::Value::Null,
                errors: Some(vec![format!("Erro interno: {}", e)]),
            })
        }
    }
}

/// Refresh an OAuth access token
///
/// This endpoint refreshes an expired access token using a refresh token.
/// Supports both Google and Microsoft OAuth providers.
#[utoipa::path(
    post,
    path = "/api/oauth/refresh-access-token",
    request_body = RefreshAccessTokenRequest,
    responses(
        (status = 200, description = "Access token refreshed successfully", body = ApiResponse),
        (status = 500, description = "Internal server error", body = ApiResponse)
    ),
    tag = "OAuth"
)]
#[post("/api/oauth/refresh-access-token")]
pub async fn refresh_access_token(
    request: actix_web::web::Json<RefreshAccessTokenRequest>,
) -> impl Responder {
    info!("Fazendo refresh do access token");

    match oauth_tokens_logic::refresh_access_token(request.into_inner()).await {
        Ok(response) => actix_web::web::Json(response),
        Err(e) => {
            error!("Erro ao fazer refresh do access token: {}", e);
            actix_web::web::Json(ApiResponse {
                status_code: 500,
                data: serde_json::Value::Null,
                errors: Some(vec![format!("Erro interno: {}", e)]),
            })
        }
    }
}

/// Generate OAuth authorization URL (Microsoft)
///
/// This endpoint generates an OAuth authorization URL for Microsoft.
/// Users should be redirected to this URL to start the OAuth flow.
#[utoipa::path(
    get,
    path = "/api/oauth/url",
    responses(
        (status = 200, description = "OAuth URL generated successfully", body = ApiResponse),
        (status = 500, description = "Internal server error", body = ApiResponse)
    ),
    tag = "OAuth URLs"
)]
#[get("/api/oauth/url")]
pub async fn generate_oauth_url_endpoint() -> impl Responder {
    info!("Gerando URL OAuth padrão (Microsoft)");

    match oauth_tokens_logic::generate_oauth_url().await {
        Ok(response) => actix_web::web::Json(response),
        Err(e) => {
            error!("Erro ao gerar URL OAuth: {}", e);
            actix_web::web::Json(ApiResponse {
                status_code: 500,
                data: serde_json::Value::Null,
                errors: Some(vec![format!("Erro interno: {}", e)]),
            })
        }
    }
}

/// Generate Google OAuth authorization URL
///
/// This endpoint generates an OAuth authorization URL specifically for Google.
/// Users should be redirected to this URL to start the Google OAuth flow.
#[utoipa::path(
    get,
    path = "/api/oauth/url/google",
    responses(
        (status = 200, description = "Google OAuth URL generated successfully", body = ApiResponse),
        (status = 500, description = "Internal server error", body = ApiResponse)
    ),
    tag = "OAuth URLs"
)]
#[get("/api/oauth/url/google")]
pub async fn generate_google_oauth_url() -> impl Responder {
    info!("Gerando URL OAuth do Google");

    match oauth_tokens_logic::generate_oauth_url_for_provider(OAuthProvider::Google).await {
        Ok(response) => actix_web::web::Json(response),
        Err(e) => {
            error!("Erro ao gerar URL OAuth do Google: {}", e);
            actix_web::web::Json(ApiResponse {
                status_code: 500,
                data: serde_json::Value::Null,
                errors: Some(vec![format!("Erro interno: {}", e)]),
            })
        }
    }
}

/// Generate Microsoft OAuth authorization URL
///
/// This endpoint generates an OAuth authorization URL specifically for Microsoft.
/// Users should be redirected to this URL to start the Microsoft OAuth flow.
#[utoipa::path(
    get,
    path = "/api/oauth/url/microsoft",
    responses(
        (status = 200, description = "Microsoft OAuth URL generated successfully", body = ApiResponse),
        (status = 500, description = "Internal server error", body = ApiResponse)
    ),
    tag = "OAuth URLs"
)]
#[get("/api/oauth/url/microsoft")]
pub async fn generate_microsoft_oauth_url() -> impl Responder {
    info!("Gerando URL OAuth da Microsoft");

    match oauth_tokens_logic::generate_oauth_url_for_provider(OAuthProvider::Microsoft).await {
        Ok(response) => actix_web::web::Json(response),
        Err(e) => {
            error!("Erro ao gerar URL OAuth da Microsoft: {}", e);
            actix_web::web::Json(ApiResponse {
                status_code: 500,
                data: serde_json::Value::Null,
                errors: Some(vec![format!("Erro interno: {}", e)]),
            })
        }
    }
}

/// Exchange authorization code for OAuth tokens
///
/// This endpoint exchanges an authorization code (received from OAuth callback)
/// for access and refresh tokens. Supports both Google and Microsoft OAuth providers.
#[utoipa::path(
    post,
    path = "/api/oauth/tokens",
    request_body = GetOAuthTokensRequest,
    responses(
        (status = 200, description = "Tokens exchanged successfully", body = ApiResponse),
        (status = 500, description = "Internal server error", body = ApiResponse)
    ),
    tag = "OAuth"
)]
#[post("/api/oauth/tokens")]
pub async fn get_oauth_tokens(
    request: actix_web::web::Json<GetOAuthTokensRequest>,
) -> impl Responder {
    info!("Fazendo exchange de tokens OAuth");

    match oauth_tokens_logic::get_oauth_tokens(request.into_inner()).await {
        Ok(response) => actix_web::web::Json(response),
        Err(e) => {
            error!("Erro ao fazer exchange de tokens OAuth: {}", e);
            actix_web::web::Json(ApiResponse {
                status_code: 500,
                data: serde_json::Value::Null,
                errors: Some(vec![format!("Erro interno: {}", e)]),
            })
        }
    }
}

/// Health check endpoint
///
/// This endpoint provides basic health status information for the OAuth service.
#[utoipa::path(
    get,
    path = "/api/health",
    responses(
        (status = 200, description = "Service is healthy", body = serde_json::Value)
    ),
    tag = "Health"
)]
#[get("/api/health")]
pub async fn health_check() -> impl Responder {
    actix_web::web::Json(json!({
        "status": "healthy",
        "service": "friday-oauth-api",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}
