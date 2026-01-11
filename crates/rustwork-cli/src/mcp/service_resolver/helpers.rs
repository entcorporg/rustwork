use super::types::ServiceResolver;
use anyhow::Result;

impl ServiceResolver {
    /// Check if workspace has microservices layout
    #[allow(dead_code)]
    pub fn has_microservices(&self) -> bool {
        let services_dir = self.workspace_root.join("services");
        services_dir.exists() && services_dir.is_dir()
    }

    /// List all available services
    pub fn list_services(&self) -> Result<Vec<String>> {
        let services_dir = self.workspace_root.join("services");

        if !services_dir.exists() || !services_dir.is_dir() {
            return Ok(Vec::new());
        }

        let mut services = Vec::new();

        for entry in std::fs::read_dir(&services_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                // Validate it has src/
                let src_dir = path.join("src");
                if src_dir.exists() && src_dir.is_dir() {
                    if let Some(name) = path.file_name() {
                        services.push(name.to_string_lossy().to_string());
                    }
                }
            }
        }

        Ok(services)
    }
}
