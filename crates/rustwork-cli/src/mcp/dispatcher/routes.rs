use crate::mcp::protocol::RpcError;
use crate::mcp::state::LiveProjectState;
use crate::mcp::tools::rustwork::{p1_get_file_doc, p1_get_function_usage, p1_get_route_impact};
use crate::mcp::tools::{legacy, rustwork};
use serde_json::{json, Value};

/// Route MCP protocol standard methods
pub async fn route_mcp_protocol(
    method: &str,
    params: &Option<Value>,
    state: Option<&LiveProjectState>,
) -> Option<Result<Value, RpcError>> {
    match method {
        "initialize" => Some(crate::mcp::tools::mcp_protocol::mcp_initialize(params).await),
        "initialized" => Some(Ok(json!({}))),
        "tools/list" => Some(crate::mcp::tools::mcp_protocol::mcp_list_tools().await),
        "tools/call" => Some(crate::mcp::tools::mcp_protocol::mcp_call_tool(params, state).await),
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
        "rustwork_get_file_doc" => Some(p1_get_file_doc(params, state).await),
        "rustwork_get_function_usage" => Some(p1_get_function_usage(params, state).await),
        "rustwork_get_route_impact" => Some(p1_get_route_impact(params, state).await),
        _ => None,
    }
}

/// Route VS Code rustwork tools
pub async fn route_rustwork_tools(
    method: &str,
    state: Option<&LiveProjectState>,
) -> Option<Result<Value, RpcError>> {
    match method {
        "rustwork_get_routes" => Some(rustwork::get_routes::rustwork_get_routes(state).await),
        "rustwork_get_diagnostics" => {
            Some(rustwork::get_diagnostics::rustwork_get_diagnostics(state).await)
        }
        "rustwork_get_conventions" => {
            Some(rustwork::get_conventions::rustwork_get_conventions().await)
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
        "get_routes" => Some(legacy::get_routes_live(state).await),
        "get_file_doc" => Some(p1_get_file_doc(params, state).await),
        "get_function_usage" => Some(p1_get_function_usage(params, state).await),
        "get_route_impact" => Some(p1_get_route_impact(params, state).await),
        "get_call_graph" => Some(legacy::get_call_graph(params, state).await),
        "get_diagnostics" => Some(legacy::get_diagnostics(state).await),
        "get_files" => Some(legacy::get_files(state).await),
        "get_functions" => Some(legacy::get_functions(params, state).await),
        _ => None,
    }
}
