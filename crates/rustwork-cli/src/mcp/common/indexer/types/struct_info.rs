use super::FieldInfo;
use serde::{Deserialize, Serialize};

/// Represents a struct in the code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructInfo {
    pub name: String,
    pub is_public: bool,
    /// Starting line number (1-indexed)
    pub start_line: usize,
    /// Ending line number (1-indexed)
    pub end_line: usize,
    pub fields: Vec<FieldInfo>,
}
