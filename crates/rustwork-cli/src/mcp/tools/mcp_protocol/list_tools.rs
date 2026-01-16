use crate::mcp::common::protocol::RpcError;
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
                "description": "Get Rustwork framework conventions in a hierarchical, explorable way. Project conventions override framework conventions.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "category": {
                            "type": "string",
                            "description": "Category ID to explore (e.g., 'database', 'http'). Returns subcategories without atomic rules."
                        },
                        "path": {
                            "type": "string",
                            "description": "Full path to a specific convention (e.g., 'database.migrations.naming'). Returns the exact convention with rules and examples."
                        }
                    }
                }
            },
            {
                "name": "rustwork_get_env_setup",
                "description": "Analyze environment variable configuration (.env files) and detect port conflicts in microservices architecture",
                "inputSchema": {
                    "type": "object",
                    "properties": {}
                }
            },
            {
                "name": "rustwork_grpc_list_definitions",
                "description": "Get complete gRPC cartography: scan .rwk files, list services/RPCs/messages, and identify inter-service dependencies",
                "inputSchema": {
                    "type": "object",
                    "properties": {}
                }
            },
            {
                "name": "rustwork_grpc_get_service_status",
                "description": "Get the real state of a gRPC service: check .rwk file, generated code, and detect inconsistencies",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "service_name": {
                            "type": "string",
                            "description": "Name of the gRPC service to check"
                        }
                    },
                    "required": ["service_name"]
                }
            },
            {
                "name": "rustwork_grpc_test_connectivity",
                "description": "Test real gRPC connectivity to a service: TCP connection, latency measurement, clear error reporting",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "service_name": {
                            "type": "string",
                            "description": "Name of the gRPC service to test"
                        },
                        "address": {
                            "type": "string",
                            "description": "Target address (e.g., '127.0.0.1:50051')"
                        },
                        "timeout_ms": {
                            "type": "number",
                            "description": "Connection timeout in milliseconds (default: 5000)"
                        }
                    },
                    "required": ["service_name", "address"]
                }
            },
            {
                "name": "rustwork_grpc_validate_workspace",
                "description": "Validate global gRPC workspace coherence: detect orphaned services/RPCs, undefined dependencies, and circular dependencies",
                "inputSchema": {
                    "type": "object",
                    "properties": {}
                }
            },
            {
                "name": "rustwork_grpc_get_call_graph",
                "description": "Get complete inter-service gRPC call graph: map dependencies, identify central services, entrypoints, and isolated services",
                "inputSchema": {
                    "type": "object",
                    "properties": {}
                }
            },
            {
                "name": "rustwork_grpc_detect_drift",
                "description": "Detect gRPC desynchronizations: missing .proto, outdated generated code, obsolete build.rs, DSL vs filesystem drift",
                "inputSchema": {
                    "type": "object",
                    "properties": {}
                }
            },
            {
                "name": "rustwork_grpc_diff_versions",
                "description": "Compare two versions of a .rwk file to detect breaking changes in gRPC contracts. Classifies changes as breaking/compatible and generates migration suggestions.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "rwk_path": {
                            "type": "string",
                            "description": "Relative path to the .rwk file to compare (e.g., 'services/user/grpc/user.rwk')"
                        },
                        "compare_with": {
                            "type": "string",
                            "description": "Git reference to compare with. Allowed values: 'main', 'commit:<sha>', 'tag:<name>'"
                        }
                    },
                    "required": ["rwk_path", "compare_with"]
                }
            },
            {
                "name": "rustwork_get_database_schema",
                "description": "Get actual database schema for a service: tables, columns, indexes, foreign keys. Introspects real database (SQLite, PostgreSQL, MySQL). CRITICAL: Returns exact data or fails explicitly.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "service": {
                            "type": "string",
                            "description": "Service name to introspect database for"
                        }
                    },
                    "required": ["service"]
                }
            },
            {
                "name": "rustwork_get_models",
                "description": "Get all Rust models/DTOs in the project: entities, request/response types, domain models. Parses source code to identify structs with fields, derives, and relations.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "service": {
                            "type": "string",
                            "description": "Optional: filter by service name. If omitted, scans all services."
                        }
                    }
                }
            },
            {
                "name": "rustwork_get_services_overview",
                "description": "Get architectural overview of all services: ports, responsibilities, routes count, models count, database usage, dependencies. Provides macro view of entire workspace.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "service": {
                            "type": "string",
                            "description": "Optional: filter by service name. If omitted, returns all services."
                        }
                    }
                }
            }
        ]
    }))
}
