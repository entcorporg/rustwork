use super::types::{ServiceInfo, ServiceResolver};
use anyhow::{bail, Result};
use std::path::Path;

impl ServiceResolver {
    /// Resolve which service a file belongs to
    ///
    /// Rules:
    /// - File must be under workspace_root/services/<service_name>/
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
        let relative = file_canonical
            .strip_prefix(&workspace_canonical)
            .map_err(|_| {
                anyhow::anyhow!(
                    "File '{}' is outside workspace root '{}'",
                    file_path.display(),
                    self.workspace_root.display()
                )
            })?;

        // Check if file is in services/ directory
        let mut components = relative.components();

        match components.next() {
            Some(std::path::Component::Normal(first)) if first == "services" => {
                // Next component should be service name
                match components.next() {
                    Some(std::path::Component::Normal(service_name)) => {
                        let service_name_str = service_name.to_string_lossy().to_string();
                        let service_root =
                            workspace_canonical.join("services").join(&service_name_str);

                        // Validate service root exists
                        if !service_root.exists() || !service_root.is_dir() {
                            bail!(
                                "Service directory does not exist: {}",
                                service_root.display()
                            );
                        }

                        Ok(ServiceInfo {
                            name: service_name_str,
                            root: service_root,
                        })
                    }
                    _ => bail!(
                        "File is outside any registered micro-service. \
                        Path: {}. \
                        Expected: workspace_root/services/<service_name>/...",
                        relative.display()
                    ),
                }
            }
            _ => bail!(
                "File is outside any registered micro-service. \
                Path: {}. \
                Expected: workspace_root/services/<service_name>/...",
                relative.display()
            ),
        }
    }
}
