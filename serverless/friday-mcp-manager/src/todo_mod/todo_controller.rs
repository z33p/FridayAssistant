use serde_json::{Value, json};

use super::task::{CreateTaskRequest, UpdateTaskRequest};
use super::todo_list::{CreateTodoListRequest, DeleteTodoListRequest, UpdateTodoListRequest};
use super::todo_logic::{self, TodoListClient};
use crate::mcp_protocol::{CallToolRequest, CallToolResponse, Tool, ToolContent, ToolInputSchema};

pub async fn handle_todo_tool(
    client: &TodoListClient,
    request: CallToolRequest,
) -> CallToolResponse {
    match request.name.as_str() {
        // Todo List operations
        "create_todo_list" => handle_create_todo_list(client, request.arguments).await,
        "list_todo_lists" => handle_list_todo_lists(client).await,
        "get_todo_list" => handle_get_todo_list(client, request.arguments).await,
        "update_todo_list" => handle_update_todo_list(client, request.arguments).await,
        "delete_todo_list" => handle_delete_todo_list(client, request.arguments).await,

        // Task operations
        "get_all_tasks" => handle_get_all_tasks(client, request.arguments).await,
        "get_task" => handle_get_task(client, request.arguments).await,
        "create_task" => handle_create_task(client, request.arguments).await,
        "update_task" => handle_update_task(client, request.arguments).await,
        "delete_task" => handle_delete_task(client, request.arguments).await,

        _ => CallToolResponse {
            content: vec![ToolContent {
                content_type: "text".to_string(),
                text: format!("Unknown tool: {}", request.name),
            }],
            is_error: true,
        },
    }
}

/// Get list of available todo tools (lists and tasks)
pub fn get_todo_tools() -> Vec<Tool> {
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
        // Task tools
        Tool {
            name: "get_all_tasks".to_string(),
            description: "Get all tasks from a specific todo list".to_string(),
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
            name: "get_task".to_string(),
            description: "Get a specific task by ID".to_string(),
            input_schema: ToolInputSchema {
                schema_type: "object".to_string(),
                properties: json!({
                    "list_id": {
                        "type": "string",
                        "description": "The ID of the todo list"
                    },
                    "task_id": {
                        "type": "string",
                        "description": "The ID of the task"
                    }
                }),
                required: Some(vec!["list_id".to_string(), "task_id".to_string()]),
            },
        },
        Tool {
            name: "create_task".to_string(),
            description: "Create a new task in a todo list".to_string(),
            input_schema: ToolInputSchema {
                schema_type: "object".to_string(),
                properties: json!({
                    "list_id": {
                        "type": "string",
                        "description": "The ID of the todo list"
                    },
                    "title": {
                        "type": "string",
                        "description": "The title of the task"
                    },
                    "body": {
                        "type": "string",
                        "description": "The description/body of the task"
                    },
                    "importance": {
                        "type": "string",
                        "description": "The importance level (low, normal, high)"
                    },
                    "is_reminder_on": {
                        "type": "boolean",
                        "description": "Whether reminder is enabled"
                    },
                    "due_date_time": {
                        "type": "integer",
                        "description": "Due date as Unix timestamp"
                    }
                }),
                required: Some(vec!["list_id".to_string(), "title".to_string()]),
            },
        },
        Tool {
            name: "update_task".to_string(),
            description: "Update an existing task".to_string(),
            input_schema: ToolInputSchema {
                schema_type: "object".to_string(),
                properties: json!({
                    "list_id": {
                        "type": "string",
                        "description": "The ID of the todo list"
                    },
                    "task_id": {
                        "type": "string",
                        "description": "The ID of the task to update"
                    },
                    "title": {
                        "type": "string",
                        "description": "New title of the task"
                    },
                    "body": {
                        "type": "string",
                        "description": "New description/body of the task"
                    },
                    "status": {
                        "type": "string",
                        "description": "New status (notStarted, inProgress, completed, waitingOnOthers, deferred)"
                    },
                    "importance": {
                        "type": "string",
                        "description": "New importance level (low, normal, high)"
                    },
                    "is_reminder_on": {
                        "type": "boolean",
                        "description": "Whether reminder is enabled"
                    },
                    "due_date_time": {
                        "type": "integer",
                        "description": "New due date as Unix timestamp"
                    }
                }),
                required: Some(vec!["list_id".to_string(), "task_id".to_string()]),
            },
        },
        Tool {
            name: "delete_task".to_string(),
            description: "Delete a task from a todo list".to_string(),
            input_schema: ToolInputSchema {
                schema_type: "object".to_string(),
                properties: json!({
                    "list_id": {
                        "type": "string",
                        "description": "The ID of the todo list"
                    },
                    "task_id": {
                        "type": "string",
                        "description": "The ID of the task to delete"
                    }
                }),
                required: Some(vec!["list_id".to_string(), "task_id".to_string()]),
            },
        },
    ]
}

async fn handle_create_todo_list(
    client: &TodoListClient,
    arguments: Option<Value>,
) -> CallToolResponse {
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

// Task handlers
async fn handle_get_all_tasks(
    client: &TodoListClient,
    arguments: Option<Value>,
) -> CallToolResponse {
    match arguments {
        Some(args) => {
            if let Some(list_id) = args.get("list_id").and_then(|v| v.as_str()) {
                let response = todo_logic::get_all_tasks(client, list_id.to_string()).await;
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
                text: "Missing arguments for get_all_tasks".to_string(),
            }],
            is_error: true,
        },
    }
}

async fn handle_get_task(client: &TodoListClient, arguments: Option<Value>) -> CallToolResponse {
    match arguments {
        Some(args) => {
            let list_id = args.get("list_id").and_then(|v| v.as_str());
            let task_id = args.get("task_id").and_then(|v| v.as_str());

            if let (Some(list_id), Some(task_id)) = (list_id, task_id) {
                let response =
                    todo_logic::get_task(client, list_id.to_string(), task_id.to_string()).await;
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
                        text: "Missing or invalid 'list_id' or 'task_id' argument".to_string(),
                    }],
                    is_error: true,
                }
            }
        }
        None => CallToolResponse {
            content: vec![ToolContent {
                content_type: "text".to_string(),
                text: "Missing arguments for get_task".to_string(),
            }],
            is_error: true,
        },
    }
}

async fn handle_create_task(client: &TodoListClient, arguments: Option<Value>) -> CallToolResponse {
    match arguments {
        Some(args) => {
            // Extract list_id from arguments
            if let Some(list_id) = args.get("list_id").and_then(|v| v.as_str()) {
                let list_id_str = list_id.to_string();
                let mut args_copy = args.clone();
                args_copy
                    .as_object_mut()
                    .unwrap()
                    .insert("list_id".to_string(), json!(list_id_str));

                match serde_json::from_value::<CreateTaskRequest>(args_copy) {
                    Ok(request) => {
                        let response = todo_logic::create_task(client, request).await;
                        if response.success {
                            CallToolResponse {
                                content: vec![ToolContent {
                                    content_type: "text".to_string(),
                                    text: serde_json::to_string_pretty(&response.data)
                                        .unwrap_or_default(),
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
                }
            } else {
                CallToolResponse {
                    content: vec![ToolContent {
                        content_type: "text".to_string(),
                        text: "Missing 'list_id' argument".to_string(),
                    }],
                    is_error: true,
                }
            }
        }
        None => CallToolResponse {
            content: vec![ToolContent {
                content_type: "text".to_string(),
                text: "Missing arguments for create_task".to_string(),
            }],
            is_error: true,
        },
    }
}

async fn handle_update_task(client: &TodoListClient, arguments: Option<Value>) -> CallToolResponse {
    match arguments {
        Some(args) => {
            let list_id = args.get("list_id").and_then(|v| v.as_str());
            let task_id = args.get("task_id").and_then(|v| v.as_str());

            if let (Some(list_id), Some(task_id)) = (list_id, task_id) {
                // Create a copy and add required fields
                let list_id_str = list_id.to_string();
                let task_id_str = task_id.to_string();
                let mut args_copy = args.clone();
                args_copy
                    .as_object_mut()
                    .unwrap()
                    .insert("list_id".to_string(), json!(list_id_str));
                args_copy
                    .as_object_mut()
                    .unwrap()
                    .insert("id".to_string(), json!(task_id_str));

                match serde_json::from_value::<UpdateTaskRequest>(args_copy) {
                    Ok(request) => {
                        let response = todo_logic::update_task(client, request).await;
                        if response.success {
                            CallToolResponse {
                                content: vec![ToolContent {
                                    content_type: "text".to_string(),
                                    text: serde_json::to_string_pretty(&response.data)
                                        .unwrap_or_default(),
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
                }
            } else {
                CallToolResponse {
                    content: vec![ToolContent {
                        content_type: "text".to_string(),
                        text: "Missing 'list_id' or 'task_id' argument".to_string(),
                    }],
                    is_error: true,
                }
            }
        }
        None => CallToolResponse {
            content: vec![ToolContent {
                content_type: "text".to_string(),
                text: "Missing arguments for update_task".to_string(),
            }],
            is_error: true,
        },
    }
}

async fn handle_delete_task(client: &TodoListClient, arguments: Option<Value>) -> CallToolResponse {
    match arguments {
        Some(args) => {
            let list_id = args.get("list_id").and_then(|v| v.as_str());
            let task_id = args.get("task_id").and_then(|v| v.as_str());

            if let (Some(list_id), Some(task_id)) = (list_id, task_id) {
                let response =
                    todo_logic::delete_task(client, list_id.to_string(), task_id.to_string()).await;
                if response.success {
                    CallToolResponse {
                        content: vec![ToolContent {
                            content_type: "text".to_string(),
                            text: "Task deleted successfully".to_string(),
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
                        text: "Missing 'list_id' or 'task_id' argument".to_string(),
                    }],
                    is_error: true,
                }
            }
        }
        None => CallToolResponse {
            content: vec![ToolContent {
                content_type: "text".to_string(),
                text: "Missing arguments for delete_task".to_string(),
            }],
            is_error: true,
        },
    }
}

async fn handle_get_todo_list(
    client: &TodoListClient,
    arguments: Option<Value>,
) -> CallToolResponse {
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

async fn handle_update_todo_list(
    client: &TodoListClient,
    arguments: Option<Value>,
) -> CallToolResponse {
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

async fn handle_delete_todo_list(
    client: &TodoListClient,
    arguments: Option<Value>,
) -> CallToolResponse {
    match arguments {
        Some(args) => {
            if let Some(id) = args.get("id").and_then(|v| v.as_str()) {
                let request = DeleteTodoListRequest { id: id.to_string() };
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
