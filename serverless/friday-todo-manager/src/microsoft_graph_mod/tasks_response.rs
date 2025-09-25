use crate::microsoft_graph_mod::task_response::TaskResponse;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TasksResponse {
    #[serde(rename = "@odata.context")]
    pub odata_context: Option<String>,
    pub value: Vec<TaskResponse>,
}