use super::types::{Layout, ServiceScope};
use serde::{Deserialize, Serialize};

/// Service context - REQUIRED for all MCP responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceContext {
    /// Service name (or "monolith" for single service)
    pub service_name: String,
    /// Architecture layout
    pub layout: Layout,
    /// Scope of the response
    pub scope: ServiceScope,
}

impl ServiceContext {
    pub fn monolith() -> Self {
        Self {
            service_name: "monolith".to_string(),
            layout: Layout::Monolith,
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
