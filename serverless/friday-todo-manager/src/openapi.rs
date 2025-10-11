use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    business_response::BusinessResponse,
    todo_mod::{
        task::{
            CreateTaskRequest, CreateTaskRequestBody, DeleteTaskRequest, Task, TaskImportance,
            TaskStatus, UpdateTaskRequest, UpdateTaskRequestBody,
        },
        todo_list::{
            CreateTodoListRequest, DeleteTodoListRequest, TodoList, UpdateTodoListRequest,
        },
    },
};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Friday Todo Manager API",
        description = r#"
A comprehensive todo list management service that integrates with Microsoft Graph API.

This API allows you to:
- Create, read, update, and delete todo lists
- Manage tasks within todo lists
- Track task status, importance, and due dates
- Set reminders for tasks
- Synchronize with Microsoft To-Do through Graph API

Perfect for AI agents and automation systems that need to manage personal or team productivity.

## Authentication
This service requires valid OAuth tokens for Microsoft Graph API access.

## Response Format
All endpoints return a consistent BusinessResponse format with success indicators and error handling.
        "#,
        version = "1.0.0",
        contact(
            name = "Friday Assistant",
            email = "support@friday-assistant.com"
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        )
    ),
    servers(
        (url = "https://k8s.z33p.com", description = "Production server")
    ),
    paths(
        crate::todo_mod::todo_list_controller::get_todo_list,
        crate::todo_mod::todo_list_controller::get_all_todo_lists,
        crate::todo_mod::todo_list_controller::create_todo_list,
        crate::todo_mod::todo_list_controller::update_todo_list,
        crate::todo_mod::todo_list_controller::delete_todo_list,
        crate::todo_mod::task_controller::get_all_tasks,
        crate::todo_mod::task_controller::get_task,
        crate::todo_mod::task_controller::create_task,
        crate::todo_mod::task_controller::update_task,
        crate::todo_mod::task_controller::delete_task,
    ),
    components(schemas(
        BusinessResponse<TodoList>,
        BusinessResponse<Vec<TodoList>>,
        BusinessResponse<Task>,
        BusinessResponse<Vec<Task>>,
        BusinessResponse<String>,
        TodoList,
        CreateTodoListRequest,
        UpdateTodoListRequest,
        DeleteTodoListRequest,
        Task,
        TaskStatus,
        TaskImportance,
        CreateTaskRequest,
        CreateTaskRequestBody,
        UpdateTaskRequest,
        UpdateTaskRequestBody,
        DeleteTaskRequest,
    )),
    tags(
        (name = "Todo Lists", description = "Operations for managing todo lists. Use these endpoints to organize tasks into collections."),
        (name = "Tasks", description = "Operations for managing individual tasks within todo lists. Control task lifecycle, status, and metadata."),
        (name = "Operations", description = "Administrative and utility operations for system management.")
    )
)]
pub struct ApiDoc;

pub fn swagger_config() -> SwaggerUi {
    SwaggerUi::new("/api/friday-todo-manager/swagger/{_:.*}").url(
        "/api/friday-todo-manager/api-docs/openapi.json",
        ApiDoc::openapi(),
    )
}
