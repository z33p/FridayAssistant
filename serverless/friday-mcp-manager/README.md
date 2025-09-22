# Friday MCP Todo Manager

A simple HTTP Model Context Protocol (MCP) server built in Rust that provides todo management functionality.

## Overview

This MCP server exposes a simple todo management tool that can be used with VS Code and other MCP-compatible clients. It follows the Rust microservices architecture guidelines and implements the MCP protocol over HTTP.

## Features

- **Create Todo**: Create new todo items with title and optional description
- **List Todos**: Get all todo items with their current status
- **Get Todo**: Retrieve a specific todo item by ID
- **Update Todo**: Update todo title, description, or completion status
- **Delete Todo**: Remove a todo item by ID

## Architecture

This project follows the FridayAssistant Rust microservices architecture:

- `src/main.rs` - Application entry point
- `src/business_response.rs` - Universal response structure
- `src/load_env.rs` - Environment configuration
- `src/mcp_protocol.rs` - MCP JSON-RPC protocol structures
- `src/server.rs` - HTTP server and MCP endpoints
- `src/todo_mod/` - Todo domain module
  - `mod.rs` - Module definition
  - `todo_list.rs` - Data models and DTOs
  - `todo_logic.rs` - Business logic
  - `todo_controller.rs` - MCP tool handlers

## Running the Server

1. Build the project:

   ```bash
   cargo build
   ```

2. Run the server:

   ```bash
   cargo run
   ```

The server will start on `http://localhost:5000` by default.

## Configuration

Environment variables:

- `HOST` - Server host (default: localhost)
- `PORT` - Server port (default: 5000)

## VS Code Integration

To use this MCP server with VS Code, add the following configuration to your MCP settings:

```json
{
  "servers": {
    "friday-todo-mcp": {
      "url": "http://localhost:5000/",
      "type": "http"
    }
  },
  "inputs": []
}
```

## MCP Protocol

The server implements the following MCP methods:

### initialize

Initialize the MCP connection and exchange capabilities.

### tools/list

List all available tools (todo management operations).

### tools/call

Execute a specific tool with provided arguments.

## Available Tools

1. **create_todo**
   - Create a new todo item
   - Arguments: `title` (required), `description` (optional)

2. **list_todos**
   - List all todo items
   - Arguments: none

3. **get_todo**
   - Get a specific todo item
   - Arguments: `id` (required)

4. **update_todo**
   - Update an existing todo item
   - Arguments: `id` (required), `title`, `description`, `completed` (all optional)

5. **delete_todo**
   - Delete a todo item
   - Arguments: `id` (required)

## Testing

You can test the server using curl:

```bash
# Initialize
curl -X POST http://localhost:5000/ \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "initialize",
    "params": {
      "protocolVersion": "2024-11-05",
      "capabilities": {"tools": {"listChanged": false}},
      "clientInfo": {"name": "test-client", "version": "1.0.0"}
    }
  }'

# List tools
curl -X POST http://localhost:5000/ \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc": "2.0", "id": 2, "method": "tools/list"}'

# Create a todo
curl -X POST http://localhost:5000/ \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 3,
    "method": "tools/call",
    "params": {
      "name": "create_todo",
      "arguments": {"title": "My Todo", "description": "Todo description"}
    }
  }'
```

## Dependencies

- `tokio` - Async runtime
- `axum` - HTTP web framework
- `serde` & `serde_json` - JSON serialization
- `tower-http` - HTTP middleware (CORS)
- `tracing` - Structured logging
- `uuid` - UUID generation
- `chrono` - Date/time handling
