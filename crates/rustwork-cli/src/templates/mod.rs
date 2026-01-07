pub mod project;

use minijinja::Environment;
use std::collections::HashMap;

pub fn create_env() -> Environment<'static> {
    let mut env = Environment::new();
    
    // Ajoute tous les templates
    env.add_template("main.rs", project::MAIN_RS).unwrap();
    env.add_template("app.rs", project::APP_RS).unwrap();
    env.add_template("routes.rs", project::ROUTES_RS).unwrap();
    env.add_template("errors.rs", project::ERRORS_RS).unwrap();
    env.add_template("health.rs", project::HEALTH_RS).unwrap();
    env.add_template("default.toml", project::DEFAULT_TOML).unwrap();
    env.add_template("dev.toml", project::DEV_TOML).unwrap();
    env.add_template(".env.example", project::ENV_EXAMPLE).unwrap();
    env.add_template("Cargo.toml", project::CARGO_TOML).unwrap();
    env.add_template("gitignore", project::GITIGNORE).unwrap();
    env.add_template("controller.rs", project::CONTROLLER_RS).unwrap();
    env.add_template("model.rs", project::MODEL_RS).unwrap();
    env.add_template("service.rs", project::SERVICE_RS).unwrap();
    env.add_template("migration.rs", project::MIGRATION_RS).unwrap();
    
    env
}

pub type TemplateContext = HashMap<String, serde_json::Value>;
