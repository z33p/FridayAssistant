use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    business_response::BusinessResponse,
    todo_list_mod::todo_list::{
        CreateTodoListRequest, DeleteTodoListRequest, TodoList, UpdateTodoListRequest,
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
        crate::todo_list_mod::todo_list_controller::get_todo_list,
        crate::todo_list_mod::todo_list_controller::get_all_todo_lists,
        crate::todo_list_mod::todo_list_controller::create_todo_list,
        crate::todo_list_mod::todo_list_controller::update_todo_list,
        crate::todo_list_mod::todo_list_controller::delete_todo_list,
    ),
    components(schemas(
        BusinessResponse<TodoList>,
        BusinessResponse<Vec<TodoList>>,
        BusinessResponse<String>,
        TodoList,
        CreateTodoListRequest,
        UpdateTodoListRequest,
        DeleteTodoListRequest,
    )),
    tags(
        (name = "Todo Lists", description = "Todo list management endpoints"),
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
