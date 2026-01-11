mod implementation;
mod types;

pub use types::{FileChangeEvent, FileWatcher};

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_file_watcher_creation() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path();

        // Create src directory
        let src_dir = project_path.join("src");
        fs::create_dir(&src_dir).unwrap();

        let watcher = FileWatcher::new(project_path);
        assert!(watcher.is_ok());
    }
}
