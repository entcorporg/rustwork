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

    /// Helper to create a valid Rustwork microservices project
    fn create_valid_rustwork_project(path: &std::path::Path) {
        // Create microservices structure: Backend/services/test-service/
        let rustwork_dir = path.join(".rustwork");
        let services_dir = path.join("Backend/services");
        let service_dir = services_dir.join("test-service");
        let src_dir = service_dir.join("src");
        
        fs::create_dir_all(&rustwork_dir).unwrap();
        fs::create_dir_all(&src_dir).unwrap();
        fs::write(rustwork_dir.join("manifest.json"), "{}").unwrap();
        fs::write(service_dir.join("Cargo.toml"), "[package]\nname = \"test-service\"").unwrap();
        fs::write(src_dir.join("main.rs"), "fn main() {}").unwrap();
    }

    #[tokio::test]
    async fn test_live_project_state_creation() {
        let temp_dir = TempDir::new().unwrap();
        create_valid_rustwork_project(temp_dir.path());

        let state = LiveProjectState::new(temp_dir.path().to_path_buf()).unwrap();

        // Workspace root should be detected correctly
        assert_eq!(
            state.workspace_root.path(),
            temp_dir.path().canonicalize().unwrap()
        );

        let code_index = state.code_index.read().await;
        assert_eq!(code_index.files.len(), 0);
    }

    #[tokio::test]
    async fn test_initial_scan_empty_project() {
        let temp_dir = TempDir::new().unwrap();
        create_valid_rustwork_project(temp_dir.path());

        let state = LiveProjectState::new(temp_dir.path().to_path_buf()).unwrap();
        let result = state.initial_scan().await;

        // Should succeed with valid project
        assert!(result.is_ok());
    }
}
