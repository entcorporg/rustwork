mod constructor;
mod events;
mod scan;
mod types;
mod watchers;

pub use types::LiveProjectState;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_live_project_state_creation() {
        let temp_dir = TempDir::new().unwrap();
        let src_dir = temp_dir.path().join("src");
        fs::create_dir(&src_dir).unwrap();

        let state = LiveProjectState::new(temp_dir.path().to_path_buf()).unwrap();

        assert_eq!(state.project_path, temp_dir.path());

        let code_index = state.code_index.read().await;
        assert_eq!(code_index.files.len(), 0);
    }

    #[tokio::test]
    async fn test_initial_scan_empty_project() {
        let temp_dir = TempDir::new().unwrap();
        let src_dir = temp_dir.path().join("src");
        fs::create_dir(&src_dir).unwrap();

        let state = LiveProjectState::new(temp_dir.path().to_path_buf()).unwrap();
        let result = state.initial_scan().await;

        // Should succeed even with empty project
        assert!(result.is_ok());
    }
}
