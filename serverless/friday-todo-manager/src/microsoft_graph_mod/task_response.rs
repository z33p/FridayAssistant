use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TaskResponse {
    pub id: String,
    pub title: String,
    pub body: Option<TaskBodyResponse>,
    pub status: String,
    pub importance: String,
    #[serde(rename = "isReminderOn")]
    pub is_reminder_on: bool,
    #[serde(rename = "reminderDateTime")]
    pub reminder_date_time: Option<TaskDateTimeResponse>,
    #[serde(rename = "dueDateTime")]
    pub due_date_time: Option<TaskDateTimeResponse>,
    #[serde(rename = "createdDateTime")]
    pub created_date_time: String,
    #[serde(rename = "lastModifiedDateTime")]
    pub last_modified_date_time: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TaskBodyResponse {
    pub content: String,
    #[serde(rename = "contentType")]
    pub content_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TaskDateTimeResponse {
    #[serde(rename = "dateTime")]
    pub date_time: String,
    #[serde(rename = "timeZone")]
    pub time_zone: String,
}