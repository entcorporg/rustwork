use crate::mcp::protocol::RpcError;
use serde_json::{json, Value};

/// MCP tools/list - List available tools
pub async fn mcp_list_tools() -> Result<Value, RpcError> {
    Ok(json!({
        "tools": [
            {
                "name": "rustwork_get_routes",
                "description": "Get all API routes in the Rustwork project",
                "inputSchema": {
                    "type": "object",
                    "properties": {}
                }
            },
            {
                "name": "rustwork_get_file_doc",
                "description": "Get documentation for a specific file (functions, structs)",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Relative path to the file (e.g., 'src/main.rs')"
                        }
                    },
                    "required": ["path"]
                }
            },
            {
                "name": "rustwork_get_function_usage",
                "description": "Find where a function is called and which routes use it",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "function": {
                            "type": "string",
                            "description": "Function name to search for"
                        }
                    },
                    "required": ["function"]
                }
            },
            {
                "name": "rustwork_get_route_impact",
                "description": "Analyze the impact of a route (called functions, affected files)",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "method": {
                            "type": "string",
                            "description": "HTTP method (GET, POST, PUT, PATCH, DELETE)"
                        },
                        "path": {
                            "type": "string",
                            "description": "Route path (e.g., '/api/users')"
                        }
                    },
                    "required": ["method", "path"]
                }
            },
            {
                "name": "rustwork_get_diagnostics",
                "description": "Get current compilation errors and warnings",
                "inputSchema": {
                    "type": "object",
                    "properties": {}
                }
            },
            {
                "name": "rustwork_get_conventions",
                "description": "Get Rustwork framework conventions and patterns",
                "inputSchema": {
                    "type": "object",
                    "properties": {}
                }
            }
        ]
    }))
}
