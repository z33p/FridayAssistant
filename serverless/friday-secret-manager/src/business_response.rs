use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct Response<T> {
    pub success: bool,
    pub data: Option<T>,
    pub errors: Vec<String>,
}

impl<T> Response<T> {
    pub fn new(success: bool, data: Option<T>, errors: Vec<String>) -> Self {
        Response {
            success,
            data,
            errors,
        }
    }
}
