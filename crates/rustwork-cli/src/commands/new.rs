use anyhow::{Context, Result};
use std::path::Path;
use tokio::fs;

use crate::templates::{create_env, TemplateContext};
use super::utils::ensure_parent_dir;

pub async fn execute(project_name: &str) -> Result<()> {
    let project_path = Path::new(project_name);

    if project_path.exists() {
        anyhow::bail!("Directory '{}' already exists", project_name);
    }

    println!("🚀 Creating new Rustwork project: {}", project_name);

    // Create project directory
    fs::create_dir_all(project_path).await?;

    // Setup template context
    let mut context = TemplateContext::new();
    context.insert("project_name".to_string(), serde_json::json!(project_name));

    let env = create_env();

    // Create Cargo.toml
    create_file(
        &project_path.join("Cargo.toml"),
        &env,
        "Cargo.toml",
        &context,
    ).await?;

    // Create src directory structure
    let src_dir = project_path.join("src");
    fs::create_dir_all(&src_dir).await?;

    create_file(
        &src_dir.join("main.rs"),
        &env,
        "main.rs",
        &context,
    ).await?;

    create_file(
        &src_dir.join("app.rs"),
        &env,
        "app.rs",
        &context,
    ).await?;

    create_file(
        &src_dir.join("routes.rs"),
        &env,
        "routes.rs",
        &context,
    ).await?;

    create_file(
        &src_dir.join("errors.rs"),
        &env,
        "errors.rs",
        &context,
    ).await?;

    // Create subdirectories
    for dir in &["controllers", "services", "models", "middlewares"] {
        let dir_path = src_dir.join(dir);
        fs::create_dir_all(&dir_path).await?;
        fs::write(dir_path.join("mod.rs"), "// Add your modules here\n").await?;
    }

    // Create health controller
    let controllers_dir = src_dir.join("controllers");
    create_file(
        &controllers_dir.join("health.rs"),
        &env,
        "health.rs",
        &context,
    ).await?;

    // Update controllers/mod.rs
    fs::write(
        controllers_dir.join("mod.rs"),
        "pub mod health;\n",
    ).await?;

    // Create graphql directory (optional)
    let graphql_dir = src_dir.join("graphql");
    fs::create_dir_all(&graphql_dir).await?;
    fs::write(graphql_dir.join("mod.rs"), "// GraphQL schema (enable with feature 'graphql')\n").await?;

    // Create config directory
    let config_dir = project_path.join("config");
    fs::create_dir_all(&config_dir).await?;

    create_file(
        &config_dir.join("default.toml"),
        &env,
        "default.toml",
        &context,
    ).await?;

    create_file(
        &config_dir.join("dev.toml"),
        &env,
        "dev.toml",
        &context,
    ).await?;

    // Create .env.example
    create_file(
        &project_path.join(".env.example"),
        &env,
        ".env.example",
        &context,
    ).await?;

    // Create .gitignore
    create_file(
        &project_path.join(".gitignore"),
        &env,
        "gitignore",
        &context,
    ).await?;

    // Create migrations directory
    let migrations_dir = project_path.join("migrations");
    fs::create_dir_all(&migrations_dir).await?;
    fs::write(
        migrations_dir.join(".gitkeep"),
        "",
    ).await?;

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
    ).await?;

    println!("✅ Project created successfully!");
    println!("\nNext steps:");
    println!("  cd {}", project_name);
    println!("  cp .env.example .env");
    println!("  # Configure your database in .env");
    println!("  cargo run");
    println!("\nOr use the dev server:");
    println!("  rustwork dev");

    Ok(())
}

async fn create_file(
    path: &Path,
    env: &minijinja::Environment<'_>,
    template_name: &str,
    context: &TemplateContext,
) -> Result<()> {
    ensure_parent_dir(path).await?;
    
    let template = env.get_template(template_name)
        .with_context(|| format!("Failed to get template: {}", template_name))?;
    
    let content = template.render(context)
        .with_context(|| format!("Failed to render template: {}", template_name))?;
    
    fs::write(path, content).await
        .with_context(|| format!("Failed to write file: {}", path.display()))?;
    
    Ok(())
}
