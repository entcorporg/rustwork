use anyhow::Result;
use std::path::Path;
use tokio::fs;

use crate::templates::{create_micro_env, create_micro_shared_env, TemplateContext};

/// Crée un projet microservices
pub async fn create_microservices_project(
    project_name: &str,
    services: Vec<String>,
    shared: bool,
) -> Result<()> {
    let root_path = Path::new(project_name);

    if root_path.exists() {
        anyhow::bail!("Directory '{}' already exists", project_name);
    }

    println!("🚀 Creating microservices project: {}", project_name);
    println!(
        "   Architecture: {}",
        if shared { "micro_shared" } else { "micro" }
    );
    println!("   Services: {}", services.join(", "));

    // Create root directory
    fs::create_dir_all(root_path).await?;

    // Create .vscode at root level
    let vscode_dir = root_path.join(".vscode");
    fs::create_dir_all(&vscode_dir).await?;

    let absolute_root_path = root_path
        .canonicalize()
        .unwrap_or_else(|_| root_path.to_path_buf())
        .to_string_lossy()
        .to_string();

    let mut context = TemplateContext::new();
    context.insert("project_name".to_string(), serde_json::json!(project_name));
    context.insert(
        "project_path".to_string(),
        serde_json::json!(absolute_root_path),
    );

    // CRITICAL: Choose the RIGHT template environment based on architecture
    let env = if shared {
        create_micro_shared_env()
    } else {
        create_micro_env()
    };

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

    // Create services directory
    let services_dir = root_path.join("services");
    fs::create_dir_all(&services_dir).await?;

    // Create each service
    for service_name in &services {
        println!("   Creating service: {}", service_name);
        let service_path = services_dir.join(service_name);
        create_service_in_project(&service_path, service_name, shared, &env).await?;
    }

    // Create shared library if requested
    if shared {
        println!("   Creating shared library");
        let shared_path = root_path.join("shared");
        fs::create_dir_all(&shared_path).await?;
        create_shared_library(&shared_path, project_name, &env).await?;
    }

    // Create root README
    let readme_content = format!(
        r#"# {}

Microservices architecture with Rustwork.

## Services

{}

## Getting Started

Each service can be run independently:

```bash
cd services/<service_name>
cargo run
```

Or use the MCP integration in VS Code with GitHub Copilot.

## Structure

- `services/` - Individual microservices
{}
"#,
        project_name,
        services
            .iter()
            .map(|s| format!("- `services/{}` - {} service", s, s))
            .collect::<Vec<_>>()
            .join("\n"),
        if shared {
            "- `shared/` - Shared library across services\n"
        } else {
            ""
        }
    );

    fs::write(root_path.join("README.md"), readme_content).await?;

    // Create root .gitignore
    fs::write(
        root_path.join(".gitignore"),
        r#"# Rust
/target
**/target
Cargo.lock

# Environment
.env
**/.env

# SQLite Database
**/data/*.db
**/data/*.db-shm
**/data/*.db-wal

# IDE
.vscode/
.idea/
*.swp
*.swo

# Rustwork
.rustwork/
**/.rustwork/
"#,
    )
    .await?;

    println!("✅ Microservices project created successfully!");
    println!("\nNext steps:");
    println!("  cd {}", project_name);
    println!("  # Each service is ready to run:");
    for service in &services {
        println!("  cd services/{} && cargo run", service);
    }
    println!("\n🔮 VS Code MCP integration ready at root level!");

    Ok(())
}

async fn create_shared_library(
    shared_path: &Path,
    project_name: &str,
    env: &minijinja::Environment<'_>,
) -> Result<()> {
    fs::create_dir_all(shared_path).await?;

    let mut context = TemplateContext::new();
    context.insert("project_name".to_string(), serde_json::json!(project_name));

    super::utils::create_file(
        &shared_path.join("Cargo.toml"),
        env,
        "shared_cargo.toml",
        &context,
    )
    .await?;

    let src_dir = shared_path.join("src");
    fs::create_dir_all(&src_dir).await?;

    super::utils::create_file(&src_dir.join("lib.rs"), env, "shared_lib.rs", &context).await?;

    // Create modules
    fs::create_dir_all(&src_dir.join("types")).await?;
    fs::write(
        src_dir.join("types").join("mod.rs"),
        "// Shared types across services\n",
    )
    .await?;

    fs::create_dir_all(&src_dir.join("utils")).await?;
    fs::write(
        src_dir.join("utils").join("mod.rs"),
        "// Shared utilities across services\n",
    )
    .await?;

    Ok(())
}

/// Create a service in a microservices project (public for reuse by add_service command)
pub async fn create_service_in_project(
    service_path: &Path,
    service_name: &str,
    _has_shared: bool,
    env: &minijinja::Environment<'_>,
) -> Result<()> {
    if service_path.exists() {
        anyhow::bail!("Service directory '{}' already exists", service_name);
    }

    // Create service directory
    fs::create_dir_all(service_path).await?;

    // Setup template context - SIMPLE AND EXPLICIT
    let mut context = TemplateContext::new();
    context.insert("project_name".to_string(), serde_json::json!(service_name));

    // Create Cargo.toml using template from correct architecture
    super::utils::create_file(
        &service_path.join("Cargo.toml"),
        env,
        "Cargo.toml",
        &context,
    )
    .await?;

    // Create src directory structure
    let src_dir = service_path.join("src");
    fs::create_dir_all(&src_dir).await?;

    super::utils::create_file(&src_dir.join("main.rs"), env, "main.rs", &context).await?;
    super::utils::create_file(&src_dir.join("app.rs"), env, "app.rs", &context).await?;
    super::utils::create_file(&src_dir.join("routes.rs"), env, "routes.rs", &context).await?;
    super::utils::create_file(&src_dir.join("errors.rs"), env, "errors.rs", &context).await?;

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
        env,
        "health.rs",
        &context,
    )
    .await?;

    // Update controllers/mod.rs
    fs::write(controllers_dir.join("mod.rs"), "pub mod health;\n").await?;

    // Create config directory
    let config_dir = service_path.join("config");
    fs::create_dir_all(&config_dir).await?;

    super::utils::create_file(
        &config_dir.join("default.toml"),
        env,
        "default.toml",
        &context,
    )
    .await?;

    super::utils::create_file(&config_dir.join("dev.toml"), env, "dev.toml", &context).await?;

    // Create .env.example
    super::utils::create_file(
        &service_path.join(".env.example"),
        env,
        ".env.example",
        &context,
    )
    .await?;

    // Create .gitignore
    super::utils::create_file(&service_path.join(".gitignore"), env, "gitignore", &context).await?;

    // Create migration crate
    let migration_dir = service_path.join("migration");
    fs::create_dir_all(&migration_dir).await?;

    super::utils::create_file(
        &migration_dir.join("Cargo.toml"),
        env,
        "migration_cargo.toml",
        &context,
    )
    .await?;

    let migration_src_dir = migration_dir.join("src");
    fs::create_dir_all(&migration_src_dir).await?;

    super::utils::create_file(
        &migration_src_dir.join("lib.rs"),
        env,
        "migration_lib.rs",
        &context,
    )
    .await?;

    super::utils::create_file(
        &migration_src_dir.join("m20240101_000001_create_migrations_table.rs"),
        env,
        "migration_initial.rs",
        &context,
    )
    .await?;

    super::utils::create_file(
        &migration_src_dir.join("main.rs"),
        env,
        "migration_main.rs",
        &context,
    )
    .await?;

    // Create data directory for SQLite
    let data_dir = service_path.join("data");
    fs::create_dir_all(&data_dir).await?;
    fs::write(data_dir.join(".gitkeep"), "").await?;

    // Create service README
    super::utils::create_file(&service_path.join("README.md"), env, "readme.md", &context).await?;

    Ok(())
}
