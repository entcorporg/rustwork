use crate::mcp::protocol::RpcError;
use crate::mcp::state::LiveProjectState;
use serde_json::{json, Value};

/// Get all routes (live) - Legacy endpoint
pub async fn get_routes_live(state: Option<&LiveProjectState>) -> Result<Value, RpcError> {
    let state = state.ok_or_else(|| RpcError::internal_error("State not initialized"))?;

    let routes = state.routes.read().await;
    Ok(json!({
        "routes": routes.routes,
        "total": routes.routes.len(),
    }))
}
