use std::env;

/// Configuration structure for the MCP server
#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub todo_api_base_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 5000,
            todo_api_base_url: "http://0.0.0.0:5000".to_string(),
        }
    }
}

/// Load environment configuration
/// Following LAW 2: Root module requirements
pub fn load_env() -> Config {
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT")
        .unwrap_or_else(|_| "5000".to_string())
        .parse::<u16>()
        .unwrap_or(5000);
    let todo_api_base_url =
        env::var("TODO_API_BASE_URL").unwrap_or_else(|_| "http://0.0.0.0:5000".to_string());

    Config {
        host,
        port,
        todo_api_base_url,
    }
}
