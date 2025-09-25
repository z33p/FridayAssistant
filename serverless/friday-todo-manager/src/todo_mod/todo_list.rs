use crate::microsoft_graph_mod::todo_list_response::TodoListResponse;
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema, Clone)]
pub struct TodoList {
    pub id: String,
    pub display_name: String,
    pub is_owner: bool,
    pub is_shared: bool,
    pub wellknown_list_name: Option<String>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub created_date_time: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub last_modified_date_time: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateTodoListRequest {
    pub display_name: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UpdateTodoListRequest {
    pub id: String,
    pub display_name: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct DeleteTodoListRequest {
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
