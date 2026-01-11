// Architecture-specific template modules
pub mod common;
pub mod micro;
pub mod micro_shared;
pub mod monolith;

// Legacy modules (kept for backwards compatibility during migration)
pub mod components;
pub mod migration;
pub mod project;
pub mod vscode;

use std::collections::HashMap;

// Re-exports for new architecture-based approach
pub use micro::create_micro_env;
pub use micro_shared::create_micro_shared_env;
pub use monolith::create_monolith_env;

pub type TemplateContext = HashMap<String, serde_json::Value>;

// Legacy function (deprecated, kept for backwards compatibility)
#[deprecated(
    note = "Use create_monolith_env, create_micro_env, or create_micro_shared_env instead"
)]
#[allow(dead_code)]
pub fn create_env() -> minijinja::Environment<'static> {
    // Default to monolith for backwards compatibility
    create_monolith_env()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_context_creation() {
        let mut ctx: TemplateContext = HashMap::new();
        ctx.insert("project_name".to_string(), serde_json::json!("test"));
        assert_eq!(ctx.get("project_name").unwrap(), &serde_json::json!("test"));
    }

    #[test]
    fn test_template_context_multiple_values() {
        let mut ctx: TemplateContext = HashMap::new();
        ctx.insert("name".to_string(), serde_json::json!("app"));
        ctx.insert("version".to_string(), serde_json::json!("1.0.0"));
        ctx.insert("port".to_string(), serde_json::json!(8080));
        assert_eq!(ctx.len(), 3);
    }

    #[test]
    fn test_monolith_env_creation() {
        let env = create_monolith_env();
        assert!(env.get_template("Cargo.toml").is_ok());
    }

    #[test]
    fn test_micro_env_creation() {
        let env = create_micro_env();
        assert!(env.get_template("Cargo.toml").is_ok());
    }

    #[test]
    fn test_micro_shared_env_creation() {
        let env = create_micro_shared_env();
        assert!(env.get_template("Cargo.toml").is_ok());
    }

    #[test]
    fn test_all_envs_have_cargo_toml() {
        assert!(create_monolith_env().get_template("Cargo.toml").is_ok());
        assert!(create_micro_env().get_template("Cargo.toml").is_ok());
        assert!(create_micro_shared_env().get_template("Cargo.toml").is_ok());
    }

    #[test]
    fn test_template_context_json_values() {
        let mut ctx: TemplateContext = HashMap::new();
        ctx.insert("string".to_string(), serde_json::json!("text"));
        ctx.insert("number".to_string(), serde_json::json!(42));
        ctx.insert("boolean".to_string(), serde_json::json!(true));
        ctx.insert("array".to_string(), serde_json::json!(["a", "b"]));
        assert_eq!(ctx.len(), 4);
    }

    #[test]
    #[allow(deprecated)]
    fn test_legacy_create_env_returns_monolith() {
        let env = create_env();
        // Should default to monolith
        assert!(env.get_template("Cargo.toml").is_ok());
    }

    #[test]
    fn test_template_context_override() {
        let mut ctx: TemplateContext = HashMap::new();
        ctx.insert("key".to_string(), serde_json::json!("old"));
        ctx.insert("key".to_string(), serde_json::json!("new"));
        assert_eq!(ctx.get("key").unwrap(), &serde_json::json!("new"));
    }
}
