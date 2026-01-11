use serde::{Deserialize, Serialize};

/// Diagnostic severity level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Error,
    Warning,
    Info,
    Help,
}

/// A single diagnostic message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub severity: Severity,
    pub message: String,
    pub file: Option<String>,
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub code: Option<String>, // Error code like E0425
    pub source: String,       // "rustc", "clippy", "runtime"
    pub timestamp: u64,
}
