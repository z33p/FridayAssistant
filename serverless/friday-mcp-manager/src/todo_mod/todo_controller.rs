use serde_json::{Value, json};
use tracing::instrument;

use super::todo_list::{CreateTodoRequest, UpdateTodoRequest};
use super::todo_logic::{self, TodoClient};
use crate::mcp_protocol::{CallToolRequest, CallToolResponse, Tool, ToolContent, ToolInputSchema};

/// LAW 6: Controller instruments SHALL use endpoint names without prefix
#[instrument(name = "handle_todo_tool")]
pub fn handle_todo_tool(client: &TodoClient, request: CallToolRequest) -> CallToolResponse {
    match request.name.as_str() {
        "create_todo" => handle_create_todo(client, request.arguments),
        "list_todos" => handle_list_todos(client),
        "get_todo" => handle_get_todo(client, request.arguments),
        "update_todo" => handle_update_todo(client, request.arguments),
        "delete_todo" => handle_delete_todo(client, request.arguments),
        _ => CallToolResponse {
            content: vec![ToolContent {
                content_type: "text".to_string(),
                text: format!("Unknown tool: {}", request.name),
            }],
            is_error: true,
        },
    }
}

/// Get list of available todo tools
pub fn get_todo_tools() -> Vec<Tool> {
    vec![
        Tool {
            name: "create_todo".to_string(),
            description: "Create a new todo item".to_string(),
            input_schema: ToolInputSchema {
                schema_type: "object".to_string(),
                properties: json!({
                    "title": {
                        "type": "string",
                        "description": "The title of the todo item"
                    },
                    "description": {
                        "type": "string",
                        "description": "Optional description of the todo item"
                    }
                }),
                required: Some(vec!["title".to_string()]),
            },
        },
        Tool {
            name: "list_todos".to_string(),
            description: "List all todo items".to_string(),
            input_schema: ToolInputSchema {
                schema_type: "object".to_string(),
                properties: json!({}),
                required: None,
            },
        },
        Tool {
            name: "get_todo".to_string(),
            description: "Get a specific todo item by ID".to_string(),
            input_schema: ToolInputSchema {
                schema_type: "object".to_string(),
                properties: json!({
                    "id": {
                        "type": "string",
                        "description": "The ID of the todo item"
                    }
                }),
                required: Some(vec!["id".to_string()]),
            },
        },
        Tool {
            name: "update_todo".to_string(),
            description: "Update an existing todo item".to_string(),
            input_schema: ToolInputSchema {
                schema_type: "object".to_string(),
                properties: json!({
                    "id": {
                        "type": "string",
                        "description": "The ID of the todo item"
                    },
                    "title": {
                        "type": "string",
                        "description": "Optional new title"
                    },
                    "description": {
                        "type": "string",
                        "description": "Optional new description"
                    },
                    "completed": {
                        "type": "boolean",
                        "description": "Optional completion status"
                    }
                }),
                required: Some(vec!["id".to_string()]),
            },
        },
        Tool {
            name: "delete_todo".to_string(),
            description: "Delete a todo item by ID".to_string(),
            input_schema: ToolInputSchema {
                schema_type: "object".to_string(),
                properties: json!({
                    "id": {
                        "type": "string",
                        "description": "The ID of the todo item to delete"
                    }
                }),
                required: Some(vec!["id".to_string()]),
            },
        },
    ]
}

fn handle_create_todo(client: &TodoClient, arguments: Option<Value>) -> CallToolResponse {
    match arguments {
        Some(args) => match serde_json::from_value::<CreateTodoRequest>(args) {
            Ok(request) => {
                let response = todo_logic::create_todo(client, request);
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
                text: "Missing arguments for create_todo".to_string(),
            }],
            is_error: true,
        },
    }
}

fn handle_list_todos(client: &TodoClient) -> CallToolResponse {
    let response = todo_logic::get_todos(client);
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

fn handle_get_todo(client: &TodoClient, arguments: Option<Value>) -> CallToolResponse {
    match arguments {
        Some(args) => {
            if let Some(id) = args.get("id").and_then(|v| v.as_str()) {
                let response = todo_logic::get_todo(client, id.to_string());
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
                        text: "Missing or invalid 'id' argument".to_string(),
                    }],
                    is_error: true,
                }
            }
        }
        None => CallToolResponse {
            content: vec![ToolContent {
                content_type: "text".to_string(),
                text: "Missing arguments for get_todo".to_string(),
            }],
            is_error: true,
        },
    }
}

fn handle_update_todo(client: &TodoClient, arguments: Option<Value>) -> CallToolResponse {
    match arguments {
        Some(args) => match serde_json::from_value::<UpdateTodoRequest>(args) {
            Ok(request) => {
                let response = todo_logic::update_todo(client, request);
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
                text: "Missing arguments for update_todo".to_string(),
            }],
            is_error: true,
        },
    }
}

fn handle_delete_todo(client: &TodoClient, arguments: Option<Value>) -> CallToolResponse {
    match arguments {
        Some(args) => {
            if let Some(id) = args.get("id").and_then(|v| v.as_str()) {
                let response = todo_logic::delete_todo(client, id.to_string());
                if response.success {
                    CallToolResponse {
                        content: vec![ToolContent {
                            content_type: "text".to_string(),
                            text: "Todo deleted successfully".to_string(),
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
                text: "Missing arguments for delete_todo".to_string(),
            }],
            is_error: true,
        },
    }
}
