use crate::business_response::Response;
use crate::todo_mod::microsoft_graph_client::MicrosoftGraphClient;
use crate::todo_mod::oauth_client::OAuthClient;
use crate::todo_mod::todo_list::{CreateTodoListRequest, TodoList, UpdateTodoListRequest};
use tracing::{debug, error, info, instrument, warn};

#[instrument(name = "logic_get_all_todo_lists")]
pub async fn get_all_todo_lists() -> Result<Response<Vec<TodoList>>, Box<dyn std::error::Error>> {
    info!("Logic layer: Getting all todo lists");

    // Get access token from OAuth manager
    let oauth_client = OAuthClient::new();
    let access_token = match oauth_client.generate_access_token().await {
        Ok(token) => token,
        Err(e) => {
            error!(
                "Logic layer: Failed to get access token from OAuth manager: {}",
                e
            );
            return Ok(Response::error(
                "Failed to authenticate with Microsoft Graph",
            ));
        }
    };

    let graph_client = MicrosoftGraphClient::new();
    match graph_client.get_todo_lists(&access_token).await {
        Ok(response) => {
            if response.success {
                info!(
                    "Logic layer: Successfully retrieved {} todo lists",
                    response.data.as_ref().map(|lists| lists.len()).unwrap_or(0)
                );
            } else {
                warn!(
                    "Logic layer: Failed to retrieve todo lists from Microsoft Graph: {:?}",
                    response.errors
                );
            }
            Ok(response)
        }
        Err(e) => {
            error!(
                "Logic layer: Error getting todo lists from Microsoft Graph: {}",
                e
            );
            Err(e)
        }
    }
}

#[instrument(name = "logic_get_todo_list", fields(list_id = %list_id))]
pub async fn get_todo_list(
    list_id: &str,
) -> Result<Response<TodoList>, Box<dyn std::error::Error>> {
    info!("Logic layer: Getting todo list with ID: {}", list_id);

    if list_id.trim().is_empty() {
        warn!("Logic layer: Invalid list_id provided (empty or whitespace)");
        return Ok(Response::error("List ID cannot be empty"));
    }

    // Get access token from OAuth manager
    let oauth_client = OAuthClient::new();
    let access_token = match oauth_client.generate_access_token().await {
        Ok(token) => token,
        Err(e) => {
            error!(
                "Logic layer: Failed to get access token from OAuth manager: {}",
                e
            );
            return Ok(Response::error(
                "Failed to authenticate with Microsoft Graph",
            ));
        }
    };

    let graph_client = MicrosoftGraphClient::new();
    match graph_client.get_todo_list(list_id, &access_token).await {
        Ok(response) => {
            if response.success {
                info!(
                    "Logic layer: Successfully retrieved todo list with ID: {}",
                    list_id
                );
            } else {
                warn!(
                    "Logic layer: Failed to retrieve todo list with ID: {}, errors: {:?}",
                    list_id, response.errors
                );
            }
            Ok(response)
        }
        Err(e) => {
            error!(
                "Logic layer: Error getting todo list with ID: {}, error: {}",
                list_id, e
            );
            Err(e)
        }
    }
}

#[instrument(name = "logic_create_todo_list", fields(display_name = %request.display_name))]
pub async fn create_todo_list(
    request: CreateTodoListRequest,
) -> Result<Response<TodoList>, Box<dyn std::error::Error>> {
    info!(
        "Logic layer: Creating todo list with name: {}",
        request.display_name
    );

    // Validate request
    if request.display_name.trim().is_empty() {
        warn!("Logic layer: Invalid display_name provided (empty or whitespace)");
        return Ok(Response::error("Display name cannot be empty"));
    }

    if request.display_name.len() > 255 {
        warn!(
            "Logic layer: Display name too long: {} characters",
            request.display_name.len()
        );
        return Ok(Response::error("Display name cannot exceed 255 characters"));
    }

    // Get access token from OAuth manager
    let oauth_client = OAuthClient::new();
    let access_token = match oauth_client.generate_access_token().await {
        Ok(token) => token,
        Err(e) => {
            error!(
                "Logic layer: Failed to get access token from OAuth manager: {}",
                e
            );
            return Ok(Response::error(
                "Failed to authenticate with Microsoft Graph",
            ));
        }
    };

    let graph_client = MicrosoftGraphClient::new();
    match graph_client.create_todo_list(request, &access_token).await {
        Ok(result) => {
            if result.success {
                debug!("Logic layer: Successfully created todo list");
            } else {
                warn!(
                    "Logic layer: Failed to create todo list: {:?}",
                    result.errors
                );
            }
            Ok(result)
        }
        Err(e) => {
            error!("Logic layer: Error creating todo list: {}", e);
            Err(e)
        }
    }
}

#[instrument(name = "logic_update_todo_list", fields(id = %request.id, display_name = %request.display_name))]
pub async fn update_todo_list(
    request: UpdateTodoListRequest,
) -> Result<Response<TodoList>, Box<dyn std::error::Error>> {
    info!(
        "Logic layer: Updating todo list with ID: {} and name: {}",
        request.id, request.display_name
    );

    // Validate request
    if request.id.trim().is_empty() {
        warn!("Logic layer: Invalid ID provided (empty or whitespace)");
        return Ok(Response::error("List ID cannot be empty"));
    }

    if request.display_name.trim().is_empty() {
        warn!("Logic layer: Invalid display_name provided (empty or whitespace)");
        return Ok(Response::error("Display name cannot be empty"));
    }

    if request.display_name.len() > 255 {
        warn!(
            "Logic layer: Display name too long: {} characters",
            request.display_name.len()
        );
        return Ok(Response::error("Display name cannot exceed 255 characters"));
    }

    // Get access token from OAuth manager
    let oauth_client = OAuthClient::new();
    let access_token = match oauth_client.generate_access_token().await {
        Ok(token) => token,
        Err(e) => {
            error!(
                "Logic layer: Failed to get access token from OAuth manager: {}",
                e
            );
            return Ok(Response::error(
                "Failed to authenticate with Microsoft Graph",
            ));
        }
    };

    let graph_client = MicrosoftGraphClient::new();
    match graph_client.update_todo_list(request, &access_token).await {
        Ok(result) => {
            if result.success {
                debug!("Logic layer: Successfully updated todo list");
            } else {
                warn!(
                    "Logic layer: Failed to update todo list: {:?}",
                    result.errors
                );
            }
            Ok(result)
        }
        Err(e) => {
            error!("Logic layer: Error updating todo list: {}", e);
            Err(e)
        }
    }
}

#[instrument(name = "logic_delete_todo_list", fields(list_id = %list_id))]
pub async fn delete_todo_list(
    list_id: &str,
) -> Result<Response<String>, Box<dyn std::error::Error>> {
    info!("Logic layer: Deleting todo list with ID: {}", list_id);

    if list_id.trim().is_empty() {
        warn!("Logic layer: Invalid list_id provided (empty or whitespace)");
        return Ok(Response::error("List ID cannot be empty"));
    }

    // Get access token from OAuth manager
    let oauth_client = OAuthClient::new();
    let access_token = match oauth_client.generate_access_token().await {
        Ok(token) => token,
        Err(e) => {
            error!(
                "Logic layer: Failed to get access token from OAuth manager: {}",
                e
            );
            return Ok(Response::error(
                "Failed to authenticate with Microsoft Graph",
            ));
        }
    };

    let graph_client = MicrosoftGraphClient::new();
    match graph_client.delete_todo_list(list_id, &access_token).await {
        Ok(response) => {
            if response.success {
                info!(
                    "Logic layer: Successfully deleted todo list with ID: {}",
                    list_id
                );
            } else {
                warn!(
                    "Logic layer: Failed to delete todo list with ID: {}, errors: {:?}",
                    list_id, response.errors
                );
            }
            Ok(response)
        }
        Err(e) => {
            error!(
                "Logic layer: Error deleting todo list with ID: {}, error: {}",
                list_id, e
            );
            Err(e)
        }
    }
}
