use std::path::{Path, PathBuf};

/// Workspace root - detected once at startup, immutable
#[derive(Debug, Clone)]
pub struct WorkspaceRoot {
    /// Absolute, canonicalized path to workspace root
    path: PathBuf,
    /// Layout type detected
    layout: WorkspaceLayout,
}

impl WorkspaceRoot {
    /// Create a new WorkspaceRoot instance
    pub(super) fn new(path: PathBuf, layout: WorkspaceLayout) -> Self {
        Self { path, layout }
    }

    /// Get the absolute path to the workspace root
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Get the workspace layout
    pub fn layout(&self) -> WorkspaceLayout {
        self.layout
    }

    /// Check if this is a microservices workspace
    pub fn is_microservices(&self) -> bool {
        matches!(self.layout, WorkspaceLayout::MicroServices)
    }
}

/// Workspace layout type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspaceLayout {
    /// Monolithic project with src/
    Monolith,
    /// Microservices with services/ directory
    MicroServices,
}
