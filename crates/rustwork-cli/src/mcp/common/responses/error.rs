/// Error response structure for MCP tools
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpError {
    /// Error code
    pub code: String,

    /// Human-readable message
    pub message: String,

    /// Root cause
    pub cause: Option<String>,

    /// Suggestion for fix
    pub suggestion: Option<String>,
}

impl McpError {
    pub fn file_not_found(path: &str) -> Self {
        Self {
            code: "FILE_NOT_FOUND".to_string(),
            message: format!("File not found: {}", path),
            cause: Some("File does not exist in workspace".to_string()),
            suggestion: Some("Verify the file path is relative to project root".to_string()),
        }
    }

    pub fn outside_workspace(path: &str) -> Self {
        Self {
            code: "OUTSIDE_WORKSPACE".to_string(),
            message: format!("Path is outside workspace: {}", path),
            cause: Some("MCP only operates within workspace boundaries".to_string()),
            suggestion: Some("Use a path within the project root".to_string()),
        }
    }

    pub fn not_rust_file(path: &str) -> Self {
        Self {
            code: "NOT_RUST_FILE".to_string(),
            message: format!("File is not a Rust source file: {}", path),
            cause: Some("MCP only indexes .rs files".to_string()),
            suggestion: None,
        }
    }

    pub fn function_not_found(function: &str) -> Self {
        Self {
            code: "FUNCTION_NOT_FOUND".to_string(),
            message: format!("Function not found: {}", function),
            cause: Some("Function does not exist in indexed codebase".to_string()),
            suggestion: Some("Request the tool `rustwork_get_diagnostics` to check the current cargo-watch and build status before retrying".to_string()),
        }
    }

    pub fn route_not_found(method: &str, path: &str) -> Self {
        Self {
            code: "ROUTE_NOT_FOUND".to_string(),
            message: format!("Route not found: {} {}", method, path),
            cause: Some("Route does not exist in indexed routes".to_string()),
            suggestion: Some("Request the tool `rustwork_get_diagnostics` to verify cargo-watch state and route scanning before retrying".to_string()),
        }
    }

    #[allow(dead_code)]
    pub fn ambiguous_location(item: &str) -> Self {
        Self {
            code: "AMBIGUOUS_LOCATION".to_string(),
            message: format!("Cannot determine exact location for: {}", item),
            cause: Some("Source location is ambiguous or unknown".to_string()),
            suggestion: Some("MCP refuses to guess. Verify item exists in source.".to_string()),
        }
    }
}
