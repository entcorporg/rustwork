/// Scanner pour les fichiers .rwk du workspace
use crate::mcp::common::workspace_root::WorkspaceRoot;
use std::path::PathBuf;
use std::sync::Arc;

/// Scanner de fichiers .rwk
pub struct RwkScanner {
    workspace_root: Arc<WorkspaceRoot>,
}

impl RwkScanner {
    pub fn new(workspace_root: Arc<WorkspaceRoot>) -> Self {
        Self { workspace_root }
    }

    /// Trouve tous les fichiers .rwk dans le workspace
    ///
    /// Règles de scan (microservices) :
    /// - Backend/services/*/grpc/*.rwk (nouvelle structure)
    /// - services/*/grpc/*.rwk (structure legacy)
    pub fn scan_rwk_files(&self) -> Result<Vec<PathBuf>, String> {
        let mut rwk_files = Vec::new();
        let root = self.workspace_root.path();

        // Scan Backend/services/*/grpc/*.rwk (new structure)
        let backend_services_dir = root.join("Backend/services");
        if backend_services_dir.exists() && backend_services_dir.is_dir() {
            self.scan_services_directory(&backend_services_dir, &mut rwk_files)?;
        }

        // Scan services/*/grpc/*.rwk (legacy structure)
        let services_dir = root.join("services");
        if services_dir.exists() && services_dir.is_dir() {
            self.scan_services_directory(&services_dir, &mut rwk_files)?;
        }

        Ok(rwk_files)
    }

    /// Scan un répertoire services pour les fichiers .rwk
    fn scan_services_directory(&self, services_dir: &PathBuf, files: &mut Vec<PathBuf>) -> Result<(), String> {
        for entry in std::fs::read_dir(services_dir)
            .map_err(|e| format!("Failed to read services/: {}", e))?
        {
            let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
            let service_path = entry.path();
            
            // Skip shared library
            if service_path.file_name().map(|n| n == "shared").unwrap_or(false) {
                continue;
            }
            
            let service_grpc_dir = service_path.join("grpc");

            if service_grpc_dir.exists() && service_grpc_dir.is_dir() {
                self.scan_directory(&service_grpc_dir, files)?;
            }
        }
        Ok(())
    }

    /// Scan un répertoire pour les fichiers .rwk
    fn scan_directory(&self, dir: &PathBuf, files: &mut Vec<PathBuf>) -> Result<(), String> {
        for entry in std::fs::read_dir(dir)
            .map_err(|e| format!("Failed to read directory {}: {}", dir.display(), e))?
        {
            let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
            let path = entry.path();

            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("rwk") {
                files.push(path);
            }
        }

        Ok(())
    }
}
