use super::Parameter;
use serde::{Deserialize, Serialize};

/// Represents a function in the code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionInfo {
    pub name: String,
    pub is_public: bool,
    pub is_async: bool,
    /// Starting line number (1-indexed)
    pub start_line: usize,
    /// Ending line number (1-indexed)
    pub end_line: usize,
    pub signature: String,
    pub calls: Vec<String>, // Functions called by this function
    pub parameters: Vec<Parameter>,
    pub return_type: Option<String>,
}
