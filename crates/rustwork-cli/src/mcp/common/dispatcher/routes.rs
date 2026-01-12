use crate::mcp::common::protocol::RpcError;
use crate::mcp::common::state::LiveProjectState;
use crate::mcp::tools::{legacy, mcp_protocol};
use crate::mcp::{
    rustwork_get_conventions, rustwork_get_database_schema, rustwork_get_diagnostics,
    rustwork_get_env_setup, rustwork_get_file_doc, rustwork_get_function_usage,
    rustwork_get_models, rustwork_get_route_impact, rustwork_get_routes,
    rustwork_get_services_overview,
};
use serde_json::{json, Value};

/// Route MCP protocol standard methods
pub async fn route_mcp_protocol(
    method: &str,
    params: &Option<Value>,
    state: Option<&LiveProjectState>,
) -> Option<Result<Value, RpcError>> {
    match method {
        "initialize" => Some(mcp_protocol::initialize::mcp_initialize(params).await),
        "initialized" => Some(Ok(json!({}))),
        "tools/list" => Some(mcp_protocol::list_tools::mcp_list_tools().await),
        "tools/call" => Some(mcp_protocol::call_tool::mcp_call_tool(params, state).await),
        _ => None,
    }
}

/// Route P1 reliable handlers (for VS Code)
pub async fn route_p1_handlers(
    method: &str,
    params: &Option<Value>,
    state: Option<&LiveProjectState>,
) -> Option<Result<Value, RpcError>> {
    match method {
        "rustwork_get_file_doc" => {
            Some(rustwork_get_file_doc::p1_get_file_doc(params, state).await)
        }
        "rustwork_get_function_usage" => {
            Some(rustwork_get_function_usage::p1_get_function_usage(params, state).await)
        }
        "rustwork_get_route_impact" => {
            Some(rustwork_get_route_impact::p1_get_route_impact(params, state).await)
        }
        _ => None,
    }
}

/// Route VS Code rustwork tools
pub async fn route_rustwork_tools(
    method: &str,
    state: Option<&LiveProjectState>,
) -> Option<Result<Value, RpcError>> {
    match method {
        "rustwork_get_routes" => Some(rustwork_get_routes::rustwork_get_routes(state).await),
        "rustwork_get_diagnostics" => {
            Some(rustwork_get_diagnostics::rustwork_get_diagnostics(state).await)
        }
        "rustwork_get_conventions" => {
            Some(rustwork_get_conventions::rustwork_get_conventions(&None, state).await)
        }
        "rustwork_get_env_setup" => {
            Some(rustwork_get_env_setup::rustwork_get_env_setup(state).await)
        }
        _ => None,
    }
}

/// Route data and architecture tools (v0.6.0)
pub async fn route_data_architecture_tools(
    method: &str,
    params: &Option<Value>,
    state: Option<&LiveProjectState>,
) -> Option<Result<Value, RpcError>> {
    match method {
        "rustwork_get_database_schema" => {
            Some(rustwork_get_database_schema::rustwork_get_database_schema(params, state).await)
        }
        "rustwork_get_models" => {
            Some(rustwork_get_models::rustwork_get_models(params, state).await)
        }
        "rustwork_get_services_overview" => {
            Some(rustwork_get_services_overview::rustwork_get_services_overview(params, state).await)
        }
        _ => None,
    }
}

/// Route legacy endpoints (backward compatibility)
pub async fn route_legacy(
    method: &str,
    params: &Option<Value>,
    state: Option<&LiveProjectState>,
) -> Option<Result<Value, RpcError>> {
    match method {
        "get_routes" => Some(legacy::get_routes::get_routes_live(state).await),
        "get_file_doc" => Some(rustwork_get_file_doc::p1_get_file_doc(params, state).await),
        "get_function_usage" => {
            Some(rustwork_get_function_usage::p1_get_function_usage(params, state).await)
        }
        "get_route_impact" => {
            Some(rustwork_get_route_impact::p1_get_route_impact(params, state).await)
        }
        "get_call_graph" => Some(legacy::get_call_graph::get_call_graph(params, state).await),
        "get_diagnostics" => Some(legacy::get_diagnostics::get_diagnostics(state).await),
        "get_files" => Some(legacy::get_files::get_files(state).await),
        "get_functions" => Some(legacy::get_functions::get_functions(params, state).await),
        _ => None,
    }
}
