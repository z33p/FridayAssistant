use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct BusinessResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub errors: Vec<String>,
}

impl<T> BusinessResponse<T> {
    pub fn success(data: T) -> Self {
        BusinessResponse {
            success: true,
            data: Some(data),
            errors: vec![],
        }
    }

    pub fn error(message: &str) -> Self {
        BusinessResponse {
            success: false,
            data: None,
            errors: vec![message.to_string()],
        }
    }
}
