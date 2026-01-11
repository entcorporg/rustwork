/// Service Resolution Module
///
/// CRITICAL: Determines which microservice a file belongs to.
/// No guessing, no heuristics - only explicit service boundaries.
mod helpers;
mod resolution;
mod types;

pub use types::ServiceResolver;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_resolve_service() {
        let temp_dir = std::env::temp_dir().join("test_service_resolver");
        let services_dir = temp_dir.join("services");
        let user_service = services_dir.join("user");
        let user_src = user_service.join("src");
        fs::create_dir_all(&user_src).unwrap();

        let test_file = user_src.join("lib.rs");
        fs::write(&test_file, "pub fn hello() {}").unwrap();

        let resolver = ServiceResolver::new(temp_dir.clone());
        let service = resolver.resolve_service(&test_file).unwrap();

        assert_eq!(service.name, "user");
        assert_eq!(service.root, user_service.canonicalize().unwrap());

        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_file_outside_service_fails() {
        let temp_dir = std::env::temp_dir().join("test_outside_service");
        let src_dir = temp_dir.join("src");
        fs::create_dir_all(&src_dir).unwrap();

        let test_file = src_dir.join("main.rs");
        fs::write(&test_file, "fn main() {}").unwrap();

        let resolver = ServiceResolver::new(temp_dir.clone());
        let result = resolver.resolve_service(&test_file);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("outside any registered micro-service"));

        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_list_services() {
        let temp_dir = std::env::temp_dir().join("test_list_services");
        let services_dir = temp_dir.join("services");

        // Create user service
        let user_src = services_dir.join("user").join("src");
        fs::create_dir_all(&user_src).unwrap();
        fs::write(user_src.join("lib.rs"), "").unwrap();

        // Create auth service
        let auth_src = services_dir.join("auth").join("src");
        fs::create_dir_all(&auth_src).unwrap();
        fs::write(auth_src.join("lib.rs"), "").unwrap();

        let resolver = ServiceResolver::new(temp_dir.clone());
        let services = resolver.list_services().unwrap();

        assert_eq!(services.len(), 2);
        assert!(services.contains(&"user".to_string()));
        assert!(services.contains(&"auth".to_string()));

        fs::remove_dir_all(&temp_dir).ok();
    }
}
