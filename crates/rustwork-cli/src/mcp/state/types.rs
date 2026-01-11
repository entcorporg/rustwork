use crate::mcp::diagnostics::DiagnosticCollection;
use crate::mcp::indexer::CodeIndex;
use crate::mcp::routes::RouteRegistry;
use crate::mcp::service_resolver::ServiceResolver;
use crate::mcp::workspace_root::WorkspaceRoot;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Live project state that updates dynamically
#[derive(Clone)]
pub struct LiveProjectState {
    /// Workspace root - detected once at startup, immutable
    pub workspace_root: Arc<WorkspaceRoot>,
    /// Legacy field for compatibility - use workspace_root.path() instead
    pub project_path: PathBuf,
    /// Service resolver for microservices layout
    pub service_resolver: Arc<ServiceResolver>,
    pub code_index: Arc<RwLock<CodeIndex>>,
    pub routes: Arc<RwLock<RouteRegistry>>,
    pub diagnostics: Arc<RwLock<DiagnosticCollection>>,
    pub is_scanning: Arc<RwLock<bool>>,
}
