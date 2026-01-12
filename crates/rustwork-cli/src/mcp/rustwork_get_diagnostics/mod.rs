use crate::mcp::common::protocol::RpcError;
use crate::mcp::common::state::LiveProjectState;
use serde_json::{json, Value};

/// rustwork.getDiagnostics - Get compilation and lint diagnostics
///
/// CRITICAL P0: Expose l'état de l'index MCP pour débogage
pub async fn rustwork_get_diagnostics(state: Option<&LiveProjectState>) -> Result<Value, RpcError> {
    let state = state.ok_or_else(|| RpcError::internal_error("State not initialized"))?;

    let diagnostics = state.diagnostics.read().await;
    let index_state = state.index_state.read().await;
    let code_index = state.code_index.read().await;

    // Check for environment-related errors
    let mut suggestions = Vec::new();
    let has_env_related_errors = diagnostics.diagnostics.iter().any(|d| {
        let msg_lower = d.message.to_lowercase();
        msg_lower.contains("port")
            || msg_lower.contains("address already in use")
            || msg_lower.contains("env")
            || msg_lower.contains("environment")
    });

    if has_env_related_errors {
        suggestions.push("Request rustwork_get_env_setup to verify environment variables and detect port conflicts before retrying");
    }

    // P0: Ajouter suggestions si index pas READY
    if !index_state.is_ready() {
        suggestions.push("Index is not ready - tools like get_file_doc cannot operate. Wait for index to reach READY state.");
    }

    let mut result = json!({
        "errors": diagnostics.errors,
        "warnings": diagnostics.warnings,
        "total": diagnostics.diagnostics.len(),
        "last_build_success": diagnostics.last_build_success,
        // P0: Exposer l'état de l'index
        "index_state": index_state.to_string(),
        "index_files_count": code_index.files.len(),
        "index_is_ready": index_state.is_ready(),
        "diagnostics": diagnostics.diagnostics.iter().map(|d| json!({
            "severity": d.severity,
            "message": d.message,
            "file": d.file,
            "line": d.line,
            "column": d.column
        })).collect::<Vec<_>>()
    });

    if !suggestions.is_empty() {
        result["suggestions"] = json!(suggestions);
    }

    Ok(result)
}
