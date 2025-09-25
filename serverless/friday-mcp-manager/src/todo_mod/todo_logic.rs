use reqwest;
use tracing::{error, info, warn};

use super::task::{CreateTaskRequest, Task, TaskResponse, TasksResponse, UpdateTaskRequest};
use super::todo_list::{
    CreateTodoListRequest, DeleteTodoListRequest, TodoList, TodoListResponse, TodoListsResponse,
    UpdateTodoListRequest,
};

use crate::business_response::BusinessResponse;

#[derive(Debug, Clone)]
pub struct TodoListClient {
    client: reqwest::Client,
    base_url: String,
}

impl TodoListClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url,
        }
    }

    fn get_api_url(&self, path: &str) -> String {
        format!("{}/api/friday-todo-manager{}", self.base_url, path)
    }
}

pub async fn create_todo_list(
    client: &TodoListClient,
    request: CreateTodoListRequest,
) -> BusinessResponse<TodoListResponse> {
    let url = client.get_api_url("/lists");

    match client.client.post(&url).json(&request).send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<BusinessResponse<TodoList>>().await {
                    Ok(api_response) => {
                        if api_response.success {
                            if let Some(todo_list) = api_response.data {
                                BusinessResponse::success(TodoListResponse { todo_list })
                            } else {
                                BusinessResponse::error("No data in API response".to_string())
                            }
                        } else {
                            BusinessResponse::error(api_response.errors.join(", "))
                        }
                    }
                    Err(e) => BusinessResponse::error(format!("Failed to parse response: {}", e)),
                }
            } else {
                BusinessResponse::error(format!(
                    "API request failed with status: {}",
                    response.status()
                ))
            }
        }
        Err(e) => BusinessResponse::error(format!("Network error: {}", e)),
    }
}

pub async fn get_todo_lists(client: &TodoListClient) -> BusinessResponse<TodoListsResponse> {
    let url = client.get_api_url("/lists");
    info!("Making request to: {}", url);

    match client.client.get(&url).send().await {
        Ok(response) => {
            info!("Received response with status: {}", response.status());
            if response.status().is_success() {
                info!("Response successful, parsing JSON...");
                match response.json::<BusinessResponse<Vec<TodoList>>>().await {
                    Ok(api_response) => {
                        if api_response.success {
                            if let Some(todo_lists) = api_response.data {
                                let total = todo_lists.len();
                                BusinessResponse::success(TodoListsResponse { todo_lists, total })
                            } else {
                                BusinessResponse::error("No data in API response".to_string())
                            }
                        } else {
                            BusinessResponse::error(api_response.errors.join(", "))
                        }
                    }
                    Err(e) => BusinessResponse::error(format!("Failed to parse response: {}", e)),
                }
            } else {
                BusinessResponse::error(format!(
                    "API request failed with status: {}",
                    response.status()
                ))
            }
        }
        Err(e) => BusinessResponse::error(format!("Network error: {}", e)),
    }
}

pub async fn get_todo_list(
    client: &TodoListClient,
    list_id: String,
) -> BusinessResponse<TodoListResponse> {
    let url = client.get_api_url(&format!("/lists/{}", list_id));

    match client.client.get(&url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<BusinessResponse<TodoList>>().await {
                    Ok(api_response) => {
                        if api_response.success {
                            if let Some(todo_list) = api_response.data {
                                BusinessResponse::success(TodoListResponse { todo_list })
                            } else {
                                BusinessResponse::error("No data in API response".to_string())
                            }
                        } else {
                            BusinessResponse::error(api_response.errors.join(", "))
                        }
                    }
                    Err(e) => BusinessResponse::error(format!("Failed to parse response: {}", e)),
                }
            } else if response.status() == 404 {
                BusinessResponse::error(format!("Todo list with id {} not found", list_id))
            } else {
                BusinessResponse::error(format!(
                    "API request failed with status: {}",
                    response.status()
                ))
            }
        }
        Err(e) => BusinessResponse::error(format!("Network error: {}", e)),
    }
}

pub async fn update_todo_list(
    client: &TodoListClient,
    request: UpdateTodoListRequest,
) -> BusinessResponse<TodoListResponse> {
    let url = client.get_api_url("/lists");

    match client.client.put(&url).json(&request).send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<BusinessResponse<TodoList>>().await {
                    Ok(api_response) => {
                        if api_response.success {
                            if let Some(todo_list) = api_response.data {
                                BusinessResponse::success(TodoListResponse { todo_list })
                            } else {
                                BusinessResponse::error("No data in API response".to_string())
                            }
                        } else {
                            BusinessResponse::error(api_response.errors.join(", "))
                        }
                    }
                    Err(e) => BusinessResponse::error(format!("Failed to parse response: {}", e)),
                }
            } else if response.status() == 404 {
                BusinessResponse::error(format!("Todo list with id {} not found", request.id))
            } else {
                BusinessResponse::error(format!(
                    "API request failed with status: {}",
                    response.status()
                ))
            }
        }
        Err(e) => BusinessResponse::error(format!("Network error: {}", e)),
    }
}

pub async fn delete_todo_list(
    client: &TodoListClient,
    request: DeleteTodoListRequest,
) -> BusinessResponse<()> {
    let url = client.get_api_url("/lists");

    match client.client.delete(&url).json(&request).send().await {
        Ok(response) => {
            if response.status().is_success() {
                BusinessResponse::success_empty()
            } else if response.status() == 404 {
                BusinessResponse::error(format!("Todo list with id {} not found", request.id))
            } else {
                BusinessResponse::error(format!(
                    "API request failed with status: {}",
                    response.status()
                ))
            }
        }
        Err(e) => BusinessResponse::error(format!("Network error: {}", e)),
    }
}

/// Get all tasks from a specific todo list
pub async fn get_all_tasks(
    client: &TodoListClient,
    list_id: String,
) -> BusinessResponse<TasksResponse> {
    let url = client.get_api_url(&format!("/lists/{}/tasks", list_id));

    match client.client.get(&url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<BusinessResponse<Vec<Task>>>().await {
                    Ok(api_response) => {
                        if api_response.success {
                            if let Some(tasks) = api_response.data {
                                let total = tasks.len();
                                BusinessResponse::success(TasksResponse { tasks, total })
                            } else {
                                BusinessResponse::success(TasksResponse {
                                    tasks: vec![],
                                    total: 0,
                                })
                            }
                        } else {
                            BusinessResponse::error(api_response.errors.join(", "))
                        }
                    }
                    Err(e) => {
                        BusinessResponse::error(format!("Failed to parse API response: {}", e))
                    }
                }
            } else {
                BusinessResponse::error(format!(
                    "API request failed with status: {}",
                    response.status()
                ))
            }
        }
        Err(e) => BusinessResponse::error(format!("Network error: {}", e)),
    }
}

/// Get a specific task by ID
pub async fn get_task(
    client: &TodoListClient,
    list_id: String,
    task_id: String,
) -> BusinessResponse<TaskResponse> {
    let url = client.get_api_url(&format!("/lists/{}/tasks/{}", list_id, task_id));

    match client.client.get(&url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<BusinessResponse<Task>>().await {
                    Ok(api_response) => {
                        if api_response.success {
                            if let Some(task) = api_response.data {
                                BusinessResponse::success(TaskResponse { task })
                            } else {
                                BusinessResponse::error("No data in API response".to_string())
                            }
                        } else {
                            BusinessResponse::error(api_response.errors.join(", "))
                        }
                    }
                    Err(e) => {
                        BusinessResponse::error(format!("Failed to parse API response: {}", e))
                    }
                }
            } else if response.status() == 404 {
                BusinessResponse::error(format!("Task with id {} not found", task_id))
            } else {
                BusinessResponse::error(format!(
                    "API request failed with status: {}",
                    response.status()
                ))
            }
        }
        Err(e) => BusinessResponse::error(format!("Network error: {}", e)),
    }
}

/// Create a new task
pub async fn create_task(
    client: &TodoListClient,
    request: CreateTaskRequest,
) -> BusinessResponse<TaskResponse> {
    let url = client.get_api_url(&format!("/lists/{}/tasks", request.list_id));

    match client.client.post(&url).json(&request).send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<BusinessResponse<Task>>().await {
                    Ok(api_response) => {
                        if api_response.success {
                            if let Some(task) = api_response.data {
                                BusinessResponse::success(TaskResponse { task })
                            } else {
                                BusinessResponse::error("No data in API response".to_string())
                            }
                        } else {
                            BusinessResponse::error(api_response.errors.join(", "))
                        }
                    }
                    Err(e) => {
                        BusinessResponse::error(format!("Failed to parse API response: {}", e))
                    }
                }
            } else {
                BusinessResponse::error(format!(
                    "API request failed with status: {}",
                    response.status()
                ))
            }
        }
        Err(e) => BusinessResponse::error(format!("Network error: {}", e)),
    }
}

/// Update an existing task
pub async fn update_task(
    client: &TodoListClient,
    request: UpdateTaskRequest,
) -> BusinessResponse<TaskResponse> {
    let url = client.get_api_url(&format!("/lists/{}/tasks/{}", request.list_id, request.id));

    match client.client.patch(&url).json(&request).send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<BusinessResponse<Task>>().await {
                    Ok(api_response) => {
                        if api_response.success {
                            if let Some(task) = api_response.data {
                                BusinessResponse::success(TaskResponse { task })
                            } else {
                                BusinessResponse::error("No data in API response".to_string())
                            }
                        } else {
                            BusinessResponse::error(api_response.errors.join(", "))
                        }
                    }
                    Err(e) => {
                        BusinessResponse::error(format!("Failed to parse API response: {}", e))
                    }
                }
            } else if response.status() == 404 {
                BusinessResponse::error(format!("Task with id {} not found", request.id))
            } else {
                BusinessResponse::error(format!(
                    "API request failed with status: {}",
                    response.status()
                ))
            }
        }
        Err(e) => BusinessResponse::error(format!("Network error: {}", e)),
    }
}

/// Delete a task
pub async fn delete_task(
    client: &TodoListClient,
    list_id: String,
    task_id: String,
) -> BusinessResponse<String> {
    let url = client.get_api_url(&format!("/lists/{}/tasks/{}", list_id, task_id));

    match client.client.delete(&url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                BusinessResponse::success(format!("Task {} deleted successfully", task_id))
            } else if response.status() == 404 {
                BusinessResponse::error(format!("Task with id {} not found", task_id))
            } else {
                BusinessResponse::error(format!(
                    "API request failed with status: {}",
                    response.status()
                ))
            }
        }
        Err(e) => BusinessResponse::error(format!("Network error: {}", e)),
    }
}
