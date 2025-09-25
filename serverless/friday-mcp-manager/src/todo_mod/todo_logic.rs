use reqwest;
use tracing::{error, info, warn};

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
