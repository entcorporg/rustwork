/// Path normalization module
///
/// CRITICAL: All paths must be normalized before processing.
/// Canonical representation: relative to workspace root, POSIX format.
///
/// Rules enforced:
/// - Path MUST exist on filesystem
/// - Path MUST be within workspace root
/// - Path MUST be a file (not directory)
/// - No symlink trickery to escape workspace
mod normalize;
mod types;

pub use types::NormalizedPath;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_normalized_path() {
        let temp_dir = std::env::temp_dir();
        let project = temp_dir.join("test_project");
        fs::create_dir_all(&project).unwrap();

        let file = project.join("src/main.rs");
        fs::create_dir_all(file.parent().unwrap()).unwrap();
        fs::write(&file, "fn main() {}").unwrap();

        let normalized = NormalizedPath::from_path(&file, &project).unwrap();
        assert_eq!(normalized.as_str(), "src/main.rs");

        fs::remove_dir_all(&project).ok();
    }

    #[test]
    fn test_outside_workspace_fails() {
        let temp_dir = std::env::temp_dir();
        let project = temp_dir.join("test_project");
        let outside = temp_dir.join("outside.rs");

        let result = NormalizedPath::from_path(&outside, &project);
        assert!(result.is_err());
    }
}
