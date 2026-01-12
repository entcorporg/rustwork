//! Templates for Rustwork microservices
//!
//! Rustwork is 100% microservices - no monolith support.

// Architecture-specific template module
pub mod common;
pub mod micro;

// Legacy modules (kept for backwards compatibility during migration)
pub mod components;
pub mod migration;
pub mod project;
pub mod vscode;

use std::collections::HashMap;

// Re-export the microservices environment
pub use micro::create_micro_env;

pub type TemplateContext = HashMap<String, serde_json::Value>;

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
    fn test_micro_env_creation() {
        let env = create_micro_env();
        assert!(env.get_template("Cargo.toml").is_ok());
    }

    #[test]
    fn test_micro_env_has_required_templates() {
        let env = create_micro_env();
        assert!(env.get_template("Cargo.toml").is_ok());
        assert!(env.get_template("main.rs").is_ok());
        assert!(env.get_template("app.rs").is_ok());
        assert!(env.get_template("routes.rs").is_ok());
        assert!(env.get_template("health.rs").is_ok());
        assert!(env.get_template("vscode_mcp.json").is_ok());
        assert!(env.get_template("shared_cargo.toml").is_ok());
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
    fn test_template_context_override() {
        let mut ctx: TemplateContext = HashMap::new();
        ctx.insert("key".to_string(), serde_json::json!("old"));
        ctx.insert("key".to_string(), serde_json::json!("new"));
        assert_eq!(ctx.get("key").unwrap(), &serde_json::json!("new"));
    }
}
