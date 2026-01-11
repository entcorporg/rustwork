# VS Code MCP Server Integration

## Overview

Rustwork provides native MCP (Model Context Protocol) server integration for VS Code Copilot, enabling intelligent code assistance with deep framework knowledge.

## What It Does

The Rustwork MCP server gives VS Code Copilot real-time access to:

- **Live Route Map**: All API endpoints with their handlers and methods
- **Function Call Graph**: Understand code dependencies and impact analysis
- **File Documentation**: Instant access to functions, structs, and their signatures
- **Diagnostics**: Compilation errors, warnings, and lint suggestions
- **Framework Conventions**: Rustwork patterns for handlers, errors, and responses

## When to Use

Copilot will automatically use Rustwork MCP tools when you:

- Ask about routes: "What routes use this function?"
- Explore code: "Show me the handlers in this file"
- Analyze impact: "What will break if I change this?"
- Debug issues: "What are the current errors?"
- Learn patterns: "How do I create a handler?"

## Setup

### 1. Install Rustwork CLI

```bash
cargo install rustwork-cli
```

Or build from source:

```bash
cd crates/rustwork-cli
cargo build --release
# Add target/release/rustwork to your PATH
```

### 2. Configure VS Code

Create `.vscode/mcp.json` in your Rustwork project:

```json
{
  "servers": {
    "rustwork": {
      "type": "stdio",
      "command": "rustwork",
      "args": ["mcp", "--stdio", "--project", "."]
    }
  },
  "inputs": []
}
```

**Important**: 
- Replace `"."` with the absolute path to your project if needed
- Ensure `rustwork` is in your PATH

### 3. Restart VS Code

Reload the window to activate the MCP server.

## Available Tools

### rustwork_get_routes

Get all routes in your project.

**Example**: "Show me all POST routes"

**Returns**:
```json
{
  "routes": [
    {
      "method": "POST",
      "path": "/api/users",
      "handler": "create_user",
      "file": "src/controllers/user_controller.rs",
      "line": 42
    }
  ],
  "count": 1
}
```

### rustwork_get_file_doc

Get documentation for a specific file.

**Parameters**: `{ "path": "src/controllers/user_controller.rs" }`

**Example**: "What functions are in user_controller?"

**Returns**:
```json
{
  "path": "src/controllers/user_controller.rs",
  "module": "controllers::user_controller",
  "functions": [
    {
      "name": "create_user",
      "signature": "async fn create_user(...) -> Result<ApiResponse<User>, AppError>",
      "line": 42,
      "is_public": true,
      "is_async": true
    }
  ],
  "structs": [...]
}
```

### rustwork_get_function_usage

Find where a function is used.

**Parameters**: `{ "function": "create_user" }`

**Example**: "Where is create_user called?"

**Returns**:
```json
{
  "function": "create_user",
  "callers": ["main::setup_routes", "tests::test_create_user"],
  "used_by_routes": [
    {
      "method": "POST",
      "path": "/api/users",
      "file": "src/main.rs",
      "line": 25
    }
  ]
}
```

### rustwork_get_route_impact

Analyze the impact of a route change.

**Parameters**: `{ "method": "POST", "path": "/api/users" }`

**Example**: "What functions does POST /api/users call?"

**Returns**:
```json
{
  "route": {
    "method": "POST",
    "path": "/api/users",
    "handler": "create_user",
    "file": "src/controllers/user_controller.rs",
    "line": 42
  },
  "called_functions": {
    "user_service::validate_user": 1,
    "user_repository::insert_user": 1
  },
  "affected_files": [
    "src/controllers/user_controller.rs",
    "src/services/user_service.rs",
    "src/repositories/user_repository.rs"
  ]
}
```

### rustwork_get_diagnostics

Get current compilation and lint errors.

**Example**: "What errors are in my code?"

**Returns**:
```json
{
  "errors": 2,
  "warnings": 5,
  "total": 7,
  "last_build_success": false,
  "diagnostics": [
    {
      "level": "error",
      "message": "cannot find value `user` in this scope",
      "file": "src/controllers/user_controller.rs",
      "line": 50,
      "column": 12
    }
  ]
}
```

### rustwork_get_conventions

Learn Rustwork framework patterns.

**Example**: "How do I write a handler?" or "What's the error pattern?"

**Returns**:
```json
{
  "error_handling": {
    "type": "AppError",
    "file": "src/errors.rs",
    "variants": ["NotFound", "BadRequest", "Unauthorized", "InternalError", "DatabaseError"],
    "pattern": "Result<ApiResponse<T>, AppError>"
  },
  "handler_patterns": {
    "basic": "async fn handler(State(state): State<AppState>) -> Result<ApiResponse<T>, AppError>",
    "with_json": "async fn handler(State(state): State<AppState>, Json(payload): Json<P>) -> Result<ApiResponse<T>, AppError>"
  }
}
```

## Architecture

### Transport: stdio

- **stdin**: JSON-RPC 2.0 requests (one per line)
- **stdout**: JSON-RPC 2.0 responses (one per line)
- **stderr**: Server logs and diagnostics

### Live Indexing

The MCP server maintains a live index of your project:

- **Code Index**: Functions, structs, call graphs (via `syn` parser)
- **Route Registry**: All Axum routes extracted from router code
- **Diagnostics**: Real-time `cargo check` output
- **File Watcher**: Automatic re-indexing on file changes

### No Framework Dependency

The MCP server runs **independently** of your Rustwork application. It doesn't require your dev server to be running and works entirely through static analysis.

## Troubleshooting

### Server Not Starting

Check logs in VS Code Output panel:
1. View → Output
2. Select "GitHub Copilot" from dropdown
3. Look for MCP server logs

### No Responses

Ensure `rustwork` is in your PATH:

```bash
which rustwork
# Should output: /path/to/rustwork
```

### Stale Data

The server auto-updates on file changes, but you can restart it:
1. Command Palette (Ctrl+Shift+P)
2. "Developer: Reload Window"

### Debug Mode

Run manually to see detailed logs:

```bash
cd your-rustwork-project
rustwork mcp --stdio
```

Then send test requests (see `examples/mcp_test_client.py`).

## Performance

- **Initial Scan**: ~1-5 seconds (depends on project size)
- **File Watch**: Incremental updates in <500ms
- **Query Response**: <100ms for most operations

## Security

- **Localhost Only**: MCP server binds to `127.0.0.1` (cannot be accessed remotely)
- **No Network Access**: Operates entirely on local filesystem
- **Sanitized Output**: Secrets (passwords, tokens) are automatically redacted

## Examples

See `examples/` directory for:
- `mcp_client.py`: Python client for testing
- `demo_mcp.sh`: Shell script demonstrating all tools

## Related Documentation

- [MCP Protocol](./MCP.md): Technical specification
- [Quick Reference](../QUICKREF.md): Rustwork framework patterns
- [Examples](../EXAMPLES.md): Code samples
