use axum::{Router, extract::State, http::StatusCode, response::Json, routing::post};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing::{error, info, warn};

use crate::load_env::EnvVariables;
use crate::mcp_protocol::*;
use crate::todo_mod::todo_controller::{get_todo_tools, handle_todo_tool};
use crate::todo_mod::todo_logic::TodoListClient;

/// Shared application state
#[derive(Clone)]
pub struct AppState {
    pub todo_client: Arc<TodoListClient>,
}

/// Create the HTTP router for MCP endpoints
pub fn create_router(config: &EnvVariables) -> Router {
    let todo_client = Arc::new(TodoListClient::new(config.todo_api_base_url.clone()));
    let state = AppState { todo_client };

    Router::new()
        .route("/", post(handle_mcp_request))
        .layer(CorsLayer::permissive())
        .with_state(state)
}

/// Main MCP request handler
async fn handle_mcp_request(
    State(state): State<AppState>,
    Json(request): Json<JsonRpcRequest>,
) -> Result<Json<JsonRpcResponse>, StatusCode> {
    info!(
        "Received MCP request: method={}, id={:?}",
        request.method, request.id
    );

    let response = match request.method.as_str() {
        "initialize" => {
            info!("Handling initialize request");
            handle_initialize(request)
        }
        "tools/list" => {
            info!("Handling tools/list request");
            handle_list_tools(request)
        }
        "tools/call" => {
            info!("Handling tools/call request");
            handle_call_tool(state, request).await
        }
        _ => {
            warn!("Unknown method: {}", request.method);
            JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: None,
                error: Some(JsonRpcError {
                    code: -32601,
                    message: format!("Method not found: {}", request.method),
                    data: None,
                }),
            }
        }
    };

    Ok(Json(response))
}

/// Handle MCP initialize request
fn handle_initialize(request: JsonRpcRequest) -> JsonRpcResponse {
    let response = InitializeResponse {
        protocol_version: "2024-11-05".to_string(),
        capabilities: ServerCapabilities {
            tools: ToolCapabilities {
                list_changed: false,
            },
        },
        server_info: ServerInfo {
            name: "friday-todo-mcp".to_string(),
            version: "0.1.0".to_string(),
        },
    };

    JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        id: request.id,
        result: Some(serde_json::to_value(response).unwrap_or_default()),
        error: None,
    }
}

/// Handle tools/list request
fn handle_list_tools(request: JsonRpcRequest) -> JsonRpcResponse {
    let tools = get_todo_tools();
    let response = ToolsListResponse { tools };

    JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        id: request.id,
        result: Some(serde_json::to_value(response).unwrap_or_default()),
        error: None,
    }
}

/// Handle tools/call request
async fn handle_call_tool(state: AppState, request: JsonRpcRequest) -> JsonRpcResponse {
    match request.params {
        Some(params) => match serde_json::from_value::<CallToolRequest>(params) {
            Ok(call_request) => {
                let response = handle_todo_tool(&state.todo_client, call_request).await;
                JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: Some(serde_json::to_value(response).unwrap_or_default()),
                    error: None,
                }
            }
            Err(e) => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: None,
                error: Some(JsonRpcError {
                    code: -32602,
                    message: format!("Invalid params: {}", e),
                    data: None,
                }),
            },
        },
        None => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id: request.id,
            result: None,
            error: Some(JsonRpcError {
                code: -32602,
                message: "Missing params for tools/call".to_string(),
                data: None,
            }),
        },
    }
}
