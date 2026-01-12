use crate::mcp::common::diagnostics::DiagnosticCollection;
use crate::mcp::common::indexer::{CodeIndex, IndexState};
use crate::mcp::common::routes::RouteRegistry;
use crate::mcp::common::service_resolver::ServiceResolver;
use crate::mcp::common::workspace_root::WorkspaceRoot;
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
    /// État explicite de l'index (P0 fix)
    pub index_state: Arc<RwLock<IndexState>>,
    pub routes: Arc<RwLock<RouteRegistry>>,
    pub diagnostics: Arc<RwLock<DiagnosticCollection>>,
    pub is_scanning: Arc<RwLock<bool>>,
}
