use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use tracing::{error, info, instrument, warn};

extern crate dotenv;

use crate::business_response::Response;
use crate::todo_mod::{
    todo_list::{CreateTodoListRequest, DeleteTodoListRequest, UpdateTodoListRequest},
    todo_logic,
};

#[utoipa::path(
    get,
    path = "/api/friday-todo-manager/todo-lists/{list_id}",
    tag = "Todo Lists",
    params(
        ("list_id", Path, description = "Todo list ID to retrieve")
    ),
    responses(
        (status = 200, description = "Todo list retrieved successfully", body = TodoList),
        (status = 404, description = "Todo list not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[get("/api/friday-todo-manager/todo-lists/{list_id}")]
#[instrument(name = "get_todo_list", skip_all, fields(list_id = %list_id.as_ref()))]
pub async fn get_todo_list(list_id: web::Path<String>) -> impl Responder {
    info!(
        "Attempting to retrieve todo list with ID: {}",
        list_id.as_ref()
    );

    match todo_logic::get_todo_list(&list_id).await {
        Ok(response) => {
            if response.success {
                info!(
                    "Successfully retrieved todo list with ID: {}",
                    list_id.as_ref()
                );
                HttpResponse::Ok().json(response)
            } else {
                warn!(
                    "Failed to retrieve todo list with ID: {}, errors: {:?}",
                    list_id.as_ref(),
                    response.errors
                );
                HttpResponse::NotFound().json(response)
            }
        }
        Err(e) => {
            error!(
                "Error retrieving todo list with ID: {}, error: {}",
                list_id.as_ref(),
                e
            );
            let error_response: Response<String> =
                Response::error(&format!("Internal server error: {}", e));
            HttpResponse::InternalServerError().json(error_response)
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/friday-todo-manager/todo-lists",
    tag = "Todo Lists",
    responses(
        (status = 200, description = "All todo lists retrieved successfully", body = Vec<TodoList>),
        (status = 500, description = "Internal server error")
    )
)]
#[get("/api/friday-todo-manager/todo-lists")]
#[instrument(name = "get_all_todo_lists")]
pub async fn get_all_todo_lists() -> impl Responder {
    info!("Attempting to retrieve all todo lists");

    match todo_logic::get_all_todo_lists().await {
        Ok(response) => {
            if response.success {
                info!(
                    "Successfully retrieved {} todo lists",
                    response.data.as_ref().map(|lists| lists.len()).unwrap_or(0)
                );
                HttpResponse::Ok().json(response)
            } else {
                warn!(
                    "Failed to retrieve todo lists, errors: {:?}",
                    response.errors
                );
                HttpResponse::InternalServerError().json(response)
            }
        }
        Err(e) => {
            error!("Error retrieving all todo lists: {}", e);
            let error_response: Response<String> =
                Response::error(&format!("Internal server error: {}", e));
            HttpResponse::InternalServerError().json(error_response)
        }
    }
}

#[utoipa::path(
    post,
    path = "/api/friday-todo-manager/todo-lists",
    tag = "Todo Lists",
    request_body = CreateTodoListRequest,
    responses(
        (status = 200, description = "Todo list created successfully", body = TodoList),
        (status = 400, description = "Invalid todo list data"),
        (status = 500, description = "Internal server error")
    )
)]
#[post("/api/friday-todo-manager/todo-lists")]
#[instrument(name = "create_todo_list", skip_all, fields(display_name = %request.display_name))]
pub async fn create_todo_list(
    request: actix_web::web::Json<CreateTodoListRequest>,
) -> impl Responder {
    let request_data = request.into_inner();
    info!(
        "Attempting to create todo list with name: {}",
        request_data.display_name
    );

    match todo_logic::create_todo_list(request_data).await {
        Ok(response) => {
            if response.success {
                info!("Successfully created todo list");
                HttpResponse::Ok().json(response)
            } else {
                warn!("Failed to create todo list, errors: {:?}", response.errors);
                HttpResponse::BadRequest().json(response)
            }
        }
        Err(e) => {
            error!("Error creating todo list: {}", e);
            let error_response: Response<String> =
                Response::error(&format!("Internal server error: {}", e));
            HttpResponse::InternalServerError().json(error_response)
        }
    }
}

#[utoipa::path(
    put,
    path = "/api/friday-todo-manager/todo-lists",
    tag = "Todo Lists",
    request_body = UpdateTodoListRequest,
    responses(
        (status = 200, description = "Todo list updated successfully", body = TodoList),
        (status = 404, description = "Todo list not found"),
        (status = 400, description = "Invalid todo list data"),
        (status = 500, description = "Internal server error")
    )
)]
#[put("/api/friday-todo-manager/todo-lists")]
#[instrument(name = "update_todo_list", skip_all, fields(id = %request.id, display_name = %request.display_name))]
pub async fn update_todo_list(
    request: actix_web::web::Json<UpdateTodoListRequest>,
) -> impl Responder {
    let request_data = request.into_inner();
    info!(
        "Attempting to update todo list with ID: {} and name: {}",
        request_data.id, request_data.display_name
    );

    match todo_logic::update_todo_list(request_data).await {
        Ok(response) => {
            if response.success {
                info!("Successfully updated todo list");
                HttpResponse::Ok().json(response)
            } else {
                warn!("Failed to update todo list, errors: {:?}", response.errors);
                // Check if it's a not found error or validation error
                if response
                    .errors
                    .iter()
                    .any(|err| err.contains("not found") || err.contains("404"))
                {
                    HttpResponse::NotFound().json(response)
                } else {
                    HttpResponse::BadRequest().json(response)
                }
            }
        }
        Err(e) => {
            error!("Error updating todo list: {}", e);
            let error_response: Response<String> =
                Response::error(&format!("Internal server error: {}", e));
            HttpResponse::InternalServerError().json(error_response)
        }
    }
}

#[utoipa::path(
    delete,
    path = "/api/friday-todo-manager/todo-lists",
    tag = "Todo Lists",
    request_body = DeleteTodoListRequest,
    responses(
        (status = 200, description = "Todo list deleted successfully", body = String),
        (status = 404, description = "Todo list not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[delete("/api/friday-todo-manager/todo-lists")]
#[instrument(name = "delete_todo_list", skip_all, fields(id = %request.id))]
pub async fn delete_todo_list(
    request: actix_web::web::Json<DeleteTodoListRequest>,
) -> impl Responder {
    let list_id = &request.id;
    info!("Attempting to delete todo list with ID: {}", list_id);

    match todo_logic::delete_todo_list(list_id).await {
        Ok(response) => {
            if response.success {
                info!("Successfully deleted todo list with ID: {}", list_id);
                HttpResponse::Ok().json(response)
            } else {
                warn!(
                    "Failed to delete todo list with ID: {}, errors: {:?}",
                    list_id, response.errors
                );
                // Check if it's a not found error
                if response
                    .errors
                    .iter()
                    .any(|err| err.contains("not found") || err.contains("404"))
                {
                    HttpResponse::NotFound().json(response)
                } else {
                    HttpResponse::BadRequest().json(response)
                }
            }
        }
        Err(e) => {
            error!(
                "Error deleting todo list with ID: {}, error: {}",
                list_id, e
            );
            let error_response: Response<String> =
                Response::error(&format!("Internal server error: {}", e));
            HttpResponse::InternalServerError().json(error_response)
        }
    }
}
