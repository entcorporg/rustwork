use super::{FunctionInfo, StructInfo};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Represents a Rust source file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceFile {
    pub path: PathBuf,
    pub relative_path: String,
    pub module_path: String,
    pub functions: Vec<FunctionInfo>,
    pub structs: Vec<StructInfo>,
    pub last_modified: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>, // Service name if in microservices mode
}
