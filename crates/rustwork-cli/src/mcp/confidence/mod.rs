/// Confidence and Context module
///
/// CRITICAL: Every MCP response MUST include confidence level.
/// The MCP prefers "none" to a false response.
mod context;
mod response;
mod types;

pub use context::ServiceContext;
pub use response::McpResponse;
pub use types::{Confidence, ServiceScope};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mcp::confidence::types::Layout;

    #[test]
    fn test_confidence_levels() {
        assert!(Confidence::High.is_sufficient());
        assert!(Confidence::Partial.is_sufficient());
        assert!(!Confidence::None.is_sufficient());
    }

    #[test]
    fn test_service_context() {
        let mono = ServiceContext::monolith();
        assert_eq!(mono.layout, Layout::Monolith);
        assert_eq!(mono.service_name, "monolith");

        let micro = ServiceContext::service("users".to_string(), ServiceScope::Local);
        assert_eq!(micro.layout, Layout::Micro);
        assert_eq!(micro.service_name, "users");
    }
}
