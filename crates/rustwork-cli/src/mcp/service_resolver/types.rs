use std::path::PathBuf;

/// Service information for a file
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceInfo {
    /// Service name (e.g., "user", "auth")
    pub name: String,
    /// Service root directory
    pub root: PathBuf,
}

/// Service resolver
pub struct ServiceResolver {
    /// Workspace root path
    pub(super) workspace_root: PathBuf,
}

impl ServiceResolver {
    /// Create a new service resolver
    pub fn new(workspace_root: PathBuf) -> Self {
        Self { workspace_root }
    }
}
