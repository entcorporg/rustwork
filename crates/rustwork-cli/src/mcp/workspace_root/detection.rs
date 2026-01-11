use super::helpers::{detect_layout, has_parent_with_services, has_valid_services};
use super::types::{WorkspaceLayout, WorkspaceRoot};
use anyhow::{bail, Context, Result};
use std::path::Path;

impl WorkspaceRoot {
    /// Detect workspace root from a given starting path
    ///
    /// This MUST be called at MCP startup and fail fast if root cannot be determined.
    pub fn detect(start_path: &Path) -> Result<Self> {
        // Canonicalize the starting path
        let start_canonical = start_path.canonicalize().context(format!(
            "Cannot canonicalize path: {}",
            start_path.display()
        ))?;

        // Walk up from the starting path
        let mut current = start_canonical.as_path();

        loop {
            // Check for .rustwork/ marker (highest priority)
            let rustwork_marker = current.join(".rustwork");
            if rustwork_marker.exists() && rustwork_marker.is_dir() {
                return Ok(Self::new(current.to_path_buf(), detect_layout(current)?));
            }

            // Check for services/ directory (microservices)
            let services_dir = current.join("services");
            if services_dir.exists() && services_dir.is_dir() {
                // Validate it's a real microservices layout
                if has_valid_services(&services_dir) {
                    return Ok(Self::new(
                        current.to_path_buf(),
                        WorkspaceLayout::MicroServices,
                    ));
                }
            }

            // Check for src/ directory (monolith)
            let src_dir = current.join("src");
            if src_dir.exists() && src_dir.is_dir() {
                // Only accept as root if no parent has services/
                if !has_parent_with_services(current) {
                    return Ok(Self::new(current.to_path_buf(), WorkspaceLayout::Monolith));
                }
            }

            // Move to parent
            match current.parent() {
                Some(parent) => current = parent,
                None => break,
            }
        }

        bail!(
            "Cannot detect workspace root from path: {}\n\
            Expected one of:\n\
            - Directory with .rustwork/ marker\n\
            - Directory with services/ (microservices layout)\n\
            - Directory with src/ (monolith layout)",
            start_path.display()
        );
    }
}
