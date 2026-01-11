/// Générateur de build.rs pour compiler les fichiers .proto
mod content;
mod dependencies;
mod files;

pub use dependencies::add_grpc_dependencies;
pub use files::generate_service_build_rs;

#[cfg(test)]
mod tests {
    use super::content::generate_build_rs_content;

    #[test]
    fn test_generate_build_rs_content() {
        let content = generate_build_rs_content();
        assert!(content.contains("tonic_build"));
        assert!(content.contains("target/rustwork/grpc"));
        assert!(content.contains("Généré automatiquement par Rustwork"));
    }

    #[test]
    fn test_build_rs_rerun_if_changed() {
        let content = generate_build_rs_content();
        assert!(content.contains("cargo:rerun-if-changed"));
    }
}
