use super::types::{Layout, ServiceScope};
use serde::{Deserialize, Serialize};

/// Service context - REQUIRED for all MCP responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceContext {
    /// Service name
    pub service_name: String,
    /// Architecture layout (always micro)
    pub layout: Layout,
    /// Scope of the response
    pub scope: ServiceScope,
}

impl ServiceContext {
    /// Create a default service context for unknown service
    pub fn default_service() -> Self {
        Self {
            service_name: "unknown".to_string(),
            layout: Layout::Micro,
            scope: ServiceScope::Local,
        }
    }

    pub fn service(name: String, scope: ServiceScope) -> Self {
        Self {
            service_name: name,
            layout: Layout::Micro,
            scope,
        }
    }
}
