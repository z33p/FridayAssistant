use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub body: Option<String>,
    pub status: String,
    pub importance: String,
    pub is_reminder_on: bool,
    pub created_date_time: i64,           // Unix timestamp as integer
    pub last_modified_date_time: i64,     // Unix timestamp as integer
    pub completed_date_time: Option<i64>, // Unix timestamp as integer
    pub due_date_time: Option<i64>,       // Unix timestamp as integer
    pub list_id: String,
}

/// Request DTO for creating a task
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTaskRequest {
    pub title: String,
    pub body: Option<String>,
    pub importance: Option<String>,
    pub is_reminder_on: Option<bool>,
    pub due_date_time: Option<i64>,
    pub list_id: String,
}

/// Request DTO for updating a task
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTaskRequest {
    pub id: String,
    pub title: Option<String>,
    pub body: Option<String>,
    pub status: Option<String>,
    pub importance: Option<String>,
    pub is_reminder_on: Option<bool>,
    pub due_date_time: Option<i64>,
    pub list_id: String,
}

/// Response DTO for task operations
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskResponse {
    pub task: Task,
}

/// Response DTO for listing tasks
#[derive(Debug, Serialize, Deserialize)]
pub struct TasksResponse {
    pub tasks: Vec<Task>,
    pub total: usize,
}
