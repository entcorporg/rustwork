use super::types::WorkspaceLayout;
use anyhow::Result;
use std::path::Path;

/// Detect layout type for a confirmed workspace root
pub(super) fn detect_layout(root: &Path) -> Result<WorkspaceLayout> {
    let services_dir = root.join("services");

    if services_dir.exists() && services_dir.is_dir() && has_valid_services(&services_dir) {
        Ok(WorkspaceLayout::MicroServices)
    } else {
        Ok(WorkspaceLayout::Monolith)
    }
}

/// Validate that services/ directory contains actual services
pub(super) fn has_valid_services(services_dir: &Path) -> bool {
    if let Ok(entries) = std::fs::read_dir(services_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                // A valid service has src/ directory
                let src_dir = path.join("src");
                if src_dir.exists() && src_dir.is_dir() {
                    return true;
                }
            }
        }
    }
    false
}

/// Check if any parent directory has services/ folder
pub(super) fn has_parent_with_services(start: &Path) -> bool {
    let mut current = start;
    while let Some(parent) = current.parent() {
        let services_dir = parent.join("services");
        if services_dir.exists() && services_dir.is_dir() {
            return true;
        }
        current = parent;
    }
    false
}
