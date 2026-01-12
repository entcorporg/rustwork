use super::types::LiveProjectState;
use crate::mcp::common::diagnostics::DiagnosticCollection;
use crate::mcp::common::indexer::{CodeIndex, IndexState};
use crate::mcp::common::routes::RouteRegistry;
use crate::mcp::common::service_resolver::ServiceResolver;
use crate::mcp::common::workspace_root::WorkspaceRoot;
use anyhow::Result;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

impl LiveProjectState {
    /// Create a new live project state with workspace root detection
    ///
    /// CRITICAL: This method detects the workspace root at startup.
    /// If detection fails, the MCP server MUST NOT start.
    pub fn new(project_path: PathBuf) -> Result<Self> {
        // Detect workspace root (fail fast if not found)
        let workspace_root = WorkspaceRoot::detect(&project_path)?;

        eprintln!(
            "✅ Workspace root detected: {}",
            workspace_root.path().display()
        );
        eprintln!("� Services directory: {}", workspace_root.services_dir().display());

        let workspace_root_arc = Arc::new(workspace_root);
        let service_resolver = Arc::new(ServiceResolver::new(
            workspace_root_arc.path().to_path_buf(),
        ));

        Ok(Self {
            workspace_root: workspace_root_arc.clone(),
            project_path: workspace_root_arc.path().to_path_buf(),
            service_resolver,
            code_index: Arc::new(RwLock::new(CodeIndex::new())),
            index_state: Arc::new(RwLock::new(IndexState::NotStarted)),
            routes: Arc::new(RwLock::new(RouteRegistry::new())),
            diagnostics: Arc::new(RwLock::new(DiagnosticCollection::new())),
            is_scanning: Arc::new(RwLock::new(false)),
        })
    }
}
