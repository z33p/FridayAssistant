use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// LAW 5: Struct naming standards - PascalCase
/// TodoList entity matching OpenAPI schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoList {
    pub id: String,
    pub display_name: String,
    pub is_owner: bool,
    pub is_shared: bool,
    pub created_date_time: i64,       // Unix timestamp as integer
    pub last_modified_date_time: i64, // Unix timestamp as integer
    pub wellknown_list_name: Option<String>,
}

/// Request DTO for creating a todo list
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTodoListRequest {
    pub display_name: String,
}

/// Request DTO for updating a todo list
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTodoListRequest {
    pub id: String,
    pub display_name: String,
}

/// Request DTO for deleting a todo list
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteTodoListRequest {
    pub id: String,
}

/// Response DTO for todo list operations
#[derive(Debug, Serialize, Deserialize)]
pub struct TodoListResponse {
    pub todo_list: TodoList,
}

/// Response DTO for listing todo lists
#[derive(Debug, Serialize, Deserialize)]
pub struct TodoListsResponse {
    pub todo_lists: Vec<TodoList>,
    pub total: usize,
}

/// Generic API response wrapper matching OpenAPI Response schema
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub errors: Vec<String>,
}

impl TodoList {
    /// LAW 4: Function naming - descriptive verbs
    pub fn create_new(display_name: String) -> Self {
        let now = chrono::Utc::now().timestamp_millis();
        Self {
            id: Uuid::new_v4().to_string(),
            display_name,
            is_owner: true,
            is_shared: false,
            created_date_time: now,
            last_modified_date_time: now,
            wellknown_list_name: None,
        }
    }

    /// Update todo list fields
    pub fn update_display_name(&mut self, display_name: String) {
        self.display_name = display_name;
        self.last_modified_date_time = chrono::Utc::now().timestamp_millis();
    }
}
