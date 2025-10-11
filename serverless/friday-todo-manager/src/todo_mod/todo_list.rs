use crate::microsoft_graph_mod::todo_list_response::TodoListResponse;
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema, Clone)]
#[schema(title = "TodoList")]
pub struct TodoList {
    #[schema(
        example = "AAMkAGVmMDEzMTM4LTZmYWUtNDdkNC1hMDZiLTU1OGY5OTZhNGY2NAAuAAAAAAAiQ8W967B7TKBjgx9rVEURBwAiIsqMbYjsT5e-T_KzowKTAAAAAAESAAA"
    )]
    pub id: String,

    #[schema(example = "Shopping List")]
    pub display_name: String,

    #[schema(example = true)]
    pub is_owner: bool,

    #[schema(example = false)]
    pub is_shared: bool,

    #[schema(example = "defaultList")]
    pub wellknown_list_name: Option<String>,

    #[serde(with = "chrono::serde::ts_milliseconds")]
    #[schema(example = "1699876543000")]
    pub created_date_time: DateTime<Utc>,

    #[serde(with = "chrono::serde::ts_milliseconds")]
    #[schema(example = "1699876543000")]
    pub last_modified_date_time: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(title = "CreateTodoListRequest")]
pub struct CreateTodoListRequest {
    #[schema(example = "Shopping List", min_length = 1, max_length = 255)]
    pub display_name: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(title = "UpdateTodoListRequest")]
pub struct UpdateTodoListRequest {
    #[schema(
        example = "AAMkAGVmMDEzMTM4LTZmYWUtNDdkNC1hMDZiLTU1OGY5OTZhNGY2NAAuAAAAAAAiQ8W967B7TKBjgx9rVEURBwAiIsqMbYjsT5e-T_KzowKTAAAAAAESAAA"
    )]
    pub id: String,

    #[schema(example = "Updated Shopping List", min_length = 1, max_length = 255)]
    pub display_name: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(title = "DeleteTodoListRequest")]
pub struct DeleteTodoListRequest {
    #[schema(
        example = "AAMkAGVmMDEzMTM4LTZmYWUtNDdkNC1hMDZiLTU1OGY5OTZhNGY2NAAuAAAAAAAiQ8W967B7TKBjgx9rVEURBwAiIsqMbYjsT5e-T_KzowKTAAAAAAESAAA"
    )]
    pub id: String,
}

impl From<TodoListResponse> for TodoList {
    fn from(response: TodoListResponse) -> Self {
        let default_time = Utc::now();

        let created_date_time = response
            .created_date_time
            .as_ref()
            .and_then(|dt| DateTime::parse_from_rfc3339(dt).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or(default_time);

        let last_modified_date_time = response
            .last_modified_date_time
            .as_ref()
            .and_then(|dt| DateTime::parse_from_rfc3339(dt).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or(default_time);

        TodoList {
            id: response.id,
            display_name: response.display_name,
            is_owner: response.is_owner,
            is_shared: response.is_shared,
            wellknown_list_name: response.wellknown_list_name,
            created_date_time,
            last_modified_date_time,
        }
    }
}
