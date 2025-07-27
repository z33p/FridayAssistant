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

    pub fn success(data: T) -> Self {
        Response {
            success: true,
            data: Some(data),
            errors: vec![],
        }
    }

    pub fn error(message: &str) -> Self {
        Response {
            success: false,
            data: None,
            errors: vec![message.to_string()],
        }
    }
}
