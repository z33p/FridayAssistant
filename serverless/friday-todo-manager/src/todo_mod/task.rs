use crate::microsoft_graph_mod::task_response::TaskResponse;
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema, Clone)]
#[schema(title = "Task")]
pub struct Task {
    #[schema(
        example = "AAMkAGVmMDEzMTM4LTZmYWUtNDdkNC1hMDZiLTU1OGY5OTZhNGY2NAAuAAAAAAAiQ8W967B7TKBjgx9rVEURBwAiIsqMbYjsT5e-T_KzowKTAAAAAAEKAAA"
    )]
    pub id: String,

    #[schema(example = "Buy groceries")]
    pub title: String,

    #[schema(example = "Milk, bread, eggs, and vegetables")]
    pub body: Option<String>,

    pub status: TaskStatus,
    pub importance: TaskImportance,

    #[schema(example = true)]
    pub is_reminder_on: bool,

    pub reminder_date_time: Option<DateTime<Utc>>,
    pub due_date_time: Option<DateTime<Utc>>,

    #[serde(with = "chrono::serde::ts_milliseconds")]
    #[schema(example = "1699876543000")]
    pub created_date_time: DateTime<Utc>,

    #[serde(with = "chrono::serde::ts_milliseconds")]
    #[schema(example = "1699876543000")]
    pub last_modified_date_time: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Clone)]
#[schema(title = "TaskStatus")]
pub enum TaskStatus {
    #[serde(rename = "notStarted")]
    NotStarted,
    #[serde(rename = "inProgress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "waitingOnOthers")]
    WaitingOnOthers,
    #[serde(rename = "deferred")]
    Deferred,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Clone)]
#[schema(title = "TaskImportance")]
pub enum TaskImportance {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "high")]
    High,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(title = "CreateTaskRequest")]
pub struct CreateTaskRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,

    #[schema(example = "Buy groceries", min_length = 1, max_length = 255)]
    pub title: String,

    #[schema(example = "Milk, bread, eggs, and vegetables")]
    pub body: Option<String>,

    pub importance: Option<TaskImportance>,

    #[schema(example = true)]
    pub is_reminder_on: Option<bool>,

    pub reminder_date_time: Option<DateTime<Utc>>,
    pub due_date_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(title = "CreateTaskRequestBody")]
pub struct CreateTaskRequestBody {
    #[schema(example = "Buy groceries", min_length = 1, max_length = 255)]
    pub title: String,

    #[schema(example = "Milk, bread, eggs, and vegetables")]
    pub body: Option<String>,

    pub importance: Option<TaskImportance>,

    #[schema(example = true)]
    pub is_reminder_on: Option<bool>,

    pub reminder_date_time: Option<DateTime<Utc>>,
    pub due_date_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(title = "UpdateTaskRequest")]
pub struct UpdateTaskRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,

    #[schema(example = "Updated task title", min_length = 1, max_length = 255)]
    pub title: Option<String>,

    #[schema(example = "Updated task description")]
    pub body: Option<String>,

    pub status: Option<TaskStatus>,
    pub importance: Option<TaskImportance>,

    #[schema(example = false)]
    pub is_reminder_on: Option<bool>,

    pub reminder_date_time: Option<DateTime<Utc>>,
    pub due_date_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(title = "UpdateTaskRequestBody")]
pub struct UpdateTaskRequestBody {
    #[schema(example = "Updated task title", min_length = 1, max_length = 255)]
    pub title: Option<String>,

    #[schema(example = "Updated task description")]
    pub body: Option<String>,

    pub status: Option<TaskStatus>,
    pub importance: Option<TaskImportance>,

    #[schema(example = false)]
    pub is_reminder_on: Option<bool>,

    pub reminder_date_time: Option<DateTime<Utc>>,
    pub due_date_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(title = "DeleteTaskRequest")]
pub struct DeleteTaskRequest {
    #[schema(
        example = "AAMkAGVmMDEzMTM4LTZmYWUtNDdkNC1hMDZiLTU1OGY5OTZhNGY2NAAuAAAAAAAiQ8W967B7TKBjgx9rVEURBwAiIsqMbYjsT5e-T_KzowKTAAAAAAEKAAA"
    )]
    pub id: String,

    #[schema(
        example = "AAMkAGVmMDEzMTM4LTZmYWUtNDdkNC1hMDZiLTU1OGY5OTZhNGY2NAAuAAAAAAAiQ8W967B7TKBjgx9rVEURBwAiIsqMbYjsT5e-T_KzowKTAAAAAAESAAA"
    )]
    pub list_id: String,
}

impl From<TaskResponse> for Task {
    fn from(response: TaskResponse) -> Self {
        let default_time = Utc::now();

        let created_date_time = DateTime::parse_from_rfc3339(&response.created_date_time)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or(default_time);

        let last_modified_date_time =
            DateTime::parse_from_rfc3339(&response.last_modified_date_time)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or(default_time);

        let status = match response.status.as_str() {
            "notStarted" => TaskStatus::NotStarted,
            "inProgress" => TaskStatus::InProgress,
            "completed" => TaskStatus::Completed,
            "waitingOnOthers" => TaskStatus::WaitingOnOthers,
            "deferred" => TaskStatus::Deferred,
            _ => TaskStatus::NotStarted,
        };

        let importance = match response.importance.as_str() {
            "low" => TaskImportance::Low,
            "normal" => TaskImportance::Normal,
            "high" => TaskImportance::High,
            _ => TaskImportance::Normal,
        };

        let body = response.body.map(|b| b.content);

        let reminder_date_time = response
            .reminder_date_time
            .and_then(|rdt| DateTime::parse_from_rfc3339(&rdt.date_time).ok())
            .map(|dt| dt.with_timezone(&Utc));

        let due_date_time = response
            .due_date_time
            .and_then(|ddt| DateTime::parse_from_rfc3339(&ddt.date_time).ok())
            .map(|dt| dt.with_timezone(&Utc));

        Task {
            id: response.id,
            title: response.title,
            body,
            status,
            importance,
            is_reminder_on: response.is_reminder_on,
            reminder_date_time,
            due_date_time,
            created_date_time,
            last_modified_date_time,
        }
    }
}
