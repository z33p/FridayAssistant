use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    business_response::Response,
    todo_mod::todo_list::{
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
        crate::todo_controller::get_todo_list,
        crate::todo_controller::get_all_todo_lists,
        crate::todo_controller::create_todo_list,
        crate::todo_controller::update_todo_list,
        crate::todo_controller::delete_todo_list,
    ),
    components(schemas(
        Response<TodoList>,
        Response<Vec<TodoList>>,
        Response<String>,
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
    SwaggerUi::new("/swagger/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi())
}
