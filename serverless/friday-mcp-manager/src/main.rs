// LAW 2: Root module requirements
mod business_response;
mod load_env;
mod mcp_protocol;
mod server;
mod todo_mod;

use tracing::{Level, info};
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    // Load configuration
    let config = load_env::load_env();
    info!("Starting Friday MCP Todo Server");
    info!("Configuration: {:?}", config);

    // Create the HTTP router
    let app = server::create_router();

    // Bind to the configured address
    let bind_addr = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .expect("Failed to bind to address");

    info!("Server listening on http://{}", bind_addr);
    info!("MCP endpoint available at: http://{}/", bind_addr);

    // Start the server
    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}
