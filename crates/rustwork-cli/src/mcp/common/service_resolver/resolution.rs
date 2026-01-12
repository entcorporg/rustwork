use super::helpers::find_all_rustwork_services;
use super::types::{ServiceInfo, ServiceResolver};
use anyhow::{bail, Result};
use std::path::Path;

impl ServiceResolver {
    /// Resolve which service a file belongs to
    ///
    /// CRITICAL RULES:
    /// - File must be under a valid Rustwork service directory
    /// - A valid Rustwork service has: .rustwork/manifest.json, Cargo.toml, src/main.rs
    /// - Searches in: workspace_root/services/, workspace_root/backend/services/, and direct children
    /// - Returns error if file is not in any service
    /// - No fuzzy matching, no "best guess"
    pub fn resolve_service(&self, file_path: &Path) -> Result<ServiceInfo> {
        // Canonicalize both paths for accurate comparison
        let file_canonical = file_path
            .canonicalize()
            .map_err(|e| anyhow::anyhow!("File does not exist: {} - {}", file_path.display(), e))?;

        let workspace_canonical = self.workspace_root.canonicalize().map_err(|e| {
            anyhow::anyhow!(
                "Workspace root does not exist: {} - {}",
                self.workspace_root.display(),
                e
            )
        })?;

        // Ensure file is within workspace
        if !file_canonical.starts_with(&workspace_canonical) {
            bail!(
                "File '{}' is outside workspace root '{}'",
                file_path.display(),
                self.workspace_root.display()
            );
        }

        // Find all valid Rustwork services in workspace
        let services = find_all_rustwork_services(&workspace_canonical)?;

        // Find which service this file belongs to
        for service_root in services {
            if file_canonical.starts_with(&service_root) {
                let service_name = service_root
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string();

                return Ok(ServiceInfo {
                    name: service_name,
                    root: service_root,
                });
            }
        }

        // File is not in any service
        let relative = file_canonical
            .strip_prefix(&workspace_canonical)
            .unwrap_or(&file_canonical);

        bail!(
            "File is outside any registered Rustwork service. \
            File: {}. \
            \
            A valid Rustwork service requires:\n\
            - .rustwork/manifest.json\n\
            - Cargo.toml\n\
            - src/main.rs",
            relative.display()
        );
    }

    /// List all services in the workspace
    pub fn list_services(&self) -> Result<Vec<String>> {
        let workspace_canonical = self.workspace_root.canonicalize()?;
        let services = find_all_rustwork_services(&workspace_canonical)?;

        Ok(services
            .into_iter()
            .filter_map(|service_root| {
                service_root
                    .file_name()
                    .and_then(|n| n.to_str())
                    .map(|s| s.to_string())
            })
            .collect())
    }
}
