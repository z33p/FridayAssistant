use crate::business_response::BusinessResponse;
use crate::microsoft_graph_mod::tasks_microsoft_graph_api::TasksMicrosoftGraphApi;
use crate::oauth_mod::oauth_api::OAuthApi;
use crate::todo_mod::task::{CreateTaskRequest, Task, UpdateTaskRequest};
use tracing::{debug, error, info, warn};

pub async fn get_all_tasks(
    list_id: &str,
) -> Result<BusinessResponse<Vec<Task>>, Box<dyn std::error::Error>> {
    info!("Logic layer: Getting all tasks from list ID: {}", list_id);

    if list_id.trim().is_empty() {
        warn!("Logic layer: Invalid list_id provided (empty or whitespace)");
        return Ok(BusinessResponse::error("List ID cannot be empty"));
    }

    let oauth_client = OAuthApi::new();
    let access_token = match oauth_client.generate_access_token().await {
        Ok(token) => token,
        Err(e) => {
            error!(
                "Logic layer: Failed to get access token from OAuth manager: {}",
                e
            );
            return Ok(BusinessResponse::error(
                "Failed to authenticate with Microsoft Graph",
            ));
        }
    };

    let graph_client = TasksMicrosoftGraphApi::new();
    match graph_client.get_tasks(list_id, &access_token).await {
        Ok(response) => {
            if response.success {
                info!(
                    "Logic layer: Successfully retrieved {} tasks",
                    response.data.as_ref().map(|tasks| tasks.len()).unwrap_or(0)
                );
            } else {
                warn!(
                    "Logic layer: Failed to retrieve tasks from Microsoft Graph: {:?}",
                    response.errors
                );
            }
            Ok(response)
        }
        Err(e) => {
            error!(
                "Logic layer: Error getting tasks from Microsoft Graph: {}",
                e
            );
            Err(e)
        }
    }
}

pub async fn get_task(
    list_id: &str,
    task_id: &str,
) -> Result<BusinessResponse<Task>, Box<dyn std::error::Error>> {
    info!(
        "Logic layer: Getting task with ID: {} from list: {}",
        task_id, list_id
    );

    if list_id.trim().is_empty() {
        warn!("Logic layer: Invalid list_id provided (empty or whitespace)");
        return Ok(BusinessResponse::error("List ID cannot be empty"));
    }

    if task_id.trim().is_empty() {
        warn!("Logic layer: Invalid task_id provided (empty or whitespace)");
        return Ok(BusinessResponse::error("Task ID cannot be empty"));
    }

    let oauth_client = OAuthApi::new();
    let access_token = match oauth_client.generate_access_token().await {
        Ok(token) => token,
        Err(e) => {
            error!(
                "Logic layer: Failed to get access token from OAuth manager: {}",
                e
            );
            return Ok(BusinessResponse::error(
                "Failed to authenticate with Microsoft Graph",
            ));
        }
    };

    let graph_client = TasksMicrosoftGraphApi::new();
    match graph_client.get_task(list_id, task_id, &access_token).await {
        Ok(response) => {
            if response.success {
                info!("Logic layer: Successfully retrieved task");
            } else {
                warn!(
                    "Logic layer: Failed to retrieve task from Microsoft Graph: {:?}",
                    response.errors
                );
            }
            Ok(response)
        }
        Err(e) => {
            error!(
                "Logic layer: Error getting task from Microsoft Graph: {}",
                e
            );
            Err(e)
        }
    }
}

pub async fn create_task(
    request: CreateTaskRequest,
) -> Result<BusinessResponse<Task>, Box<dyn std::error::Error>> {
    let list_id = match &request.list_id {
        Some(id) => id,
        None => {
            warn!("Logic layer: List ID is required");
            return Ok(BusinessResponse::error("List ID cannot be empty"));
        }
    };

    info!(
        "Logic layer: Creating task with title: {} in list: {}",
        request.title, list_id
    );

    if request.title.trim().is_empty() {
        warn!("Logic layer: Invalid task title provided (empty or whitespace)");
        return Ok(BusinessResponse::error("Task title cannot be empty"));
    }

    if list_id.trim().is_empty() {
        warn!("Logic layer: Invalid list_id provided (empty or whitespace)");
        return Ok(BusinessResponse::error("List ID cannot be empty"));
    }

    let oauth_client = OAuthApi::new();
    let access_token = match oauth_client.generate_access_token().await {
        Ok(token) => token,
        Err(e) => {
            error!(
                "Logic layer: Failed to get access token from OAuth manager: {}",
                e
            );
            return Ok(BusinessResponse::error(
                "Failed to authenticate with Microsoft Graph",
            ));
        }
    };

    let graph_client = TasksMicrosoftGraphApi::new();
    match graph_client.create_task(request, &access_token).await {
        Ok(result) => {
            if result.success {
                debug!("Logic layer: Successfully created task");
            } else {
                warn!(
                    "Logic layer: Failed to create task in Microsoft Graph: {:?}",
                    result.errors
                );
            }
            Ok(result)
        }
        Err(e) => {
            error!("Logic layer: Error creating task in Microsoft Graph: {}", e);
            Err(e)
        }
    }
}

pub async fn update_task(
    request: UpdateTaskRequest,
) -> Result<BusinessResponse<Task>, Box<dyn std::error::Error>> {
    let task_id = match &request.id {
        Some(id) => id,
        None => {
            warn!("Logic layer: Task ID is required");
            return Ok(BusinessResponse::error("Task ID cannot be empty"));
        }
    };

    let list_id = match &request.list_id {
        Some(id) => id,
        None => {
            warn!("Logic layer: List ID is required");
            return Ok(BusinessResponse::error("List ID cannot be empty"));
        }
    };

    info!(
        "Logic layer: Updating task with ID: {} in list: {}",
        task_id, list_id
    );

    if task_id.trim().is_empty() {
        warn!("Logic layer: Invalid task ID provided (empty or whitespace)");
        return Ok(BusinessResponse::error("Task ID cannot be empty"));
    }

    if list_id.trim().is_empty() {
        warn!("Logic layer: Invalid list_id provided (empty or whitespace)");
        return Ok(BusinessResponse::error("List ID cannot be empty"));
    }

    let oauth_client = OAuthApi::new();
    let access_token = match oauth_client.generate_access_token().await {
        Ok(token) => token,
        Err(e) => {
            error!(
                "Logic layer: Failed to get access token from OAuth manager: {}",
                e
            );
            return Ok(BusinessResponse::error(
                "Failed to authenticate with Microsoft Graph",
            ));
        }
    };

    let graph_client = TasksMicrosoftGraphApi::new();
    match graph_client.update_task(request, &access_token).await {
        Ok(response) => {
            if response.success {
                info!("Logic layer: Successfully updated task");
            } else {
                warn!(
                    "Logic layer: Failed to update task in Microsoft Graph: {:?}",
                    response.errors
                );
            }
            Ok(response)
        }
        Err(e) => {
            error!("Logic layer: Error updating task in Microsoft Graph: {}", e);
            Err(e)
        }
    }
}

pub async fn delete_task(
    list_id: &str,
    task_id: &str,
) -> Result<BusinessResponse<String>, Box<dyn std::error::Error>> {
    info!(
        "Logic layer: Deleting task with ID: {} from list: {}",
        task_id, list_id
    );

    if list_id.trim().is_empty() {
        warn!("Logic layer: Invalid list_id provided (empty or whitespace)");
        return Ok(BusinessResponse::error("List ID cannot be empty"));
    }

    if task_id.trim().is_empty() {
        warn!("Logic layer: Invalid task_id provided (empty or whitespace)");
        return Ok(BusinessResponse::error("Task ID cannot be empty"));
    }

    let oauth_client = OAuthApi::new();
    let access_token = match oauth_client.generate_access_token().await {
        Ok(token) => token,
        Err(e) => {
            error!(
                "Logic layer: Failed to get access token from OAuth manager: {}",
                e
            );
            return Ok(BusinessResponse::error(
                "Failed to authenticate with Microsoft Graph",
            ));
        }
    };

    let graph_client = TasksMicrosoftGraphApi::new();
    match graph_client
        .delete_task(list_id, task_id, &access_token)
        .await
    {
        Ok(response) => {
            if response.success {
                info!("Logic layer: Successfully deleted task");
            } else {
                warn!(
                    "Logic layer: Failed to delete task from Microsoft Graph: {:?}",
                    response.errors
                );
            }
            Ok(response)
        }
        Err(e) => {
            error!(
                "Logic layer: Error deleting task from Microsoft Graph: {}",
                e
            );
            Err(e)
        }
    }
}
