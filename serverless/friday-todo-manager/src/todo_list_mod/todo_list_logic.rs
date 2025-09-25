use crate::business_response::BusinessResponse;
use crate::microsoft_graph_mod::microsoft_graph_api::MicrosoftGraphApi;
use crate::oauth_mod::oauth_api::OAuthApi;
use crate::todo_list_mod::todo_list::{CreateTodoListRequest, TodoList, UpdateTodoListRequest};
use tracing::{debug, error, info, warn};

pub async fn get_all_todo_lists(
) -> Result<BusinessResponse<Vec<TodoList>>, Box<dyn std::error::Error>> {
    info!("Logic layer: Getting all todo lists");

    // Get access token from OAuth manager
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

    let graph_client = MicrosoftGraphApi::new();
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

pub async fn get_todo_list(
    list_id: &str,
) -> Result<BusinessResponse<TodoList>, Box<dyn std::error::Error>> {
    info!("Logic layer: Getting todo list with ID: {}", list_id);

    if list_id.trim().is_empty() {
        warn!("Logic layer: Invalid list_id provided (empty or whitespace)");
        return Ok(BusinessResponse::error("List ID cannot be empty"));
    }

    // Get access token from OAuth manager
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

    let graph_client = MicrosoftGraphApi::new();
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

pub async fn create_todo_list(
    request: CreateTodoListRequest,
) -> Result<BusinessResponse<TodoList>, Box<dyn std::error::Error>> {
    info!(
        "Logic layer: Creating todo list with name: {}",
        request.display_name
    );

    // Validate request
    if request.display_name.trim().is_empty() {
        warn!("Logic layer: Invalid display_name provided (empty or whitespace)");
        return Ok(BusinessResponse::error("Display name cannot be empty"));
    }

    if request.display_name.len() > 255 {
        warn!(
            "Logic layer: Display name too long: {} characters",
            request.display_name.len()
        );
        return Ok(BusinessResponse::error(
            "Display name cannot exceed 255 characters",
        ));
    }

    // Get access token from OAuth manager
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

    let graph_client = MicrosoftGraphApi::new();
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

pub async fn update_todo_list(
    request: UpdateTodoListRequest,
) -> Result<BusinessResponse<TodoList>, Box<dyn std::error::Error>> {
    info!(
        "Logic layer: Updating todo list with ID: {} and name: {}",
        request.id, request.display_name
    );

    // Validate request
    if request.id.trim().is_empty() {
        warn!("Logic layer: Invalid ID provided (empty or whitespace)");
        return Ok(BusinessResponse::error("List ID cannot be empty"));
    }

    if request.display_name.trim().is_empty() {
        warn!("Logic layer: Invalid display_name provided (empty or whitespace)");
        return Ok(BusinessResponse::error("Display name cannot be empty"));
    }

    if request.display_name.len() > 255 {
        warn!(
            "Logic layer: Display name too long: {} characters",
            request.display_name.len()
        );
        return Ok(BusinessResponse::error(
            "Display name cannot exceed 255 characters",
        ));
    }

    // Get access token from OAuth manager
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

    let graph_client = MicrosoftGraphApi::new();
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

pub async fn delete_todo_list(
    list_id: &str,
) -> Result<BusinessResponse<String>, Box<dyn std::error::Error>> {
    info!("Logic layer: Deleting todo list with ID: {}", list_id);

    if list_id.trim().is_empty() {
        warn!("Logic layer: Invalid list_id provided (empty or whitespace)");
        return Ok(BusinessResponse::error("List ID cannot be empty"));
    }

    // Get access token from OAuth manager
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

    let graph_client = MicrosoftGraphApi::new();
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
