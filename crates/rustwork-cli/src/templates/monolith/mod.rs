pub mod components;
pub mod migration;
pub mod project;
pub mod vscode;

use minijinja::Environment;

/// Crée un environnement de templates pour l'architecture MONOLITH
pub fn create_monolith_env() -> Environment<'static> {
    let mut env = Environment::new();

    // Project templates
    env.add_template("main.rs", project::MAIN_RS).unwrap();
    env.add_template("app.rs", project::APP_RS).unwrap();
    env.add_template("routes.rs", project::ROUTES_RS).unwrap();
    env.add_template("errors.rs", project::ERRORS_RS).unwrap();
    env.add_template("health.rs", project::HEALTH_RS).unwrap();
    env.add_template("default.toml", project::DEFAULT_TOML)
        .unwrap();
    env.add_template("dev.toml", project::DEV_TOML).unwrap();
    env.add_template(".env.example", project::ENV_EXAMPLE)
        .unwrap();
    env.add_template("Cargo.toml", project::CARGO_TOML).unwrap();
    env.add_template("gitignore", project::GITIGNORE).unwrap();

    // Component templates
    env.add_template("controller.rs", components::CONTROLLER_RS)
        .unwrap();
    env.add_template("model.rs", components::MODEL_RS).unwrap();
    env.add_template("service.rs", components::SERVICE_RS)
        .unwrap();

    // Migration templates
    env.add_template("migration.rs", migration::MIGRATION_RS)
        .unwrap();
    env.add_template("migration_cargo.toml", migration::MIGRATION_CARGO_TOML)
        .unwrap();
    env.add_template("migration_lib.rs", migration::MIGRATION_LIB_RS)
        .unwrap();
    env.add_template("migration_initial.rs", migration::MIGRATION_INITIAL)
        .unwrap();
    env.add_template("migration_main.rs", migration::MIGRATION_MAIN_RS)
        .unwrap();

    // VSCode templates - MONOLITH SPECIFIC
    env.add_template("vscode_mcp.json", vscode::VSCODE_MCP_JSON)
        .unwrap();
    env.add_template("vscode_settings.json", vscode::VSCODE_SETTINGS_JSON)
        .unwrap();

    env
}
