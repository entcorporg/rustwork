/// Response structures for get_file_doc tool
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDocResponse {
    /// Normalized path (relative to project root)
    pub path: String,

    /// Module path
    pub module_path: String,

    /// Functions in this file with EXACT locations
    pub functions: Vec<FunctionDocInfo>,

    /// Structs in this file with EXACT locations
    pub structs: Vec<StructDocInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDocInfo {
    pub name: String,
    pub signature: String,
    /// Starting line (1-indexed)
    pub start_line: usize,
    /// Ending line (1-indexed)
    pub end_line: usize,
    pub is_public: bool,
    pub is_async: bool,
    pub parameters: Vec<ParameterInfo>,
    pub return_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterInfo {
    pub name: String,
    pub type_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructDocInfo {
    pub name: String,
    /// Starting line (1-indexed)
    pub start_line: usize,
    /// Ending line (1-indexed)
    pub end_line: usize,
    pub is_public: bool,
    pub fields: Vec<FieldDocInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDocInfo {
    pub name: String,
    pub type_name: String,
    pub is_public: bool,
}
