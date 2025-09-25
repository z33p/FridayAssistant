use crate::microsoft_graph_mod::todo_list_response::TodoListResponse;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TodoListsResponse {
    #[serde(rename = "@odata.context")]
    pub odata_context: Option<String>,
    pub value: Vec<TodoListResponse>,
}