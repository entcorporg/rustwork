use crate::mcp::common::protocol::RpcError;
use crate::mcp::common::state::LiveProjectState;
use serde::Deserialize;
use serde_json::{json, Value};

/// Get functions (optionally filtered by file) - Legacy endpoint
pub async fn get_functions(
    params: &Option<Value>,
    state: Option<&LiveProjectState>,
) -> Result<Value, RpcError> {
    let state = state.ok_or_else(|| RpcError::internal_error("State not initialized"))?;

    #[derive(Deserialize)]
    struct Params {
        file: Option<String>,
    }

    let params: Params = serde_json::from_value(params.clone().unwrap_or(json!({})))
        .map_err(|e| RpcError::invalid_request(format!("Invalid params: {}", e)))?;

    let code_index = state.code_index.read().await;

    let functions: Vec<Value> = if let Some(file) = params.file {
        // Get functions from specific file
        code_index
            .files
            .get(&file)
            .map(|f| {
                f.functions
                    .iter()
                    .map(|func| {
                        json!({
                            "name": func.name,
                            "is_public": func.is_public,
                            "is_async": func.is_async,
                            "start_line": func.start_line,
                            "end_line": func.end_line,
                            "signature": func.signature,
                            "file": f.relative_path,
                        })
                    })
                    .collect()
            })
            .unwrap_or_default()
    } else {
        // Get all functions
        code_index
            .files
            .values()
            .flat_map(|f| {
                f.functions.iter().map(move |func| {
                    json!({
                        "name": func.name,
                        "is_public": func.is_public,
                        "is_async": func.is_async,
                        "start_line": func.start_line,
                        "end_line": func.end_line,
                        "signature": func.signature,
                        "file": f.relative_path,
                    })
                })
            })
            .collect()
    };

    Ok(json!({
        "functions": functions,
        "total": functions.len(),
    }))
}
