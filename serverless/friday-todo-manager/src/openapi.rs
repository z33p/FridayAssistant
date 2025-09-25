use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    business_response::BusinessResponse,
    todo_mod::{
        task::{
            CreateTaskRequest, DeleteTaskRequest, Task, TaskImportance, TaskStatus,
            UpdateTaskRequest,
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
        description = "Todo list management service integrating with Microsoft Graph API",
        version = "1.0.0",
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
        UpdateTaskRequest,
        DeleteTaskRequest,
    )),
    tags(
        (name = "Todo Lists", description = "Todo list management endpoints"),
        (name = "Tasks", description = "Task management endpoints"),
        (name = "Operations", description = "Administrative operations")
    )
)]
pub struct ApiDoc;

pub fn swagger_config() -> SwaggerUi {
    SwaggerUi::new("/api/friday-todo-manager/swagger/{_:.*}").url(
        "/api/friday-todo-manager/api-docs/openapi.json",
        ApiDoc::openapi(),
    )
}
