pub mod common;

// Tools MCP
pub mod rustwork_get_conventions;
pub mod rustwork_get_diagnostics;
pub mod rustwork_get_env_setup;
pub mod rustwork_get_file_doc;
pub mod rustwork_get_function_usage;
pub mod rustwork_get_route_impact;
pub mod rustwork_get_routes;

// Tools MCP Data & Architecture (v0.6.0)
pub mod rustwork_get_database_schema;
pub mod rustwork_get_models;
pub mod rustwork_get_services_overview;

// Tools MCP gRPC (P2)
pub mod rustwork_grpc_get_service_status;
pub mod rustwork_grpc_list_definitions;
pub mod rustwork_grpc_test_connectivity;

// Tools MCP gRPC (FINAL)
pub mod rustwork_grpc_detect_drift;
pub mod rustwork_grpc_diff_versions;
pub mod rustwork_grpc_get_call_graph;
pub mod rustwork_grpc_validate_workspace;

// Legacy (conservé temporairement)
pub mod tools;

pub use common::server::{run_server, run_stdio_server};
