use anyhow::Result;
use std::path::Path;
use tokio::fs;

use crate::templates::{create_monolith_env, TemplateContext};

/// Crée un projet monolithe
pub async fn create_monolith_project(project_name: &str) -> Result<()> {
    let project_path = Path::new(project_name);
    create_monolith_project_at(project_path, project_name, false).await
}

/// Crée un projet monolithe à un emplacement spécifique
pub async fn create_monolith_project_at(
    project_path: &Path,
    project_name: &str,
    _is_service: bool,
) -> Result<()> {
    if project_path.exists() {
        anyhow::bail!("Directory '{}' already exists", project_name);
    }

    println!(
        "🚀 Creating new Rustwork monolith project: {}",
        project_name
    );

    // Create project directory
    fs::create_dir_all(project_path).await?;

    // Setup template context - SIMPLE AND EXPLICIT
    let mut context = TemplateContext::new();
    context.insert("project_name".to_string(), serde_json::json!(project_name));

    // Use MONOLITH-SPECIFIC environment
    let env = create_monolith_env();

    // Create Cargo.toml (monolith never uses .service variant)
    super::utils::create_file(
        &project_path.join("Cargo.toml"),
        &env,
        "Cargo.toml",
        &context,
    )
    .await?;

    // Create src directory structure
    let src_dir = project_path.join("src");
    fs::create_dir_all(&src_dir).await?;

    super::utils::create_file(&src_dir.join("main.rs"), &env, "main.rs", &context).await?;

    super::utils::create_file(&src_dir.join("app.rs"), &env, "app.rs", &context).await?;

    super::utils::create_file(&src_dir.join("routes.rs"), &env, "routes.rs", &context).await?;

    super::utils::create_file(&src_dir.join("errors.rs"), &env, "errors.rs", &context).await?;

    // Create subdirectories
    for dir in &["controllers", "services", "models", "middlewares"] {
        let dir_path = src_dir.join(dir);
        fs::create_dir_all(&dir_path).await?;
        fs::write(dir_path.join("mod.rs"), "// Add your modules here\n").await?;
    }

    // Create health controller
    let controllers_dir = src_dir.join("controllers");
    super::utils::create_file(
        &controllers_dir.join("health.rs"),
        &env,
        "health.rs",
        &context,
    )
    .await?;

    // Update controllers/mod.rs
    fs::write(controllers_dir.join("mod.rs"), "pub mod health;\n").await?;

    // Create graphql directory (optional)
    let graphql_dir = src_dir.join("graphql");
    fs::create_dir_all(&graphql_dir).await?;
    fs::write(
        graphql_dir.join("mod.rs"),
        "// GraphQL schema (enable with feature 'graphql')\n",
    )
    .await?;

    // Create config directory
    let config_dir = project_path.join("config");
    fs::create_dir_all(&config_dir).await?;

    super::utils::create_file(
        &config_dir.join("default.toml"),
        &env,
        "default.toml",
        &context,
    )
    .await?;

    super::utils::create_file(&config_dir.join("dev.toml"), &env, "dev.toml", &context).await?;

    // Create .env.example
    super::utils::create_file(
        &project_path.join(".env.example"),
        &env,
        ".env.example",
        &context,
    )
    .await?;

    // Create .gitignore
    super::utils::create_file(
        &project_path.join(".gitignore"),
        &env,
        "gitignore",
        &context,
    )
    .await?;

    // Create migration crate
    let migration_dir = project_path.join("migration");
    fs::create_dir_all(&migration_dir).await?;

    super::utils::create_file(
        &migration_dir.join("Cargo.toml"),
        &env,
        "migration_cargo.toml",
        &context,
    )
    .await?;

    let migration_src_dir = migration_dir.join("src");
    fs::create_dir_all(&migration_src_dir).await?;

    super::utils::create_file(
        &migration_src_dir.join("lib.rs"),
        &env,
        "migration_lib.rs",
        &context,
    )
    .await?;

    super::utils::create_file(
        &migration_src_dir.join("m20240101_000001_create_migrations_table.rs"),
        &env,
        "migration_initial.rs",
        &context,
    )
    .await?;

    super::utils::create_file(
        &migration_src_dir.join("main.rs"),
        &env,
        "migration_main.rs",
        &context,
    )
    .await?;

    // Create data directory for SQLite (by default)
    let data_dir = project_path.join("data");
    fs::create_dir_all(&data_dir).await?;
    fs::write(
        data_dir.join(".gitkeep"),
        "# SQLite databases will be stored here\n",
    )
    .await?;

    // Create .rustwork directory for metadata
    let rustwork_dir = project_path.join(".rustwork");
    fs::create_dir_all(&rustwork_dir).await?;

    let manifest = serde_json::json!({
        "version": "0.1.0",
        "routes": [],
        "models": [],
        "controllers": ["health"],
    });
    fs::write(
        rustwork_dir.join("manifest.json"),
        serde_json::to_string_pretty(&manifest)?,
    )
    .await?;

    // Create .vscode directory for VS Code integration
    let vscode_dir = project_path.join(".vscode");
    fs::create_dir_all(&vscode_dir).await?;

    // Add absolute project path to context
    let absolute_project_path = project_path
        .canonicalize()
        .unwrap_or_else(|_| project_path.to_path_buf())
        .to_string_lossy()
        .to_string();
    context.insert(
        "project_path".to_string(),
        serde_json::json!(absolute_project_path),
    );

    super::utils::create_file(
        &vscode_dir.join("mcp.json"),
        &env,
        "vscode_mcp.json",
        &context,
    )
    .await?;

    super::utils::create_file(
        &vscode_dir.join("settings.json"),
        &env,
        "vscode_settings.json",
        &context,
    )
    .await?;

    println!("✅ Project created successfully!");
    println!("\nNext steps:");
    println!("  cd {}", project_name);
    println!("  cp .env.example .env");
    println!("  # Database is SQLite by default (zero config)");
    println!("  # To use PostgreSQL/MySQL, edit .env (see .env.example)");
    println!("  cargo run");
    println!("\nOr use the dev server:");
    println!("  rustwork dev");
    println!("\n🔮 VS Code MCP integration ready!");
    println!("  The project is configured for GitHub Copilot with MCP support.");

    Ok(())
}
