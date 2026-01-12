use super::helpers::{count_rust_projects_in_workspace, is_valid_rustwork_workspace};
use super::types::WorkspaceRoot;
use anyhow::{bail, Context, Result};
use std::path::Path;

impl WorkspaceRoot {
    /// Detect workspace root from a given starting path
    ///
    /// CRITICAL: This MUST be called at MCP startup and fail fast if root cannot be determined.
    ///
    /// Detection rules (STRICT ORDER):
    /// 1. If explicit_path provided -> use it (no search)
    /// 2. From start_path, walk up recursively:
    ///    a. Look for Cargo.toml with [workspace]
    ///    b. Look for Backend/services/ or services/ directory
    /// 3. If none found -> FAIL FAST with clear error
    ///
    /// Rustwork is 100% microservices - no monolith detection.
    pub fn detect(start_path: &Path) -> Result<Self> {
        Self::detect_with_explicit(start_path, None)
    }

    /// Detect workspace root with optional explicit path
    ///
    /// If explicit_path is Some, use it directly without search.
    /// Otherwise, follow the detection rules.
    pub fn detect_with_explicit(start_path: &Path, explicit_path: Option<&Path>) -> Result<Self> {
        // Rule 1: If explicit path provided, use it
        if let Some(explicit) = explicit_path {
            let canonical = explicit.canonicalize().context(format!(
                "Cannot canonicalize explicit path: {}",
                explicit.display()
            ))?;

            // Validate it's a valid workspace root
            if !is_valid_rustwork_workspace(&canonical)? {
                bail!(
                    "Not a valid Rustwork workspace: {}\n\
                    \n\
                    Expected structure:\n\
                    ./\n\
                    └── Backend/\n\
                        └── services/\n\
                            ├── <service1>/\n\
                            └── <service2>/\n\
                    \n\
                    Create a new workspace with: rustwork new auth,user,session",
                    canonical.display()
                );
            }

            return Ok(Self::new(canonical.to_path_buf()));
        }

        // Rule 2: Walk up from start_path
        let start_canonical = start_path.canonicalize().context(format!(
            "Cannot canonicalize start path: {}",
            start_path.display()
        ))?;

        let mut current = start_canonical.as_path();

        loop {
            // Check if this directory is a valid workspace root
            if Self::is_valid_workspace_root(current) {
                return Ok(Self::new(current.to_path_buf()));
            }

            // Move to parent
            match current.parent() {
                Some(parent) => current = parent,
                None => break,
            }
        }

        // Rule 3: FAIL FAST
        bail!(
            "Cannot detect Rustwork workspace from: {}\n\
            \n\
            Rustwork requires a microservices workspace structure:\n\
            ./\n\
            └── Backend/\n\
                └── services/\n\
                    ├── <service1>/\n\
                    └── <service2>/\n\
            \n\
            Create a new workspace with: rustwork new auth,user,session\n\
            Or specify the workspace path with: --path <path>",
            start_path.display()
        );
    }

    /// Check if a directory is a valid workspace root
    fn is_valid_workspace_root(dir: &Path) -> bool {
        // Check 1: Cargo.toml workspace
        if Self::is_cargo_workspace(dir) {
            return true;
        }

        // Check 2: Has Backend/services/ structure (new)
        if dir.join("Backend/services").exists() {
            return true;
        }

        // Check 3: Has services/ structure (legacy)
        if dir.join("services").exists() && count_rust_projects_in_workspace(dir) > 0 {
            return true;
        }

        false
    }

    /// Check if a directory contains a Cargo.toml with [workspace] section
    fn is_cargo_workspace(dir: &Path) -> bool {
        let cargo_toml = dir.join("Cargo.toml");
        if !cargo_toml.exists() {
            return false;
        }

        // Read and parse Cargo.toml
        if let Ok(content) = std::fs::read_to_string(&cargo_toml) {
            // Simple check: look for [workspace] section
            return content.contains("[workspace]");
        }

        false
    }
}
