use crate::business_response::BusinessResponse;
use crate::microsoft_graph_mod::task_response::TaskResponse;
use crate::microsoft_graph_mod::tasks_response::TasksResponse;
use crate::todo_mod::task::{CreateTaskRequest, Task, UpdateTaskRequest};
use reqwest::Client;
use serde_json::json;
use tracing::{error, info};

#[derive(Debug)]
pub struct TasksMicrosoftGraphApi {
    client: Client,
    base_url: String,
}

impl TasksMicrosoftGraphApi {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://graph.microsoft.com/v1.0".to_string(),
        }
    }

    pub async fn get_tasks(
        &self,
        list_id: &str,
        access_token: &str,
    ) -> Result<BusinessResponse<Vec<Task>>, Box<dyn std::error::Error>> {
        info!("Data layer: Fetching all tasks from list ID: {}", list_id);

        if access_token.trim().is_empty() {
            error!("Data layer: Access token is empty");
            return Ok(BusinessResponse::error("Access token is required"));
        }

        if list_id.trim().is_empty() {
            error!("Data layer: List ID is empty");
            return Ok(BusinessResponse::error("List ID is required"));
        }

        let url = format!("{}/me/todo/lists/{}/tasks", self.base_url, list_id);
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
            let response_text = match response.text().await {
                Ok(text) => text,
                Err(e) => {
                    error!("Data layer: Failed to read response body: {}", e);
                    return Ok(BusinessResponse::error(
                        "Failed to read Microsoft Graph response",
                    ));
                }
            };

            info!(
                "Data layer: Raw response from Microsoft Graph: {}",
                response_text
            );

            match serde_json::from_str::<TasksResponse>(&response_text) {
                Ok(tasks_response) => {
                    let tasks: Vec<Task> = tasks_response
                        .value
                        .into_iter()
                        .map(Task::from)
                        .collect();

                    info!(
                        "Data layer: Successfully parsed {} tasks",
                        tasks.len()
                    );
                    Ok(BusinessResponse::success(tasks))
                }
                Err(_e) => {
                    error!(
                        "Data layer: Response that failed to parse: {}",
                        response_text
                    );
                    Ok(BusinessResponse::error(
                        "Failed to parse Microsoft Graph response",
                    ))
                }
            }
        } else {
            match response.text().await {
                Ok(error_text) => {
                    error!(
                        "Data layer: Microsoft Graph API error ({}): {}",
                        status, error_text
                    );
                    Ok(BusinessResponse::error(&format!(
                        "Failed to get tasks: {} - {}",
                        status, error_text
                    )))
                }
                Err(e) => {
                    error!("Data layer: Failed to read error response: {}", e);
                    Ok(BusinessResponse::error(&format!(
                        "Failed to get tasks: HTTP {}",
                        status
                    )))
                }
            }
        }
    }

    pub async fn get_task(
        &self,
        list_id: &str,
        task_id: &str,
        access_token: &str,
    ) -> Result<BusinessResponse<Task>, Box<dyn std::error::Error>> {
        info!("Data layer: Fetching task with ID: {} from list: {}", task_id, list_id);

        if access_token.trim().is_empty() {
            error!("Data layer: Access token is empty");
            return Ok(BusinessResponse::error("Access token is required"));
        }

        if list_id.trim().is_empty() {
            error!("Data layer: List ID is empty");
            return Ok(BusinessResponse::error("List ID is required"));
        }

        if task_id.trim().is_empty() {
            error!("Data layer: Task ID is empty");
            return Ok(BusinessResponse::error("Task ID is required"));
        }

        let url = format!("{}/me/todo/lists/{}/tasks/{}", self.base_url, list_id, task_id);
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
            let response_text = match response.text().await {
                Ok(text) => text,
                Err(e) => {
                    error!("Data layer: Failed to read response body: {}", e);
                    return Ok(BusinessResponse::error(
                        "Failed to read Microsoft Graph response",
                    ));
                }
            };

            info!(
                "Data layer: Raw response from Microsoft Graph: {}",
                response_text
            );

            match serde_json::from_str::<TaskResponse>(&response_text) {
                Ok(task_response) => {
                    let task = Task::from(task_response);
                    info!("Data layer: Successfully parsed task");
                    Ok(BusinessResponse::success(task))
                }
                Err(e) => {
                    error!("Data layer: Failed to parse task response: {}", e);
                    error!(
                        "Data layer: Response that failed to parse: {}",
                        response_text
                    );
                    Ok(BusinessResponse::error(
                        "Failed to parse Microsoft Graph response",
                    ))
                }
            }
        } else if status == 404 {
            info!("Data layer: Task not found with ID: {} in list: {}", task_id, list_id);
            Ok(BusinessResponse::error("Task not found"))
        } else {
            match response.text().await {
                Ok(error_text) => {
                    error!(
                        "Data layer: Microsoft Graph API error ({}): {}",
                        status, error_text
                    );
                    Ok(BusinessResponse::error(&format!(
                        "Failed to get task: {} - {}",
                        status, error_text
                    )))
                }
                Err(e) => {
                    error!("Data layer: Failed to read error response: {}", e);
                    Ok(BusinessResponse::error(&format!(
                        "Failed to get task: HTTP {}",
                        status
                    )))
                }
            }
        }
    }

    pub async fn create_task(
        &self,
        request: CreateTaskRequest,
        access_token: &str,
    ) -> Result<BusinessResponse<Task>, Box<dyn std::error::Error>> {
        info!(
            "Data layer: Creating task with title: {} in list: {}",
            request.title, request.list_id
        );

        if access_token.trim().is_empty() {
            error!("Data layer: Access token is empty");
            return Ok(BusinessResponse::error("Access token is required"));
        }

        let mut body = json!({
            "title": request.title
        });

        if let Some(body_content) = &request.body {
            body["body"] = json!({
                "content": body_content,
                "contentType": "text"
            });
        }

        if let Some(importance) = &request.importance {
            body["importance"] = json!(match importance {
                crate::todo_mod::task::TaskImportance::Low => "low",
                crate::todo_mod::task::TaskImportance::Normal => "normal",
                crate::todo_mod::task::TaskImportance::High => "high",
            });
        }

        if let Some(is_reminder_on) = request.is_reminder_on {
            body["isReminderOn"] = json!(is_reminder_on);
        }

        if let Some(reminder_date_time) = &request.reminder_date_time {
            body["reminderDateTime"] = json!({
                "dateTime": reminder_date_time.to_rfc3339(),
                "timeZone": "UTC"
            });
        }

        if let Some(due_date_time) = &request.due_date_time {
            body["dueDateTime"] = json!({
                "dateTime": due_date_time.to_rfc3339(),
                "timeZone": "UTC"
            });
        }

        let url = format!("{}/me/todo/lists/{}/tasks", self.base_url, request.list_id);
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
            let response_text = match response.text().await {
                Ok(text) => text,
                Err(e) => {
                    error!("Data layer: Failed to read response body: {}", e);
                    return Ok(BusinessResponse::error(
                        "Failed to read Microsoft Graph response",
                    ));
                }
            };

            info!(
                "Data layer: Raw response from Microsoft Graph: {}",
                response_text
            );

            match serde_json::from_str::<TaskResponse>(&response_text) {
                Ok(task_response) => {
                    let task = Task::from(task_response);
                    info!("Data layer: Successfully created task");
                    Ok(BusinessResponse::success(task))
                }
                Err(e) => {
                    error!(
                        "Data layer: Failed to parse create task response: {}",
                        e
                    );
                    error!(
                        "Data layer: Response that failed to parse: {}",
                        response_text
                    );
                    Ok(BusinessResponse::error(
                        "Failed to parse Microsoft Graph response",
                    ))
                }
            }
        } else {
            match response.text().await {
                Ok(error_text) => {
                    error!(
                        "Data layer: Microsoft Graph API error ({}): {}",
                        status, error_text
                    );
                    Ok(BusinessResponse::error(&format!(
                        "Failed to create task: {} - {}",
                        status, error_text
                    )))
                }
                Err(e) => {
                    error!("Data layer: Failed to read error response: {}", e);
                    Ok(BusinessResponse::error(&format!(
                        "Failed to create task: HTTP {}",
                        status
                    )))
                }
            }
        }
    }

    pub async fn update_task(
        &self,
        request: UpdateTaskRequest,
        access_token: &str,
    ) -> Result<BusinessResponse<Task>, Box<dyn std::error::Error>> {
        info!(
            "Data layer: Updating task with ID: {} in list: {}",
            request.id, request.list_id
        );

        if access_token.trim().is_empty() {
            error!("Data layer: Access token is empty");
            return Ok(BusinessResponse::error("Access token is required"));
        }

        let mut body = json!({});

        if let Some(title) = &request.title {
            body["title"] = json!(title);
        }

        if let Some(body_content) = &request.body {
            body["body"] = json!({
                "content": body_content,
                "contentType": "text"
            });
        }

        if let Some(status) = &request.status {
            body["status"] = json!(match status {
                crate::todo_mod::task::TaskStatus::NotStarted => "notStarted",
                crate::todo_mod::task::TaskStatus::InProgress => "inProgress",
                crate::todo_mod::task::TaskStatus::Completed => "completed",
                crate::todo_mod::task::TaskStatus::WaitingOnOthers => "waitingOnOthers",
                crate::todo_mod::task::TaskStatus::Deferred => "deferred",
            });
        }

        if let Some(importance) = &request.importance {
            body["importance"] = json!(match importance {
                crate::todo_mod::task::TaskImportance::Low => "low",
                crate::todo_mod::task::TaskImportance::Normal => "normal",
                crate::todo_mod::task::TaskImportance::High => "high",
            });
        }

        if let Some(is_reminder_on) = request.is_reminder_on {
            body["isReminderOn"] = json!(is_reminder_on);
        }

        if let Some(reminder_date_time) = &request.reminder_date_time {
            body["reminderDateTime"] = json!({
                "dateTime": reminder_date_time.to_rfc3339(),
                "timeZone": "UTC"
            });
        }

        if let Some(due_date_time) = &request.due_date_time {
            body["dueDateTime"] = json!({
                "dateTime": due_date_time.to_rfc3339(),
                "timeZone": "UTC"
            });
        }

        let url = format!("{}/me/todo/lists/{}/tasks/{}", self.base_url, request.list_id, request.id);
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
            let response_text = match response.text().await {
                Ok(text) => text,
                Err(e) => {
                    error!("Data layer: Failed to read response body: {}", e);
                    return Ok(BusinessResponse::error(
                        "Failed to read Microsoft Graph response",
                    ));
                }
            };

            info!(
                "Data layer: Raw response from Microsoft Graph: {}",
                response_text
            );

            match serde_json::from_str::<TaskResponse>(&response_text) {
                Ok(task_response) => {
                    let task = Task::from(task_response);
                    info!("Data layer: Successfully updated task");
                    Ok(BusinessResponse::success(task))
                }
                Err(e) => {
                    error!(
                        "Data layer: Failed to parse update task response: {}",
                        e
                    );
                    error!(
                        "Data layer: Response that failed to parse: {}",
                        response_text
                    );
                    Ok(BusinessResponse::error(
                        "Failed to parse Microsoft Graph response",
                    ))
                }
            }
        } else if status == 404 {
            info!(
                "Data layer: Task not found for update with ID: {} in list: {}",
                request.id, request.list_id
            );
            Ok(BusinessResponse::error("Task not found"))
        } else {
            match response.text().await {
                Ok(error_text) => {
                    error!(
                        "Data layer: Microsoft Graph API error ({}): {}",
                        status, error_text
                    );
                    Ok(BusinessResponse::error(&format!(
                        "Failed to update task: {} - {}",
                        status, error_text
                    )))
                }
                Err(e) => {
                    error!("Data layer: Failed to read error response: {}", e);
                    Ok(BusinessResponse::error(&format!(
                        "Failed to update task: HTTP {}",
                        status
                    )))
                }
            }
        }
    }

    pub async fn delete_task(
        &self,
        list_id: &str,
        task_id: &str,
        access_token: &str,
    ) -> Result<BusinessResponse<String>, Box<dyn std::error::Error>> {
        info!("Data layer: Deleting task with ID: {} from list: {}", task_id, list_id);

        if access_token.trim().is_empty() {
            error!("Data layer: Access token is empty");
            return Ok(BusinessResponse::error("Access token is required"));
        }

        if list_id.trim().is_empty() {
            error!("Data layer: List ID is empty");
            return Ok(BusinessResponse::error("List ID is required"));
        }

        if task_id.trim().is_empty() {
            error!("Data layer: Task ID is empty");
            return Ok(BusinessResponse::error("Task ID is required"));
        }

        let url = format!("{}/me/todo/lists/{}/tasks/{}", self.base_url, list_id, task_id);
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
            info!("Data layer: Successfully deleted task");
            Ok(BusinessResponse::success(
                "Task deleted successfully".to_string(),
            ))
        } else if status == 404 {
            info!(
                "Data layer: Task not found for deletion with ID: {} in list: {}",
                task_id, list_id
            );
            Ok(BusinessResponse::error("Task not found"))
        } else {
            match response.text().await {
                Ok(error_text) => {
                    error!(
                        "Data layer: Microsoft Graph API error ({}): {}",
                        status, error_text
                    );
                    Ok(BusinessResponse::error(&format!(
                        "Failed to delete task: {} - {}",
                        status, error_text
                    )))
                }
                Err(e) => {
                    error!("Data layer: Failed to read error response: {}", e);
                    Ok(BusinessResponse::error(&format!(
                        "Failed to delete task: HTTP {}",
                        status
                    )))
                }
            }
        }
    }
}