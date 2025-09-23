use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
    pub success: bool,
    pub data: Option<T>,
    pub errors: Vec<String>,
}

impl<T> Response<T> {
    /// Create a successful response with data
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            errors: Vec::new(),
        }
    }

    /// Create a successful response without data
    pub fn success_empty() -> Self {
        Self {
            success: true,
            data: None,
            errors: Vec::new(),
        }
    }

    /// Create an error response with a single message
    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            errors: vec![message],
        }
    }

    /// Create an error response with multiple messages
    pub fn errors(messages: Vec<String>) -> Self {
        Self {
            success: false,
            data: None,
            errors: messages,
        }
    }
}
