use std::env;

/// Configuration structure for the MCP server
#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 5000,
        }
    }
}

/// Load environment configuration
/// Following LAW 2: Root module requirements
pub fn load_env() -> Config {
    let host = env::var("HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = env::var("PORT")
        .unwrap_or_else(|_| "5000".to_string())
        .parse::<u16>()
        .unwrap_or(5000);

    Config { host, port }
}
