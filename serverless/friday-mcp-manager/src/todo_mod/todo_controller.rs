use serde_json::{Value, json};
use tracing::instrument;

use super::todo_list::{CreateTodoListRequest, UpdateTodoListRequest, DeleteTodoListRequest};
use super::todo_logic::{self, TodoListClient};
use crate::mcp_protocol::{CallToolRequest, CallToolResponse, Tool, ToolContent, ToolInputSchema};

/// LAW 6: Controller instruments SHALL use endpoint names without prefix
#[instrument(name = "handle_todo_list_tool")]
pub async fn handle_todo_list_tool(client: &TodoListClient, request: CallToolRequest) -> CallToolResponse {
    match request.name.as_str() {
        "create_todo_list" => handle_create_todo_list(client, request.arguments).await,
        "list_todo_lists" => handle_list_todo_lists(client).await,
        "get_todo_list" => handle_get_todo_list(client, request.arguments).await,
        "update_todo_list" => handle_update_todo_list(client, request.arguments).await,
        "delete_todo_list" => handle_delete_todo_list(client, request.arguments).await,
        _ => CallToolResponse {
            content: vec![ToolContent {
                content_type: "text".to_string(),
                text: format!("Unknown tool: {}", request.name),
            }],
            is_error: true,
        },
    }
}

/// Get list of available todo list tools
pub fn get_todo_list_tools() -> Vec<Tool> {
    vec![
        Tool {
            name: "create_todo_list".to_string(),
            description: "Create a new todo list".to_string(),
            input_schema: ToolInputSchema {
                schema_type: "object".to_string(),
                properties: json!({
                    "display_name": {
                        "type": "string",
                        "description": "The display name of the todo list"
                    }
                }),
                required: Some(vec!["display_name".to_string()]),
            },
        },
        Tool {
            name: "list_todo_lists".to_string(),
            description: "List all todo lists".to_string(),
            input_schema: ToolInputSchema {
                schema_type: "object".to_string(),
                properties: json!({}),
                required: None,
            },
        },
        Tool {
            name: "get_todo_list".to_string(),
            description: "Get a specific todo list by ID".to_string(),
            input_schema: ToolInputSchema {
                schema_type: "object".to_string(),
                properties: json!({
                    "list_id": {
                        "type": "string",
                        "description": "The ID of the todo list"
                    }
                }),
                required: Some(vec!["list_id".to_string()]),
            },
        },
        Tool {
            name: "update_todo_list".to_string(),
            description: "Update an existing todo list".to_string(),
            input_schema: ToolInputSchema {
                schema_type: "object".to_string(),
                properties: json!({
                    "id": {
                        "type": "string",
                        "description": "The ID of the todo list"
                    },
                    "display_name": {
                        "type": "string",
                        "description": "New display name for the todo list"
                    }
                }),
                required: Some(vec!["id".to_string(), "display_name".to_string()]),
            },
        },
        Tool {
            name: "delete_todo_list".to_string(),
            description: "Delete a todo list by ID".to_string(),
            input_schema: ToolInputSchema {
                schema_type: "object".to_string(),
                properties: json!({
                    "id": {
                        "type": "string",
                        "description": "The ID of the todo list to delete"
                    }
                }),
                required: Some(vec!["id".to_string()]),
            },
        },
    ]
}

async fn handle_create_todo_list(client: &TodoListClient, arguments: Option<Value>) -> CallToolResponse {
    match arguments {
        Some(args) => match serde_json::from_value::<CreateTodoListRequest>(args) {
            Ok(request) => {
                let response = todo_logic::create_todo_list(client, request).await;
                if response.success {
                    CallToolResponse {
                        content: vec![ToolContent {
                            content_type: "text".to_string(),
                            text: serde_json::to_string_pretty(&response.data).unwrap_or_default(),
                        }],
                        is_error: false,
                    }
                } else {
                    CallToolResponse {
                        content: vec![ToolContent {
                            content_type: "text".to_string(),
                            text: response.errors.join(", "),
                        }],
                        is_error: true,
                    }
                }
            }
            Err(e) => CallToolResponse {
                content: vec![ToolContent {
                    content_type: "text".to_string(),
                    text: format!("Invalid arguments: {}", e),
                }],
                is_error: true,
            },
        },
        None => CallToolResponse {
            content: vec![ToolContent {
                content_type: "text".to_string(),
                text: "Missing arguments for create_todo_list".to_string(),
            }],
            is_error: true,
        },
    }
}

async fn handle_list_todo_lists(client: &TodoListClient) -> CallToolResponse {
    let response = todo_logic::get_todo_lists(client).await;
    if response.success {
        CallToolResponse {
            content: vec![ToolContent {
                content_type: "text".to_string(),
                text: serde_json::to_string_pretty(&response.data).unwrap_or_default(),
            }],
            is_error: false,
        }
    } else {
        CallToolResponse {
            content: vec![ToolContent {
                content_type: "text".to_string(),
                text: response.errors.join(", "),
            }],
            is_error: true,
        }
    }
}

async fn handle_get_todo_list(client: &TodoListClient, arguments: Option<Value>) -> CallToolResponse {
    match arguments {
        Some(args) => {
            if let Some(list_id) = args.get("list_id").and_then(|v| v.as_str()) {
                let response = todo_logic::get_todo_list(client, list_id.to_string()).await;
                if response.success {
                    CallToolResponse {
                        content: vec![ToolContent {
                            content_type: "text".to_string(),
                            text: serde_json::to_string_pretty(&response.data).unwrap_or_default(),
                        }],
                        is_error: false,
                    }
                } else {
                    CallToolResponse {
                        content: vec![ToolContent {
                            content_type: "text".to_string(),
                            text: response.errors.join(", "),
                        }],
                        is_error: true,
                    }
                }
            } else {
                CallToolResponse {
                    content: vec![ToolContent {
                        content_type: "text".to_string(),
                        text: "Missing or invalid 'list_id' argument".to_string(),
                    }],
                    is_error: true,
                }
            }
        }
        None => CallToolResponse {
            content: vec![ToolContent {
                content_type: "text".to_string(),
                text: "Missing arguments for get_todo_list".to_string(),
            }],
            is_error: true,
        },
    }
}

async fn handle_update_todo_list(client: &TodoListClient, arguments: Option<Value>) -> CallToolResponse {
    match arguments {
        Some(args) => match serde_json::from_value::<UpdateTodoListRequest>(args) {
            Ok(request) => {
                let response = todo_logic::update_todo_list(client, request).await;
                if response.success {
                    CallToolResponse {
                        content: vec![ToolContent {
                            content_type: "text".to_string(),
                            text: serde_json::to_string_pretty(&response.data).unwrap_or_default(),
                        }],
                        is_error: false,
                    }
                } else {
                    CallToolResponse {
                        content: vec![ToolContent {
                            content_type: "text".to_string(),
                            text: response.errors.join(", "),
                        }],
                        is_error: true,
                    }
                }
            }
            Err(e) => CallToolResponse {
                content: vec![ToolContent {
                    content_type: "text".to_string(),
                    text: format!("Invalid arguments: {}", e),
                }],
                is_error: true,
            },
        },
        None => CallToolResponse {
            content: vec![ToolContent {
                content_type: "text".to_string(),
                text: "Missing arguments for update_todo_list".to_string(),
            }],
            is_error: true,
        },
    }
}

async fn handle_delete_todo_list(client: &TodoListClient, arguments: Option<Value>) -> CallToolResponse {
    match arguments {
        Some(args) => {
            if let Some(id) = args.get("id").and_then(|v| v.as_str()) {
                let request = DeleteTodoListRequest {
                    id: id.to_string(),
                };
                let response = todo_logic::delete_todo_list(client, request).await;
                if response.success {
                    CallToolResponse {
                        content: vec![ToolContent {
                            content_type: "text".to_string(),
                            text: "Todo list deleted successfully".to_string(),
                        }],
                        is_error: false,
                    }
                } else {
                    CallToolResponse {
                        content: vec![ToolContent {
                            content_type: "text".to_string(),
                            text: response.errors.join(", "),
                        }],
                        is_error: true,
                    }
                }
            } else {
                CallToolResponse {
                    content: vec![ToolContent {
                        content_type: "text".to_string(),
                        text: "Missing or invalid 'id' argument".to_string(),
                    }],
                    is_error: true,
                }
            }
        }
        None => CallToolResponse {
            content: vec![ToolContent {
                content_type: "text".to_string(),
                text: "Missing arguments for delete_todo_list".to_string(),
            }],
            is_error: true,
        },
    }
}
