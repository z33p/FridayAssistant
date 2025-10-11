use crate::business_response::BusinessResponse;
use crate::todo_mod::task::{
    CreateTaskRequest, CreateTaskRequestBody, Task, UpdateTaskRequest, UpdateTaskRequestBody,
};
use crate::todo_mod::task_logic;
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use tracing::{error, info};

#[utoipa::path(
    get,
    path = "/api/friday-todo-manager/lists/{list_id}/tasks",
    responses(
        (status = 200, description = "All tasks from the specified todo list retrieved successfully. Returns an array of tasks with their current status, due dates, and other metadata.", body = BusinessResponse<Vec<Task>>),
        (status = 400, description = "Bad request - invalid list ID format or missing required parameters", body = BusinessResponse<String>),
        (status = 500, description = "Internal server error - Microsoft Graph API error or system failure", body = BusinessResponse<String>)
    ),
    params(
        ("list_id" = String, Path, description = "The unique identifier of the todo list to retrieve tasks from. This is typically a Microsoft Graph ID.")
    ),
    tag = "Tasks"
)]
#[get("/api/friday-todo-manager/lists/{list_id}/tasks")]
pub async fn get_all_tasks(path: web::Path<String>) -> impl Responder {
    let list_id = path.into_inner();
    info!(
        "Controller layer: GET /api/friday-todo-manager/lists/{}/tasks",
        list_id
    );

    match task_logic::get_all_tasks(&list_id).await {
        Ok(response) => {
            if response.success {
                info!("Controller layer: Successfully retrieved tasks");
                HttpResponse::Ok().json(response)
            } else {
                error!(
                    "Controller layer: Business logic error: {:?}",
                    response.errors
                );
                HttpResponse::BadRequest().json(response)
            }
        }
        Err(e) => {
            error!("Controller layer: System error getting tasks: {}", e);
            HttpResponse::InternalServerError().json(BusinessResponse::<Vec<Task>>::error(
                "An internal error occurred while retrieving tasks",
            ))
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/friday-todo-manager/lists/{list_id}/tasks/{task_id}",
    responses(
        (status = 200, description = "Specific task retrieved successfully. Returns detailed task information including title, description, status, importance, and dates.", body = BusinessResponse<Task>),
        (status = 400, description = "Bad request - invalid list ID or task ID format", body = BusinessResponse<String>),
        (status = 404, description = "Task not found - the specified task ID does not exist in the given list", body = BusinessResponse<String>),
        (status = 500, description = "Internal server error", body = BusinessResponse<String>)
    ),
    params(
        ("list_id" = String, Path, description = "Todo list unique identifier"),
        ("task_id" = String, Path, description = "Task unique identifier")
    ),
    tag = "Tasks"
)]
#[get("/api/friday-todo-manager/lists/{list_id}/tasks/{task_id}")]
pub async fn get_task(path: web::Path<(String, String)>) -> impl Responder {
    let (list_id, task_id) = path.into_inner();
    info!(
        "Controller layer: GET /api/friday-todo-manager/lists/{}/tasks/{}",
        list_id, task_id
    );

    match task_logic::get_task(&list_id, &task_id).await {
        Ok(response) => {
            if response.success {
                info!("Controller layer: Successfully retrieved task");
                HttpResponse::Ok().json(response)
            } else {
                if response.errors.iter().any(|e| e.contains("not found")) {
                    error!("Controller layer: Task not found: {:?}", response.errors);
                    HttpResponse::NotFound().json(response)
                } else {
                    error!(
                        "Controller layer: Business logic error: {:?}",
                        response.errors
                    );
                    HttpResponse::BadRequest().json(response)
                }
            }
        }
        Err(e) => {
            error!("Controller layer: System error getting task: {}", e);
            HttpResponse::InternalServerError().json(BusinessResponse::<Task>::error(
                "An internal error occurred while retrieving the task",
            ))
        }
    }
}

#[utoipa::path(
    post,
    path = "/api/friday-todo-manager/lists/{list_id}/tasks",
    request_body = CreateTaskRequestBody,
    responses(
        (status = 200, description = "Task created successfully", body = BusinessResponse<Task>),
        (status = 400, description = "Bad request", body = BusinessResponse<String>),
        (status = 500, description = "Internal server error", body = BusinessResponse<String>)
    ),
    params(
        ("list_id" = String, Path, description = "Todo list unique identifier")
    ),
    tag = "Tasks"
)]
#[post("/api/friday-todo-manager/lists/{list_id}/tasks")]
pub async fn create_task(
    path: web::Path<String>,
    request: web::Json<CreateTaskRequestBody>,
) -> impl Responder {
    let list_id = path.into_inner();

    let full_request = CreateTaskRequest {
        list_id: Some(list_id.clone()),
        title: request.title.clone(),
        body: request.body.clone(),
        importance: request.importance.clone(),
        is_reminder_on: request.is_reminder_on,
        reminder_date_time: request.reminder_date_time,
        due_date_time: request.due_date_time,
    };

    info!(
        "Controller layer: POST /api/friday-todo-manager/lists/{}/tasks with title: {}",
        list_id, full_request.title
    );

    match task_logic::create_task(full_request).await {
        Ok(response) => {
            if response.success {
                info!("Controller layer: Successfully created task");
                HttpResponse::Ok().json(response)
            } else {
                error!(
                    "Controller layer: Business logic error: {:?}",
                    response.errors
                );
                HttpResponse::BadRequest().json(response)
            }
        }
        Err(e) => {
            error!("Controller layer: System error creating task: {}", e);
            HttpResponse::InternalServerError().json(BusinessResponse::<Task>::error(
                "An internal error occurred while creating the task",
            ))
        }
    }
}

#[utoipa::path(
    patch,
    path = "/api/friday-todo-manager/lists/{list_id}/tasks/{task_id}",
    request_body = UpdateTaskRequestBody,
    responses(
        (status = 200, description = "Task updated successfully", body = BusinessResponse<Task>),
        (status = 400, description = "Bad request", body = BusinessResponse<String>),
        (status = 404, description = "Task not found", body = BusinessResponse<String>),
        (status = 500, description = "Internal server error", body = BusinessResponse<String>)
    ),
    params(
        ("list_id" = String, Path, description = "Todo list unique identifier"),
        ("task_id" = String, Path, description = "Task unique identifier")
    ),
    tag = "Tasks"
)]
#[patch("/api/friday-todo-manager/lists/{list_id}/tasks/{task_id}")]
pub async fn update_task(
    path: web::Path<(String, String)>,
    request: web::Json<UpdateTaskRequestBody>,
) -> impl Responder {
    let (list_id, task_id) = path.into_inner();

    let full_request = UpdateTaskRequest {
        id: Some(task_id.clone()),
        list_id: Some(list_id.clone()),
        title: request.title.clone(),
        body: request.body.clone(),
        status: request.status.clone(),
        importance: request.importance.clone(),
        is_reminder_on: request.is_reminder_on,
        reminder_date_time: request.reminder_date_time,
        due_date_time: request.due_date_time,
    };

    info!(
        "Controller layer: PATCH /api/friday-todo-manager/lists/{}/tasks/{}",
        list_id, task_id
    );

    match task_logic::update_task(full_request).await {
        Ok(response) => {
            if response.success {
                info!("Controller layer: Successfully updated task");
                HttpResponse::Ok().json(response)
            } else {
                if response.errors.iter().any(|e| e.contains("not found")) {
                    error!("Controller layer: Task not found: {:?}", response.errors);
                    HttpResponse::NotFound().json(response)
                } else {
                    error!(
                        "Controller layer: Business logic error: {:?}",
                        response.errors
                    );
                    HttpResponse::BadRequest().json(response)
                }
            }
        }
        Err(e) => {
            error!("Controller layer: System error updating task: {}", e);
            HttpResponse::InternalServerError().json(BusinessResponse::<Task>::error(
                "An internal error occurred while updating the task",
            ))
        }
    }
}

#[utoipa::path(
    delete,
    path = "/api/friday-todo-manager/lists/{list_id}/tasks/{task_id}",
    responses(
        (status = 200, description = "Task deleted successfully", body = BusinessResponse<String>),
        (status = 400, description = "Bad request", body = BusinessResponse<String>),
        (status = 404, description = "Task not found", body = BusinessResponse<String>),
        (status = 500, description = "Internal server error", body = BusinessResponse<String>)
    ),
    params(
        ("list_id" = String, Path, description = "Todo list unique identifier"),
        ("task_id" = String, Path, description = "Task unique identifier")
    ),
    tag = "Tasks"
)]
#[delete("/api/friday-todo-manager/lists/{list_id}/tasks/{task_id}")]
pub async fn delete_task(path: web::Path<(String, String)>) -> impl Responder {
    let (list_id, task_id) = path.into_inner();
    info!(
        "Controller layer: DELETE /api/friday-todo-manager/lists/{}/tasks/{}",
        list_id, task_id
    );

    match task_logic::delete_task(&list_id, &task_id).await {
        Ok(response) => {
            if response.success {
                info!("Controller layer: Successfully deleted task");
                HttpResponse::Ok().json(response)
            } else {
                if response.errors.iter().any(|e| e.contains("not found")) {
                    error!("Controller layer: Task not found: {:?}", response.errors);
                    HttpResponse::NotFound().json(response)
                } else {
                    error!(
                        "Controller layer: Business logic error: {:?}",
                        response.errors
                    );
                    HttpResponse::BadRequest().json(response)
                }
            }
        }
        Err(e) => {
            error!("Controller layer: System error deleting task: {}", e);
            HttpResponse::InternalServerError().json(BusinessResponse::<String>::error(
                "An internal error occurred while deleting the task",
            ))
        }
    }
}
