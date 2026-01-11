use crate::mcp::protocol::RpcError;
use crate::mcp::state::LiveProjectState;
use serde_json::{json, Value};

/// rustwork.getRoutes - Get all routes in the project
pub async fn rustwork_get_routes(state: Option<&LiveProjectState>) -> Result<Value, RpcError> {
    let state = state.ok_or_else(|| RpcError::internal_error("State not initialized"))?;

    let routes = state.routes.read().await;

    // Copilot-friendly: concise, structured data
    let route_list: Vec<Value> = routes
        .routes
        .iter()
        .map(|r| {
            json!({
                "method": format!("{:?}", r.method),
                "path": r.path,
                "handler": r.handler,
                "handler_function": r.handler_function,
                "file": r.file,
                "line": r.line
            })
        })
        .collect();

    Ok(json!({
        "routes": route_list,
        "count": route_list.len()
    }))
}
