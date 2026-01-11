use crate::mcp::protocol::RpcError;
use crate::mcp::state::LiveProjectState;
use serde_json::{json, Value};

/// Get all diagnostics - Legacy endpoint
pub async fn get_diagnostics(state: Option<&LiveProjectState>) -> Result<Value, RpcError> {
    let state = state.ok_or_else(|| RpcError::internal_error("State not initialized"))?;

    let diagnostics = state.diagnostics.read().await;

    Ok(json!({
        "diagnostics": diagnostics.diagnostics,
        "errors": diagnostics.errors,
        "warnings": diagnostics.warnings,
        "last_build_success": diagnostics.last_build_success,
    }))
}
