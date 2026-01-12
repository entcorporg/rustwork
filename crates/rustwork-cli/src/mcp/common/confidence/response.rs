use super::context::ServiceContext;
use super::types::Confidence;
use serde::{Deserialize, Serialize};

/// MCP response wrapper with confidence and context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpResponse<T> {
    /// The actual response data
    #[serde(flatten)]
    pub data: T,

    /// Confidence level - MANDATORY
    pub confidence: Confidence,

    /// Service context - MANDATORY
    pub context: ServiceContext,
}

/// Refusal response when MCP cannot provide reliable answer
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct RefusalData {
    pub reason: String,
}

impl<T> McpResponse<T> {
    /// Create a high-confidence response
    pub fn high_confidence(data: T, context: ServiceContext) -> Self {
        Self {
            data,
            confidence: Confidence::High,
            context,
        }
    }

    /// Create a partial-confidence response with explanation
    #[allow(dead_code)]
    pub fn partial_confidence(data: T, context: ServiceContext) -> Self {
        Self {
            data,
            confidence: Confidence::Partial,
            context,
        }
    }

    /// Create a refusal response when confidence is none
    #[allow(dead_code)]
    pub fn refuse(reason: String, context: ServiceContext) -> McpResponse<RefusalData> {
        McpResponse {
            data: RefusalData { reason },
            confidence: Confidence::None,
            context,
        }
    }
}
