use anyhow::Result;
use std::path::{Path, PathBuf};

/// Find all valid Rustwork services in the workspace
///
/// A valid Rustwork service requires:
/// - .rustwork/manifest.json
/// - Cargo.toml
/// - src/main.rs
///
/// Searches in:
/// 1. workspace_root/services/
/// 2. workspace_root/backend/services/
/// 3. Direct children of workspace_root
pub fn find_all_rustwork_services(workspace_root: &Path) -> Result<Vec<PathBuf>> {
    let mut services = Vec::new();

    // Pattern 1: workspace_root/services/*
    let services_dir = workspace_root.join("services");
    if services_dir.exists() && services_dir.is_dir() {
        scan_for_valid_services(&services_dir, &mut services)?;
    }

    // Pattern 2: workspace_root/backend/services/*
    let backend_services = workspace_root.join("backend").join("services");
    if backend_services.exists() && backend_services.is_dir() {
        scan_for_valid_services(&backend_services, &mut services)?;
    }

    // Pattern 3: Direct children of workspace_root (single-service case)
    if let Ok(entries) = std::fs::read_dir(workspace_root) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() && is_valid_rustwork_service(&path) {
                services.push(path);
            }
        }
    }

    Ok(services)
}

/// Scan a directory for valid Rustwork services
fn scan_for_valid_services(dir: &Path, services: &mut Vec<PathBuf>) -> Result<()> {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() && is_valid_rustwork_service(&path) {
                services.push(path);
            }
        }
    }
    Ok(())
}

/// Check if a directory is a valid Rustwork service
fn is_valid_rustwork_service(dir: &Path) -> bool {
    dir.join(".rustwork/manifest.json").exists()
        && dir.join("Cargo.toml").exists()
        && dir.join("src/main.rs").exists()
}
