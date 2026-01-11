use super::types::NormalizedPath;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

impl NormalizedPath {
    /// Create a normalized path from any input path
    ///
    /// # Rules
    /// - Path MUST exist on filesystem
    /// - Convert to relative path from project_root
    /// - Use POSIX separators (/)
    /// - Strip drive letters and external folders
    /// - Return error if path doesn't exist or is outside workspace
    /// - Return error if path is a directory (not a file)
    pub fn from_path(path: &Path, project_root: &Path) -> Result<Self> {
        // Canonicalize to resolve symlinks and .. paths
        let canonical_path = path
            .canonicalize()
            .context(format!("File does not exist: {}", path.display()))?;

        // Verify it's a file, not a directory
        if canonical_path.is_dir() {
            anyhow::bail!("Path is a directory, not a file: {}", path.display());
        }

        let canonical_root = project_root.canonicalize().context(format!(
            "Project root does not exist: {}",
            project_root.display()
        ))?;

        // Ensure path is within project (prevents symlink escapes)
        let relative = canonical_path
            .strip_prefix(&canonical_root)
            .context(format!(
                "Path '{}' is outside workspace root '{}'",
                path.display(),
                project_root.display()
            ))?;

        // Convert to POSIX format
        let canonical = relative
            .to_str()
            .context("Path contains invalid UTF-8")?
            .replace('\\', "/");

        Ok(Self { canonical })
    }

    /// Create from string (validates existence)
    pub fn from_str(path_str: &str, project_root: &Path) -> Result<Self> {
        let path = if path_str.starts_with('/') || path_str.contains(':') {
            PathBuf::from(path_str)
        } else {
            project_root.join(path_str)
        };

        Self::from_path(&path, project_root)
    }
}
