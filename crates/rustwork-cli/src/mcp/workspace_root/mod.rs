/// Workspace Root Detection
///
/// CRITICAL: The workspace root MUST be detected exactly ONCE at startup.
/// This is the single source of truth for all path operations.
///
/// Detection rules (in order):
/// 1. Directory containing `.rustwork/` marker
/// 2. Directory containing `services/` folder (microservices)
/// 3. Directory containing `src/` folder (monolith)
///
/// For microservices layouts like:
///   backend/
///     services/
///       user/
///       auth/
///
/// The workspace root is `backend/`, NOT the individual service folders.
mod detection;
mod helpers;
mod types;

pub use types::WorkspaceRoot;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mcp::workspace_root::types::WorkspaceLayout;
    use std::fs;

    #[test]
    fn test_detect_monolith() {
        let temp_dir = std::env::temp_dir().join("test_monolith");
        let src_dir = temp_dir.join("src");
        fs::create_dir_all(&src_dir).unwrap();
        fs::write(src_dir.join("main.rs"), "fn main() {}").unwrap();

        let root = WorkspaceRoot::detect(&temp_dir).unwrap();
        assert_eq!(root.path(), temp_dir.canonicalize().unwrap());
        assert_eq!(root.layout(), WorkspaceLayout::Monolith);

        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_detect_microservices() {
        let temp_dir = std::env::temp_dir().join("test_micro");
        let services_dir = temp_dir.join("services");
        let user_service = services_dir.join("user").join("src");
        fs::create_dir_all(&user_service).unwrap();
        fs::write(user_service.join("lib.rs"), "pub fn hello() {}").unwrap();

        let root = WorkspaceRoot::detect(&temp_dir).unwrap();
        assert_eq!(root.path(), temp_dir.canonicalize().unwrap());
        assert_eq!(root.layout(), WorkspaceLayout::MicroServices);

        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_rustwork_marker_priority() {
        let temp_dir = std::env::temp_dir().join("test_marker");
        let rustwork_dir = temp_dir.join(".rustwork");
        let services_dir = temp_dir.join("services");
        fs::create_dir_all(&rustwork_dir).unwrap();
        fs::create_dir_all(&services_dir).unwrap();

        let root = WorkspaceRoot::detect(&temp_dir).unwrap();
        assert_eq!(root.path(), temp_dir.canonicalize().unwrap());

        fs::remove_dir_all(&temp_dir).ok();
    }
}
