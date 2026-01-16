/// Workspace Root Detection
///
/// CRITICAL: The workspace root MUST be detected exactly ONCE at startup.
/// This is the single source of truth for all path operations.
///
/// Rustwork is 100% microservices - no monolith support.
///
/// Detection rules (in order):
/// 1. Directory containing `Backend/services/` folder (new structure)
/// 2. Directory containing `services/` folder (legacy structure)
/// 3. Cargo.toml with [workspace] section
///
/// For microservices layouts like:
///   ./
///     Backend/
///       services/
///         user/
///         auth/
///
/// The workspace root is `./`, NOT the Backend folder or individual service folders.
mod detection;
mod helpers;
mod types;

pub use types::WorkspaceRoot;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    /// Helper to create a valid Rust service (minimal: Cargo.toml + src/main.rs)
    fn create_valid_rust_service(path: &std::path::Path) {
        let src_dir = path.join("src");
        fs::create_dir_all(&src_dir).unwrap();
        fs::write(path.join("Cargo.toml"), "[package]\nname = \"test\"").unwrap();
        fs::write(src_dir.join("main.rs"), "fn main() {}").unwrap();
    }

    /// Helper to create a Rustwork-enhanced service (with .rustwork marker)
    fn create_rustwork_service(path: &std::path::Path) {
        create_valid_rust_service(path);
        let rustwork_dir = path.join(".rustwork");
        fs::create_dir_all(&rustwork_dir).unwrap();
        fs::write(rustwork_dir.join("manifest.json"), "{}").unwrap();
    }

    #[test]
    fn test_detect_microservices_new_structure() {
        let temp_dir = std::env::temp_dir().join("test_micro_new_v4");
        let services_dir = temp_dir.join("Backend/services");

        // Create two valid Rust services
        let user_service = services_dir.join("user");
        let auth_service = services_dir.join("auth");

        fs::create_dir_all(&user_service).unwrap();
        fs::create_dir_all(&auth_service).unwrap();

        create_valid_rust_service(&user_service);
        create_valid_rust_service(&auth_service);

        let root = WorkspaceRoot::detect(&temp_dir).unwrap();
        assert_eq!(root.path(), temp_dir.canonicalize().unwrap());
        assert_eq!(root.services_dir(), temp_dir.join("Backend/services"));

        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_detect_microservices_legacy_structure() {
        let temp_dir = std::env::temp_dir().join("test_micro_legacy_v4");
        let services_dir = temp_dir.join("services");

        // Create two valid Rust services
        let user_service = services_dir.join("user");
        let auth_service = services_dir.join("auth");

        fs::create_dir_all(&user_service).unwrap();
        fs::create_dir_all(&auth_service).unwrap();

        create_valid_rust_service(&user_service);
        create_valid_rust_service(&auth_service);

        let root = WorkspaceRoot::detect(&temp_dir).unwrap();
        assert_eq!(root.path(), temp_dir.canonicalize().unwrap());

        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_rustwork_marker_detection() {
        let temp_dir = std::env::temp_dir().join("test_marker_v4");
        let services_dir = temp_dir.join("Backend/services");

        // Create a Rustwork-enhanced service
        let auth_service = services_dir.join("auth");
        fs::create_dir_all(&auth_service).unwrap();
        create_rustwork_service(&auth_service);

        let root = WorkspaceRoot::detect(&temp_dir).unwrap();
        assert_eq!(root.path(), temp_dir.canonicalize().unwrap());

        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_cargo_workspace_detection() {
        let temp_dir = std::env::temp_dir().join("test_cargo_workspace_v4");
        fs::create_dir_all(&temp_dir).unwrap();

        // Create a Cargo.toml with [workspace]
        fs::write(
            temp_dir.join("Cargo.toml"),
            "[workspace]\nmembers = [\"services/*\"]",
        )
        .unwrap();

        let root = WorkspaceRoot::detect(&temp_dir).unwrap();
        assert_eq!(root.path(), temp_dir.canonicalize().unwrap());

        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_has_shared() {
        let temp_dir = std::env::temp_dir().join("test_shared_v4");
        let services_dir = temp_dir.join("Backend/services");

        // Create service and shared
        let auth_service = services_dir.join("auth");
        let shared_lib = services_dir.join("shared");

        fs::create_dir_all(&auth_service).unwrap();
        fs::create_dir_all(&shared_lib).unwrap();
        create_valid_rust_service(&auth_service);

        let root = WorkspaceRoot::detect(&temp_dir).unwrap();
        assert!(root.has_shared());

        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_cargo_workspace_dir_backend_structure() {
        let temp_dir = std::env::temp_dir().join("test_cargo_workspace_backend");
        let backend_dir = temp_dir.join("Backend");
        let services_dir = backend_dir.join("services");

        // Create Backend/Cargo.toml
        fs::create_dir_all(&backend_dir).unwrap();
        fs::write(
            backend_dir.join("Cargo.toml"),
            "[workspace]\nmembers = [\"services/*\"]",
        )
        .unwrap();

        // Create a service
        let auth_service = services_dir.join("auth");
        fs::create_dir_all(&auth_service).unwrap();
        create_valid_rust_service(&auth_service);

        let root = WorkspaceRoot::detect(&temp_dir).unwrap();
        assert_eq!(
            root.cargo_workspace_dir(),
            temp_dir.join("Backend"),
            "cargo_workspace_dir should return Backend/ when Backend/Cargo.toml exists"
        );

        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_cargo_workspace_dir_legacy_structure() {
        let temp_dir = std::env::temp_dir().join("test_cargo_workspace_legacy");
        let services_dir = temp_dir.join("services");

        // Create directory structure
        fs::create_dir_all(&services_dir).unwrap();

        // Create root Cargo.toml
        fs::write(
            temp_dir.join("Cargo.toml"),
            "[workspace]\nmembers = [\"services/*\"]",
        )
        .unwrap();

        // Create a service
        let auth_service = services_dir.join("auth");
        fs::create_dir_all(&auth_service).unwrap();
        create_valid_rust_service(&auth_service);

        let root = WorkspaceRoot::detect(&temp_dir).unwrap();
        assert_eq!(
            root.cargo_workspace_dir(),
            temp_dir.canonicalize().unwrap(),
            "cargo_workspace_dir should return root path when Backend/Cargo.toml doesn't exist"
        );

        fs::remove_dir_all(&temp_dir).ok();
    }
}
