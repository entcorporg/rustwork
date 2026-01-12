use crate::mcp::common::confidence::{Confidence, McpResponse};
use crate::mcp::common::indexer::{IndexState, SourceFile};
use crate::mcp::common::path_normalization::NormalizedPath;
use crate::mcp::common::protocol::RpcError;
use crate::mcp::common::responses::*;
use crate::mcp::common::state::LiveProjectState;
/// P1 HANDLER: get_file_doc (FIABLE)
///
/// Get file documentation with EXACT locations
/// CRITICAL: Zero heuristics, explicit confidence, deterministic responses
use crate::mcp::common::utils::p1_helpers::{
    determine_service_context, determine_service_context_from_file,
};
use serde::Deserialize;
use serde_json::{json, Value};

/// Get file documentation with EXACT locations
///
/// Rules (P1.1 - Fiabilité finale + P0 index state):
/// - CRITICAL P0: Vérifie l'état de l'index AVANT toute opération
/// - Refuse si index != READY avec erreur explicite
/// - Input: file path (user-provided)
/// - Normalize path BEFORE lookup using workspace root
/// - If file not in workspace → error
/// - If file not Rust → error
/// - In microservices: file MUST be in a service → error otherwise
/// - If file empty → empty response (not error)
/// - Return: service, functions with exact spans, structs with exact spans
/// - ZERO heuristics, ZERO guessing
pub async fn p1_get_file_doc(
    params: &Option<Value>,
    state: Option<&LiveProjectState>,
) -> Result<Value, RpcError> {
    let state = state.ok_or_else(|| RpcError::internal_error("State not initialized"))?;

    #[derive(Deserialize)]
    struct Params {
        path: String,
    }

    let params: Params = serde_json::from_value(params.clone().unwrap_or(json!({})))
        .map_err(|e| RpcError::invalid_request(format!("Invalid params: {}", e)))?;

    // CRITIQUE P0 : Vérifier l'état de l'index AVANT toute opération
    let index_state = state.index_state.read().await;
    if !index_state.is_ready() {
        let error = McpError {
            code: index_state.error_code().to_string(),
            message: index_state.error_message(),
            cause: Some("Index must be in READY state for get_file_doc to operate".to_string()),
            suggestion: Some(
                "Use `rustwork_get_diagnostics` to check index status and wait for it to be ready"
                    .to_string(),
            ),
        };
        let context = determine_service_context(state).await;
        let confidence = match *index_state {
            IndexState::Scanning => Confidence::Partial,
            _ => Confidence::None,
        };
        let response: McpResponse<McpError> = McpResponse {
            data: error,
            confidence,
            context,
        };
        return Ok(serde_json::to_value(response).unwrap());
    }
    drop(index_state); // Libérer le lock

    // Step 1: Normalize path (CRITICAL - use workspace root)
    let workspace_root = state.workspace_root.path();
    let normalized_path = match NormalizedPath::from_str(&params.path, workspace_root) {
        Ok(p) => p,
        Err(e) => {
            // Refuse with clear error
            let error = if e.to_string().contains("does not exist") {
                McpError::file_not_found(&params.path)
            } else if e.to_string().contains("outside workspace") {
                McpError::outside_workspace(&params.path)
            } else if e.to_string().contains("directory") {
                McpError {
                    code: "NOT_A_FILE".to_string(),
                    message: format!("Path is a directory, not a file: {}", params.path),
                    cause: Some("get_file_doc requires a file path".to_string()),
                    suggestion: Some("Specify a .rs file, not a directory".to_string()),
                }
            } else {
                McpError {
                    code: "PATH_NORMALIZATION_FAILED".to_string(),
                    message: format!("Cannot normalize path: {}", params.path),
                    cause: Some(e.to_string()),
                    suggestion: Some("Use a valid path relative to workspace root".to_string()),
                }
            };

            let context = determine_service_context(state).await;
            let response: McpResponse<McpError> = McpResponse {
                data: error,
                confidence: Confidence::None,
                context,
            };
            return Ok(serde_json::to_value(response).unwrap());
        }
    };

    // Step 2: Verify it's a Rust file
    if !normalized_path.as_str().ends_with(".rs") {
        let error = McpError::not_rust_file(&params.path);
        let context = determine_service_context(state).await;
        let response: McpResponse<McpError> = McpResponse {
            data: error,
            confidence: Confidence::None,
            context,
        };
        return Ok(serde_json::to_value(response).unwrap());
    }

    // Step 3: Verify file is in a valid service (Rustwork is 100% microservices)
    let absolute_path = normalized_path.to_absolute(workspace_root);
    match state.service_resolver.resolve_service(&absolute_path) {
        Ok(_service_info) => {
            // File is in a valid service - continue
        }
        Err(e) => {
            // File is NOT in any service - REFUSE
            let error = McpError {
                code: "FILE_OUTSIDE_SERVICE".to_string(),
                message: format!(
                    "File is outside any registered micro-service: {}",
                    params.path
                ),
                cause: Some(e.to_string()),
                suggestion: Some(format!(
                    "get_file_doc refuses to guess. File must be in workspace_root/services/<service_name>/... \
                    Available services: {:?}",
                    state.service_resolver.list_services().unwrap_or_default()
                )),
            };
            let context = determine_service_context(state).await;
            let response: McpResponse<McpError> = McpResponse {
                data: error,
                confidence: Confidence::None,
                context,
            };
            return Ok(serde_json::to_value(response).unwrap());
        }
    }

    // Step 4: Lookup in index
    let code_index = state.code_index.read().await;

    // DEBUG: Log lookup attempt
    eprintln!(
        "🔍 DEBUG get_file_doc: Looking for key: '{}'",
        normalized_path.as_str()
    );
    eprintln!(
        "🔍 DEBUG get_file_doc: Index contains {} files",
        code_index.files.len()
    );
    if code_index.files.is_empty() {
        eprintln!("⚠️  DEBUG get_file_doc: Index is empty!");
    } else {
        eprintln!("🔍 DEBUG get_file_doc: First 5 keys in index:");
        for (idx, key) in code_index.files.keys().take(5).enumerate() {
            eprintln!("    [{}] '{}'", idx, key);
        }
    }

    let file = match code_index.files.get(normalized_path.as_str()) {
        Some(f) => f,
        None => {
            // File exists but not indexed - refuse with clear message
            let error = McpError {
                code: "FILE_NOT_INDEXED".to_string(),
                message: format!("File not found in index: {}", params.path),
                cause: Some("File may exist but has not been indexed yet".to_string()),
                suggestion: Some(
                    "Request the tool `rustwork_get_diagnostics` to monitor indexing and build progress before retrying".to_string(),
                ),
            };
            let context = determine_service_context(state).await;
            let response: McpResponse<McpError> = McpResponse {
                data: error,
                confidence: Confidence::None,
                context,
            };
            return Ok(serde_json::to_value(response).unwrap());
        }
    };

    // Step 5: Build reliable response with EXACT spans
    let response_data = build_file_doc_response(file);
    let context = determine_service_context_from_file(file, state);

    // High confidence - data comes directly from AST
    let response = McpResponse::high_confidence(response_data, context);

    Ok(serde_json::to_value(response).unwrap())
}

fn build_file_doc_response(file: &SourceFile) -> FileDocResponse {
    FileDocResponse {
        path: file.relative_path.clone(),
        module_path: file.module_path.clone(),
        functions: file
            .functions
            .iter()
            .map(|f| FunctionDocInfo {
                name: f.name.clone(),
                signature: f.signature.clone(),
                start_line: f.start_line,
                end_line: f.end_line,
                is_public: f.is_public,
                is_async: f.is_async,
                parameters: f
                    .parameters
                    .iter()
                    .map(|p| ParameterInfo {
                        name: p.name.clone(),
                        type_name: p.type_name.clone(),
                    })
                    .collect(),
                return_type: f.return_type.clone(),
            })
            .collect(),
        structs: file
            .structs
            .iter()
            .map(|s| StructDocInfo {
                name: s.name.clone(),
                start_line: s.start_line,
                end_line: s.end_line,
                is_public: s.is_public,
                fields: s
                    .fields
                    .iter()
                    .map(|f| FieldDocInfo {
                        name: f.name.clone(),
                        type_name: f.type_name.clone(),
                        is_public: f.is_public,
                    })
                    .collect(),
            })
            .collect(),
    }
}
