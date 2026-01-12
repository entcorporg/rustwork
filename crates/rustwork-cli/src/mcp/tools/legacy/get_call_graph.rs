use crate::mcp::common::protocol::RpcError;
use crate::mcp::common::state::LiveProjectState;
use serde::Deserialize;
use serde_json::{json, Value};

/// Get call graph for a function - Legacy endpoint
pub async fn get_call_graph(
    params: &Option<Value>,
    state: Option<&LiveProjectState>,
) -> Result<Value, RpcError> {
    let state = state.ok_or_else(|| RpcError::internal_error("State not initialized"))?;

    #[derive(Deserialize)]
    struct Params {
        function: String,
        #[serde(default = "default_depth")]
        depth: usize,
    }

    fn default_depth() -> usize {
        5
    }

    let params: Params = serde_json::from_value(params.clone().unwrap_or(json!({})))
        .map_err(|e| RpcError::invalid_request(format!("Invalid params: {}", e)))?;

    let code_index = state.code_index.read().await;
    let calls = code_index.get_calls(&params.function, params.depth);

    Ok(json!({
        "function": params.function,
        "depth": params.depth,
        "calls": calls,
    }))
}
