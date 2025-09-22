use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::instrument;

use super::todo_list::{
    CreateTodoRequest, TodoItem, TodoListResponse, TodoResponse, UpdateTodoRequest,
};
use crate::business_response::Response;

/// LAW 5: Client structs SHALL end with Client
#[derive(Debug)]
pub struct TodoClient {
    todos: Arc<Mutex<HashMap<String, TodoItem>>>,
}

impl TodoClient {
    pub fn new() -> Self {
        Self {
            todos: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl Default for TodoClient {
    fn default() -> Self {
        Self::new()
    }
}

/// LAW 6: Instrumentation naming - business layer
#[instrument(name = "logic_create_todo")]
pub fn create_todo(client: &TodoClient, request: CreateTodoRequest) -> Response<TodoResponse> {
    let todo = TodoItem::create_new(request.title, request.description);
    let todo_id = todo.id.clone();

    match client.todos.lock() {
        Ok(mut todos) => {
            todos.insert(todo_id, todo.clone());
            Response::success(TodoResponse { todo })
        }
        Err(_) => Response::error("Failed to acquire lock on todos".to_string()),
    }
}

#[instrument(name = "logic_get_todos")]
pub fn get_todos(client: &TodoClient) -> Response<TodoListResponse> {
    match client.todos.lock() {
        Ok(todos) => {
            let todo_list: Vec<TodoItem> = todos.values().cloned().collect();
            let total = todo_list.len();
            Response::success(TodoListResponse {
                todos: todo_list,
                total,
            })
        }
        Err(_) => Response::error("Failed to acquire lock on todos".to_string()),
    }
}

#[instrument(name = "logic_get_todo")]
pub fn get_todo(client: &TodoClient, todo_id: String) -> Response<TodoResponse> {
    match client.todos.lock() {
        Ok(todos) => match todos.get(&todo_id) {
            Some(todo) => Response::success(TodoResponse { todo: todo.clone() }),
            None => Response::error(format!("Todo with id {} not found", todo_id)),
        },
        Err(_) => Response::error("Failed to acquire lock on todos".to_string()),
    }
}

#[instrument(name = "logic_update_todo")]
pub fn update_todo(client: &TodoClient, request: UpdateTodoRequest) -> Response<TodoResponse> {
    match client.todos.lock() {
        Ok(mut todos) => match todos.get_mut(&request.id) {
            Some(todo) => {
                todo.update_fields(request.title, request.description, request.completed);
                Response::success(TodoResponse { todo: todo.clone() })
            }
            None => Response::error(format!("Todo with id {} not found", request.id)),
        },
        Err(_) => Response::error("Failed to acquire lock on todos".to_string()),
    }
}

#[instrument(name = "logic_delete_todo")]
pub fn delete_todo(client: &TodoClient, todo_id: String) -> Response<()> {
    match client.todos.lock() {
        Ok(mut todos) => match todos.remove(&todo_id) {
            Some(_) => Response::success_empty(),
            None => Response::error(format!("Todo with id {} not found", todo_id)),
        },
        Err(_) => Response::error("Failed to acquire lock on todos".to_string()),
    }
}
