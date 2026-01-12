use serde::{Deserialize, Serialize};

/// Confidence level for MCP responses
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Confidence {
    /// Proven by AST - high confidence
    High,
    /// Indirect relationship - partial confidence
    Partial,
    /// Cannot determine or ambiguous - MCP refuses to guess
    None,
}

impl Confidence {
    /// Check if response should be returned
    #[allow(dead_code)]
    pub fn is_sufficient(&self) -> bool {
        matches!(self, Confidence::High | Confidence::Partial)
    }
}

/// Service scope for micro-services architecture
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ServiceScope {
    /// Within a single service
    Local,
    /// Crosses service boundaries
    InterService,
}

/// Architecture layout
/// 
/// Rustwork is 100% microservices
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Layout {
    /// Microservices architecture
    Micro,
}
