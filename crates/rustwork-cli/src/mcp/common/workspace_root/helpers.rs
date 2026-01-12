use anyhow::Result;
use std::path::Path;

/// Check if a directory is a valid Rust project
///
/// A valid Rust project requires:
/// - Cargo.toml
/// - src/ directory with main.rs or lib.rs
///
/// Note: .rustwork/manifest.json is OPTIONAL (for Rustwork-specific features)
pub(super) fn is_valid_rust_project(dir: &Path) -> bool {
    let has_cargo = dir.join("Cargo.toml").exists();
    let has_main = dir.join("src/main.rs").exists();
    let has_lib = dir.join("src/lib.rs").exists();

    has_cargo && (has_main || has_lib)
}

/// Check if a directory is a Rustwork-enhanced project (has .rustwork marker)
#[allow(dead_code)]
pub(super) fn is_rustwork_project(dir: &Path) -> bool {
    is_valid_rust_project(dir) && dir.join(".rustwork").exists()
}

/// Count the number of valid Rust projects in services directories
///
/// Scans: Backend/services/, services/, and direct children
pub(super) fn count_rust_projects_in_workspace(dir: &Path) -> usize {
    let mut count = 0;

    // Check Backend/services/ (new structure)
    let backend_services = dir.join("Backend/services");
    if backend_services.exists() && backend_services.is_dir() {
        count += count_projects_in_dir(&backend_services);
    }

    // Check services/ (legacy structure)
    let services_dir = dir.join("services");
    if services_dir.exists() && services_dir.is_dir() {
        count += count_projects_in_dir(&services_dir);
    }

    // Also check direct children for backward compatibility
    if count == 0 {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() && is_valid_rust_project(&path) {
                    count += 1;
                }
            }
        }
    }

    count
}

/// Count valid Rust projects in a directory (excluding 'shared')
fn count_projects_in_dir(dir: &Path) -> usize {
    let mut count = 0;
    
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                // Skip shared library - it's not a runnable service
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name == "shared" {
                        continue;
                    }
                }
                if is_valid_rust_project(&path) {
                    count += 1;
                }
            }
        }
    }
    
    count
}

/// Validate that a directory is a valid Rustwork workspace
/// 
/// A valid workspace has:
/// - Backend/services/ with at least one service, OR
/// - services/ with at least one service
pub(super) fn is_valid_rustwork_workspace(dir: &Path) -> Result<bool> {
    // Check for Backend/services structure (new)
    let backend_services = dir.join("Backend/services");
    if backend_services.exists() && backend_services.is_dir() {
        let count = count_projects_in_dir(&backend_services);
        if count > 0 {
            return Ok(true);
        }
    }

    // Check for services/ structure (legacy)
    let services_dir = dir.join("services");
    if services_dir.exists() && services_dir.is_dir() {
        let count = count_projects_in_dir(&services_dir);
        if count > 0 {
            return Ok(true);
        }
    }

    Ok(false)
}
