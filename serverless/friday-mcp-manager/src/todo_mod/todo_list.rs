use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// LAW 5: Struct naming standards - PascalCase
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoItem {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: String,
}

/// Request DTO for creating a todo
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTodoRequest {
    pub title: String,
    pub description: Option<String>,
}

/// Request DTO for updating a todo
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTodoRequest {
    pub id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
}

/// Response DTO for todo operations
#[derive(Debug, Serialize, Deserialize)]
pub struct TodoResponse {
    pub todo: TodoItem,
}

/// Response DTO for listing todos
#[derive(Debug, Serialize, Deserialize)]
pub struct TodoListResponse {
    pub todos: Vec<TodoItem>,
    pub total: usize,
}

impl TodoItem {
    /// LAW 4: Function naming - descriptive verbs
    pub fn create_new(title: String, description: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            description,
            completed: false,
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Update todo item fields
    pub fn update_fields(
        &mut self,
        title: Option<String>,
        description: Option<String>,
        completed: Option<bool>,
    ) {
        if let Some(new_title) = title {
            self.title = new_title;
        }
        if let Some(new_description) = description {
            self.description = Some(new_description);
        }
        if let Some(new_completed) = completed {
            self.completed = new_completed;
        }
    }
}
