use crate::business_response::Response;
use crate::todo_mod::todo_list::{
    CreateTodoListRequest, TodoList, TodoListResponse, TodoListsResponse, UpdateTodoListRequest,
};
use reqwest::Client;
use serde_json::json;
use tracing::{error, info, instrument};

#[derive(Debug)]
pub struct MicrosoftGraphClient {
    client: Client,
    base_url: String,
}

impl MicrosoftGraphClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://graph.microsoft.com/v1.0".to_string(),
        }
    }

    #[instrument(name = "get_access_token")]
    async fn get_access_token(&self) -> Result<String, Box<dyn std::error::Error>> {
        info!("Data layer: Retrieving Microsoft Graph access token");

        match std::env::var("MICROSOFT_ACCESS_TOKEN") {
            Ok(token) => {
                if token.trim().is_empty() {
                    error!("Data layer: MICROSOFT_ACCESS_TOKEN environment variable is empty");
                    return Err("Microsoft Graph access token is not configured".into());
                }
                info!("Data layer: Successfully retrieved access token");
                Ok(token)
            }
            Err(_) => {
                error!("Data layer: MICROSOFT_ACCESS_TOKEN environment variable is not set");
                Err("Microsoft Graph access token is not configured".into())
            }
        }
    }

    #[instrument(name = "data_get_todo_lists")]
    pub async fn get_todo_lists(
        &self,
    ) -> Result<Response<Vec<TodoList>>, Box<dyn std::error::Error>> {
        info!("Data layer: Fetching all todo lists from Microsoft Graph");

        let access_token = match self.get_access_token().await {
            Ok(token) => token,
            Err(e) => {
                error!("Data layer: Failed to get access token: {}", e);
                return Ok(Response::error("Authentication failed"));
            }
        };

        let url = format!("{}/me/todo/lists", self.base_url);
        info!("Data layer: Making GET request to: {}", url);

        let response = match self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Content-Type", "application/json")
            .send()
            .await
        {
            Ok(response) => response,
            Err(e) => {
                error!("Data layer: HTTP request failed: {}", e);
                return Err(e.into());
            }
        };

        let status = response.status();
        info!("Data layer: Received response with status: {}", status);

        if response.status().is_success() {
            // First get the response text for debugging
            let response_text = match response.text().await {
                Ok(text) => text,
                Err(e) => {
                    error!("Data layer: Failed to read response body: {}", e);
                    return Ok(Response::error("Failed to read Microsoft Graph response"));
                }
            };

            info!(
                "Data layer: Raw response from Microsoft Graph: {}",
                response_text
            );

            match serde_json::from_str::<TodoListsResponse>(&response_text) {
                Ok(todo_lists_response) => {
                    let todo_lists: Vec<TodoList> = todo_lists_response
                        .value
                        .into_iter()
                        .map(TodoList::from)
                        .collect();

                    info!(
                        "Data layer: Successfully parsed {} todo lists",
                        todo_lists.len()
                    );
                    Ok(Response::success(todo_lists))
                }
                Err(e) => {
                    error!(
                        "Data layer: Response that failed to parse: {}",
                        response_text
                    );
                    Ok(Response::error("Failed to parse Microsoft Graph response"))
                }
            }
        } else {
            match response.text().await {
                Ok(error_text) => {
                    error!(
                        "Data layer: Microsoft Graph API error ({}): {}",
                        status, error_text
                    );
                    Ok(Response::error(&format!(
                        "Failed to get todo lists: {} - {}",
                        status, error_text
                    )))
                }
                Err(e) => {
                    error!("Data layer: Failed to read error response: {}", e);
                    Ok(Response::error(&format!(
                        "Failed to get todo lists: HTTP {}",
                        status
                    )))
                }
            }
        }
    }

    #[instrument(name = "data_get_todo_list", fields(list_id = %list_id))]
    pub async fn get_todo_list(
        &self,
        list_id: &str,
    ) -> Result<Response<TodoList>, Box<dyn std::error::Error>> {
        info!("Data layer: Fetching todo list with ID: {}", list_id);

        let access_token = match self.get_access_token().await {
            Ok(token) => token,
            Err(e) => {
                error!("Data layer: Failed to get access token: {}", e);
                return Ok(Response::error("Authentication failed"));
            }
        };

        let url = format!("{}/me/todo/lists/{}", self.base_url, list_id);
        info!("Data layer: Making GET request to: {}", url);

        let response = match self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Content-Type", "application/json")
            .send()
            .await
        {
            Ok(response) => response,
            Err(e) => {
                error!("Data layer: HTTP request failed: {}", e);
                return Err(e.into());
            }
        };

        let status = response.status();
        info!("Data layer: Received response with status: {}", status);

        if response.status().is_success() {
            // First get the response text for debugging
            let response_text = match response.text().await {
                Ok(text) => text,
                Err(e) => {
                    error!("Data layer: Failed to read response body: {}", e);
                    return Ok(Response::error("Failed to read Microsoft Graph response"));
                }
            };

            info!(
                "Data layer: Raw response from Microsoft Graph: {}",
                response_text
            );

            match serde_json::from_str::<TodoListResponse>(&response_text) {
                Ok(list_response) => {
                    let todo_list = TodoList::from(list_response);
                    info!("Data layer: Successfully parsed todo list");
                    Ok(Response::success(todo_list))
                }
                Err(e) => {
                    error!("Data layer: Failed to parse todo list response: {}", e);
                    error!(
                        "Data layer: Response that failed to parse: {}",
                        response_text
                    );
                    Ok(Response::error("Failed to parse Microsoft Graph response"))
                }
            }
        } else if status == 404 {
            info!("Data layer: Todo list not found with ID: {}", list_id);
            Ok(Response::error("Todo list not found"))
        } else {
            match response.text().await {
                Ok(error_text) => {
                    error!(
                        "Data layer: Microsoft Graph API error ({}): {}",
                        status, error_text
                    );
                    Ok(Response::error(&format!(
                        "Failed to get todo list: {} - {}",
                        status, error_text
                    )))
                }
                Err(e) => {
                    error!("Data layer: Failed to read error response: {}", e);
                    Ok(Response::error(&format!(
                        "Failed to get todo list: HTTP {}",
                        status
                    )))
                }
            }
        }
    }

    #[instrument(name = "data_create_todo_list", fields(display_name = %request.display_name))]
    pub async fn create_todo_list(
        &self,
        request: CreateTodoListRequest,
    ) -> Result<Response<TodoList>, Box<dyn std::error::Error>> {
        info!(
            "Data layer: Creating todo list with name: {}",
            request.display_name
        );

        let access_token = match self.get_access_token().await {
            Ok(token) => token,
            Err(e) => {
                error!("Data layer: Failed to get access token: {}", e);
                return Ok(Response::error("Authentication failed"));
            }
        };

        let body = json!({
            "displayName": request.display_name
        });

        let url = format!("{}/me/todo/lists", self.base_url);
        info!("Data layer: Making POST request to: {}", url);

        let response = match self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
        {
            Ok(response) => response,
            Err(e) => {
                error!("Data layer: HTTP request failed: {}", e);
                return Err(e.into());
            }
        };

        let status = response.status();
        info!("Data layer: Received response with status: {}", status);

        if response.status().is_success() {
            // First get the response text for debugging
            let response_text = match response.text().await {
                Ok(text) => text,
                Err(e) => {
                    error!("Data layer: Failed to read response body: {}", e);
                    return Ok(Response::error("Failed to read Microsoft Graph response"));
                }
            };

            info!(
                "Data layer: Raw response from Microsoft Graph: {}",
                response_text
            );

            match serde_json::from_str::<TodoListResponse>(&response_text) {
                Ok(list_response) => {
                    let todo_list = TodoList::from(list_response);
                    info!("Data layer: Successfully created todo list");
                    Ok(Response::success(todo_list))
                }
                Err(e) => {
                    error!(
                        "Data layer: Failed to parse create todo list response: {}",
                        e
                    );
                    error!(
                        "Data layer: Response that failed to parse: {}",
                        response_text
                    );
                    Ok(Response::error("Failed to parse Microsoft Graph response"))
                }
            }
        } else {
            match response.text().await {
                Ok(error_text) => {
                    error!(
                        "Data layer: Microsoft Graph API error ({}): {}",
                        status, error_text
                    );
                    Ok(Response::error(&format!(
                        "Failed to create todo list: {} - {}",
                        status, error_text
                    )))
                }
                Err(e) => {
                    error!("Data layer: Failed to read error response: {}", e);
                    Ok(Response::error(&format!(
                        "Failed to create todo list: HTTP {}",
                        status
                    )))
                }
            }
        }
    }

    #[instrument(name = "data_update_todo_list", fields(id = %request.id, display_name = %request.display_name))]
    pub async fn update_todo_list(
        &self,
        request: UpdateTodoListRequest,
    ) -> Result<Response<TodoList>, Box<dyn std::error::Error>> {
        info!(
            "Data layer: Updating todo list with ID: {} and name: {}",
            request.id, request.display_name
        );

        let access_token = match self.get_access_token().await {
            Ok(token) => token,
            Err(e) => {
                error!("Data layer: Failed to get access token: {}", e);
                return Ok(Response::error("Authentication failed"));
            }
        };

        let body = json!({
            "displayName": request.display_name
        });

        let url = format!("{}/me/todo/lists/{}", self.base_url, request.id);
        info!("Data layer: Making PATCH request to: {}", url);

        let response = match self
            .client
            .patch(&url)
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
        {
            Ok(response) => response,
            Err(e) => {
                error!("Data layer: HTTP request failed: {}", e);
                return Err(e.into());
            }
        };

        let status = response.status();
        info!("Data layer: Received response with status: {}", status);

        if response.status().is_success() {
            // First get the response text for debugging
            let response_text = match response.text().await {
                Ok(text) => text,
                Err(e) => {
                    error!("Data layer: Failed to read response body: {}", e);
                    return Ok(Response::error("Failed to read Microsoft Graph response"));
                }
            };

            info!(
                "Data layer: Raw response from Microsoft Graph: {}",
                response_text
            );

            match serde_json::from_str::<TodoListResponse>(&response_text) {
                Ok(list_response) => {
                    let todo_list = TodoList::from(list_response);
                    info!("Data layer: Successfully updated todo list");
                    Ok(Response::success(todo_list))
                }
                Err(e) => {
                    error!(
                        "Data layer: Failed to parse update todo list response: {}",
                        e
                    );
                    error!(
                        "Data layer: Response that failed to parse: {}",
                        response_text
                    );
                    Ok(Response::error("Failed to parse Microsoft Graph response"))
                }
            }
        } else if status == 404 {
            info!(
                "Data layer: Todo list not found for update with ID: {}",
                request.id
            );
            Ok(Response::error("Todo list not found"))
        } else {
            match response.text().await {
                Ok(error_text) => {
                    error!(
                        "Data layer: Microsoft Graph API error ({}): {}",
                        status, error_text
                    );
                    Ok(Response::error(&format!(
                        "Failed to update todo list: {} - {}",
                        status, error_text
                    )))
                }
                Err(e) => {
                    error!("Data layer: Failed to read error response: {}", e);
                    Ok(Response::error(&format!(
                        "Failed to update todo list: HTTP {}",
                        status
                    )))
                }
            }
        }
    }

    #[instrument(name = "data_delete_todo_list", fields(list_id = %list_id))]
    pub async fn delete_todo_list(
        &self,
        list_id: &str,
    ) -> Result<Response<String>, Box<dyn std::error::Error>> {
        info!("Data layer: Deleting todo list with ID: {}", list_id);

        let access_token = match self.get_access_token().await {
            Ok(token) => token,
            Err(e) => {
                error!("Data layer: Failed to get access token: {}", e);
                return Ok(Response::error("Authentication failed"));
            }
        };

        let url = format!("{}/me/todo/lists/{}", self.base_url, list_id);
        info!("Data layer: Making DELETE request to: {}", url);

        let response = match self
            .client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await
        {
            Ok(response) => response,
            Err(e) => {
                error!("Data layer: HTTP request failed: {}", e);
                return Err(e.into());
            }
        };

        let status = response.status();
        info!("Data layer: Received response with status: {}", status);

        if response.status().is_success() {
            info!("Data layer: Successfully deleted todo list");
            Ok(Response::success(
                "Todo list deleted successfully".to_string(),
            ))
        } else if status == 404 {
            info!(
                "Data layer: Todo list not found for deletion with ID: {}",
                list_id
            );
            Ok(Response::error("Todo list not found"))
        } else {
            match response.text().await {
                Ok(error_text) => {
                    error!(
                        "Data layer: Microsoft Graph API error ({}): {}",
                        status, error_text
                    );
                    Ok(Response::error(&format!(
                        "Failed to delete todo list: {} - {}",
                        status, error_text
                    )))
                }
                Err(e) => {
                    error!("Data layer: Failed to read error response: {}", e);
                    Ok(Response::error(&format!(
                        "Failed to delete todo list: HTTP {}",
                        status
                    )))
                }
            }
        }
    }
}
