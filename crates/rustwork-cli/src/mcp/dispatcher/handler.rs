use super::routes::{route_legacy, route_mcp_protocol, route_p1_handlers, route_rustwork_tools};
use crate::mcp::protocol::{RpcError, RpcRequest, RpcResponse};
use crate::mcp::state::LiveProjectState;
use std::path::Path;

/// Handle an RPC request and return a response
pub async fn handle_request(
    request: RpcRequest,
    _project_path: &Path,
    state: Option<&LiveProjectState>,
) -> RpcResponse {
    let method = request.method.as_str();

    // Try routes in order: MCP protocol → P1 → Rustwork → Legacy
    let result = if let Some(res) = route_mcp_protocol(method, &request.params, state).await {
        res
    } else if let Some(res) = route_p1_handlers(method, &request.params, state).await {
        res
    } else if let Some(res) = route_rustwork_tools(method, state).await {
        res
    } else if let Some(res) = route_legacy(method, &request.params, state).await {
        res
    } else {
        Err(RpcError::method_not_found(&request.method))
    };

    match result {
        Ok(value) => RpcResponse::success(request.id, value),
        Err(error) => RpcResponse::error(request.id, error),
    }
}
