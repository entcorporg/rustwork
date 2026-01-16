use std::path::{Path, PathBuf};

/// Workspace root - detected once at startup, immutable
///
/// Rustwork is 100% microservices - no monolith support.
#[derive(Debug, Clone)]
pub struct WorkspaceRoot {
    /// Absolute, canonicalized path to workspace root
    path: PathBuf,
}

impl WorkspaceRoot {
    /// Create a new WorkspaceRoot instance
    pub(super) fn new(path: PathBuf) -> Self {
        Self { path }
    }

    /// Get the absolute path to the workspace root
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Get the services directory path
    ///
    /// Returns Backend/services/ if it exists, otherwise services/
    pub fn services_dir(&self) -> PathBuf {
        let backend_services = self.path.join("Backend/services");
        if backend_services.exists() {
            backend_services
        } else {
            self.path.join("services")
        }
    }

    /// Get the Cargo workspace directory path
    ///
    /// Returns Backend/ if it exists (microservices with Backend/Cargo.toml),
    /// otherwise returns the root path (legacy structure)
    pub fn cargo_workspace_dir(&self) -> PathBuf {
        let backend_dir = self.path.join("Backend");
        if backend_dir.join("Cargo.toml").exists() {
            backend_dir
        } else {
            self.path.clone()
        }
    }

    /// Check if this workspace has a shared library
    pub fn has_shared(&self) -> bool {
        self.services_dir().join("shared").exists()
    }
}
