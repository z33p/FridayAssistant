use crate::microsoft_graph_mod::task_response::TaskResponse;
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema, Clone)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub body: Option<String>,
    pub status: TaskStatus,
    pub importance: TaskImportance,
    pub is_reminder_on: bool,
    pub reminder_date_time: Option<DateTime<Utc>>,
    pub due_date_time: Option<DateTime<Utc>>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub created_date_time: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub last_modified_date_time: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Clone)]
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
pub enum TaskImportance {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "high")]
    High,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateTaskRequest {
    pub list_id: String,
    pub title: String,
    pub body: Option<String>,
    pub importance: Option<TaskImportance>,
    pub is_reminder_on: Option<bool>,
    pub reminder_date_time: Option<DateTime<Utc>>,
    pub due_date_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UpdateTaskRequest {
    pub id: String,
    pub list_id: String,
    pub title: Option<String>,
    pub body: Option<String>,
    pub status: Option<TaskStatus>,
    pub importance: Option<TaskImportance>,
    pub is_reminder_on: Option<bool>,
    pub reminder_date_time: Option<DateTime<Utc>>,
    pub due_date_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct DeleteTaskRequest {
    pub id: String,
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
