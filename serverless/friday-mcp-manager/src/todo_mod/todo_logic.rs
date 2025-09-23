use reqwest;
use tracing::instrument;

use super::todo_list::{
    ApiResponse, CreateTodoListRequest, DeleteTodoListRequest, TodoList, TodoListResponse,
    TodoListsResponse, UpdateTodoListRequest,
};
use crate::business_response::Response;

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
) -> Response<TodoListResponse> {
    let url = client.get_api_url("/todo-lists");

    match client.client.post(&url).json(&request).send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<ApiResponse<TodoList>>().await {
                    Ok(api_response) => {
                        if api_response.success {
                            if let Some(todo_list) = api_response.data {
                                Response::success(TodoListResponse { todo_list })
                            } else {
                                Response::error("No data in API response".to_string())
                            }
                        } else {
                            Response::error(api_response.errors.join(", "))
                        }
                    }
                    Err(e) => Response::error(format!("Failed to parse response: {}", e)),
                }
            } else {
                Response::error(format!(
                    "API request failed with status: {}",
                    response.status()
                ))
            }
        }
        Err(e) => Response::error(format!("Network error: {}", e)),
    }
}

pub async fn get_todo_lists(client: &TodoListClient) -> Response<TodoListsResponse> {
    let url = client.get_api_url("/todo-lists");

    match client.client.get(&url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<ApiResponse<Vec<TodoList>>>().await {
                    Ok(api_response) => {
                        if api_response.success {
                            if let Some(todo_lists) = api_response.data {
                                let total = todo_lists.len();
                                Response::success(TodoListsResponse { todo_lists, total })
                            } else {
                                Response::error("No data in API response".to_string())
                            }
                        } else {
                            Response::error(api_response.errors.join(", "))
                        }
                    }
                    Err(e) => Response::error(format!("Failed to parse response: {}", e)),
                }
            } else {
                Response::error(format!(
                    "API request failed with status: {}",
                    response.status()
                ))
            }
        }
        Err(e) => Response::error(format!("Network error: {}", e)),
    }
}

pub async fn get_todo_list(client: &TodoListClient, list_id: String) -> Response<TodoListResponse> {
    let url = client.get_api_url(&format!("/todo-lists/{}", list_id));

    match client.client.get(&url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<ApiResponse<TodoList>>().await {
                    Ok(api_response) => {
                        if api_response.success {
                            if let Some(todo_list) = api_response.data {
                                Response::success(TodoListResponse { todo_list })
                            } else {
                                Response::error("No data in API response".to_string())
                            }
                        } else {
                            Response::error(api_response.errors.join(", "))
                        }
                    }
                    Err(e) => Response::error(format!("Failed to parse response: {}", e)),
                }
            } else if response.status() == 404 {
                Response::error(format!("Todo list with id {} not found", list_id))
            } else {
                Response::error(format!(
                    "API request failed with status: {}",
                    response.status()
                ))
            }
        }
        Err(e) => Response::error(format!("Network error: {}", e)),
    }
}

pub async fn update_todo_list(
    client: &TodoListClient,
    request: UpdateTodoListRequest,
) -> Response<TodoListResponse> {
    let url = client.get_api_url("/todo-lists");

    match client.client.put(&url).json(&request).send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<ApiResponse<TodoList>>().await {
                    Ok(api_response) => {
                        if api_response.success {
                            if let Some(todo_list) = api_response.data {
                                Response::success(TodoListResponse { todo_list })
                            } else {
                                Response::error("No data in API response".to_string())
                            }
                        } else {
                            Response::error(api_response.errors.join(", "))
                        }
                    }
                    Err(e) => Response::error(format!("Failed to parse response: {}", e)),
                }
            } else if response.status() == 404 {
                Response::error(format!("Todo list with id {} not found", request.id))
            } else {
                Response::error(format!(
                    "API request failed with status: {}",
                    response.status()
                ))
            }
        }
        Err(e) => Response::error(format!("Network error: {}", e)),
    }
}

pub async fn delete_todo_list(
    client: &TodoListClient,
    request: DeleteTodoListRequest,
) -> Response<()> {
    let url = client.get_api_url("/todo-lists");

    match client.client.delete(&url).json(&request).send().await {
        Ok(response) => {
            if response.status().is_success() {
                Response::success_empty()
            } else if response.status() == 404 {
                Response::error(format!("Todo list with id {} not found", request.id))
            } else {
                Response::error(format!(
                    "API request failed with status: {}",
                    response.status()
                ))
            }
        }
        Err(e) => Response::error(format!("Network error: {}", e)),
    }
}
