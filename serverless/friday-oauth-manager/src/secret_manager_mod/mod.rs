/// LAW 1: Domain module organization
pub mod secret;
pub mod secret_manager_api;
pub mod secret_manager_logic;
pub mod secret_response;

// Re-export for easier access
pub use secret_manager_logic::{get_database_url, get_oauth_credentials};
