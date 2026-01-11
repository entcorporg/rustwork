use crate::mcp::protocol::RpcError;
use crate::mcp::state::LiveProjectState;
use serde_json::{json, Value};

/// Get all indexed files - Legacy endpoint
pub async fn get_files(state: Option<&LiveProjectState>) -> Result<Value, RpcError> {
    let state = state.ok_or_else(|| RpcError::internal_error("State not initialized"))?;

    let code_index = state.code_index.read().await;

    let files: Vec<Value> = code_index
        .files
        .values()
        .map(|f| {
            json!({
                "path": f.relative_path,
                "module_path": f.module_path,
                "functions_count": f.functions.len(),
                "structs_count": f.structs.len(),
            })
        })
        .collect();

    Ok(json!({
        "files": files,
        "total": files.len(),
    }))
}
