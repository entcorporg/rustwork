use std::path::PathBuf;

/// Normalized path representation - ALWAYS relative to project root
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NormalizedPath {
    /// Relative path from project root, POSIX format
    pub(super) canonical: String,
}

impl NormalizedPath {
    /// Get the canonical string representation
    pub fn as_str(&self) -> &str {
        &self.canonical
    }

    /// Convert to absolute path
    pub fn to_absolute(&self, project_root: &std::path::Path) -> PathBuf {
        project_root.join(&self.canonical)
    }
}

impl std::fmt::Display for NormalizedPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.canonical)
    }
}

impl AsRef<str> for NormalizedPath {
    fn as_ref(&self) -> &str {
        &self.canonical
    }
}
