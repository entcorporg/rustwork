use crate::mcp::protocol::RpcError;
use crate::mcp::state::LiveProjectState;
use serde_json::{json, Value};

/// rustwork.getDiagnostics - Get compilation and lint diagnostics
pub async fn rustwork_get_diagnostics(state: Option<&LiveProjectState>) -> Result<Value, RpcError> {
    let state = state.ok_or_else(|| RpcError::internal_error("State not initialized"))?;

    let diagnostics = state.diagnostics.read().await;

    Ok(json!({
        "errors": diagnostics.errors,
        "warnings": diagnostics.warnings,
        "total": diagnostics.diagnostics.len(),
        "last_build_success": diagnostics.last_build_success,
        "diagnostics": diagnostics.diagnostics.iter().map(|d| json!({
            "severity": d.severity,
            "message": d.message,
            "file": d.file,
            "line": d.line,
            "column": d.column
        })).collect::<Vec<_>>()
    }))
}
