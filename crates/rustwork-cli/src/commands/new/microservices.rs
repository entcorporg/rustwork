use anyhow::Result;
use std::path::Path;
use tokio::fs;

use crate::templates::{create_micro_env, TemplateContext};

/// Crée un workspace micro-services Rustwork
///
/// Structure générée:
/// ```text
/// ./
/// ├── .vscode/
/// │   ├── settings.json
/// │   └── mcp.example.json
/// ├── Backend/
/// │   ├── services/
/// │   │   ├── <service1>/
/// │   │   ├── <service2>/
/// │   │   └── shared/
/// │   └── README.md
/// ```
pub async fn create_microservices_workspace(
    services: Vec<String>,
    create_shared: bool,
) -> Result<()> {
    let root_path = Path::new(".");

    // Check if Backend already exists
    let backend_path = root_path.join("Backend");
    if backend_path.exists() {
        anyhow::bail!(
            "Backend/ directory already exists in current directory.\n\
             Please run this command from an empty directory or remove the existing Backend folder."
        );
    }

    println!("🚀 Creating Rustwork microservices workspace");
    println!("   Services: {}", services.join(", "));
    if create_shared {
        println!("   Shared library: enabled");
    }

    // Create .vscode at workspace root
    let vscode_dir = root_path.join(".vscode");
    fs::create_dir_all(&vscode_dir).await?;

    let absolute_root_path = root_path
        .canonicalize()
        .unwrap_or_else(|_| root_path.to_path_buf())
        .to_string_lossy()
        .to_string();

    let workspace_name = std::env::current_dir()
        .ok()
        .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
        .unwrap_or_else(|| "rustwork-workspace".to_string());

    let mut context = TemplateContext::new();
    context.insert(
        "project_name".to_string(),
        serde_json::json!(workspace_name),
    );
    context.insert(
        "project_path".to_string(),
        serde_json::json!(absolute_root_path),
    );
    context.insert("services".to_string(), serde_json::json!(services));

    // Use micro-services template environment
    let env = create_micro_env();

    // Create VS Code configuration
    super::utils::create_file(
        &vscode_dir.join("mcp.example.json"),
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

    // Create Backend directory structure
    fs::create_dir_all(&backend_path).await?;

    let services_dir = backend_path.join("services");
    fs::create_dir_all(&services_dir).await?;

    // Create each service with unique port
    for (index, service_name) in services.iter().enumerate() {
        println!("   📦 Creating service: {}", service_name);
        let service_path = services_dir.join(service_name);
        let service_port = 3001 + index as u16;
        create_service_in_project(&service_path, service_name, service_port, &env).await?;
    }

    // Always create shared library inside services/
    if create_shared {
        println!("   📚 Creating shared library");
        let shared_path = services_dir.join("shared");
        create_shared_library(&shared_path, &workspace_name, &env).await?;
    }

    // Create Backend Cargo.toml workspace
    let mut workspace_members: Vec<String> =
        services.iter().map(|s| format!("services/{}", s)).collect();

    if create_shared {
        workspace_members.push("services/shared".to_string());
    }

    let backend_cargo_toml = format!(
        r#"[workspace]
resolver = "2"
members = [
{}
]
"#,
        workspace_members
            .iter()
            .map(|m| format!("    \"{}\",", m))
            .collect::<Vec<_>>()
            .join("\n")
    );
    fs::write(backend_path.join("Cargo.toml"), backend_cargo_toml).await?;

    // Create Backend README
    let backend_readme = format!(
        r#"# Backend

Rustwork Microservices Backend.

## Services

{}

## Development

Start all services:
```bash
rustwork dev
```

Start a specific service:
```bash
cd services/<service_name>
cargo run
```

## Structure

```
Backend/
└── services/
{}
```

## Adding a New Service

```bash
rustwork add-service <name>
```
"#,
        services
            .iter()
            .map(|s| format!("- **{}** - {} service", s, s))
            .collect::<Vec<_>>()
            .join("\n"),
        {
            let shared_str = "shared".to_string();
            services
                .iter()
                .chain(if create_shared {
                    Some(&shared_str)
                } else {
                    None
                })
                .map(|s| format!("    ├── {}/", s))
                .collect::<Vec<_>>()
                .join("\n")
        },
    );

    fs::write(backend_path.join("README.md"), backend_readme).await?;

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
.idea/
*.swp
*.swo

# Rustwork
.rustwork/
**/.rustwork/

# Do NOT ignore .vscode - it contains MCP config
!.vscode/
"#,
    )
    .await?;

    // Create root README
    let root_readme = format!(
        r#"# {}

A Rustwork microservices workspace.

## Quick Start

```bash
# Start all services with hot-reload
rustwork dev

# Start MCP server for IDE integration
rustwork dev --mcp
```

## Project Structure

```
./
├── .vscode/          # VS Code + MCP configuration
├── Backend/
│   ├── services/     # All microservices
{}│   └── README.md
└── README.md
```

## Services

{}

## Tools

- `rustwork dev` - Start all services with hot-reload
- `rustwork dev --mcp` - Include MCP server for AI assistance
- `rustwork add-service <name>` - Add a new service
- `rustwork grpc build` - Build gRPC services from .rwk files

## VS Code Integration

Copy `.vscode/mcp.example.json` to `.vscode/mcp.json` to enable MCP integration with GitHub Copilot.

---
Built with [Rustwork](https://github.com/rustwork) 🦀
"#,
        workspace_name,
        {
            let shared_str = "shared".to_string();
            services
                .iter()
                .chain(if create_shared {
                    Some(&shared_str)
                } else {
                    None
                })
                .map(|s| format!("│   │   ├── {}/\n", s))
                .collect::<String>()
        },
        services
            .iter()
            .map(|s| format!("- `Backend/services/{}` - {} service", s, s))
            .collect::<Vec<_>>()
            .join("\n"),
    );

    fs::write(root_path.join("README.md"), root_readme).await?;

    println!();
    println!("✅ Rustwork workspace created successfully!");
    println!();
    println!("📁 Structure:");
    println!("   ./");
    println!("   ├── .vscode/");
    println!("   ├── Backend/");
    println!("   │   └── services/");
    for service in &services {
        println!("   │       ├── {}/", service);
    }
    if create_shared {
        println!("   │       └── shared/");
    }
    println!("   └── README.md");
    println!();
    println!("🚀 Next steps:");
    println!("   rustwork dev          # Start all services");
    println!("   rustwork dev --mcp    # Start with MCP server");
    println!();
    println!("🔮 VS Code MCP integration:");
    println!("   cp .vscode/mcp.example.json .vscode/mcp.json");

    Ok(())
}

async fn create_shared_library(
    shared_path: &Path,
    _workspace_name: &str,
    env: &minijinja::Environment<'_>,
) -> Result<()> {
    fs::create_dir_all(shared_path).await?;

    let mut context = TemplateContext::new();
    context.insert("project_name".to_string(), serde_json::json!("shared"));

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
        "//! Shared types across services\n\n// Add your shared types here\n",
    )
    .await?;

    fs::create_dir_all(&src_dir.join("utils")).await?;
    fs::write(
        src_dir.join("utils").join("mod.rs"),
        "//! Shared utilities across services\n\n// Add your shared utilities here\n",
    )
    .await?;

    // Create .rustwork manifest for shared lib
    let rustwork_dir = shared_path.join(".rustwork");
    fs::create_dir_all(&rustwork_dir).await?;
    let manifest = serde_json::json!({
        "version": "0.1.0",
        "type": "shared_library",
        "exports": []
    });
    fs::write(
        rustwork_dir.join("manifest.json"),
        serde_json::to_string_pretty(&manifest)?,
    )
    .await?;

    Ok(())
}

/// Create a service in a microservices project (public for reuse by add_service command)
pub async fn create_service_in_project(
    service_path: &Path,
    service_name: &str,
    service_port: u16,
    env: &minijinja::Environment<'_>,
) -> Result<()> {
    if service_path.exists() {
        anyhow::bail!("Service directory '{}' already exists", service_name);
    }

    // Create service directory
    fs::create_dir_all(service_path).await?;

    // Setup template context with port
    let mut context = TemplateContext::new();
    context.insert("project_name".to_string(), serde_json::json!(service_name));
    context.insert("service_port".to_string(), serde_json::json!(service_port));

    // Create Cargo.toml
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

    // Create migrations directory (SQL files only)
    let migrations_dir = service_path.join("migrations");
    fs::create_dir_all(&migrations_dir).await?;

    // Create README for migrations
    super::utils::create_file(
        &migrations_dir.join("README.md"),
        env,
        "migration_readme.md",
        &context,
    )
    .await?;

    // Create initial migration files
    super::utils::create_file(
        &migrations_dir.join("20240101_000001_initial.up.sql"),
        env,
        "migration_initial_up.sql",
        &context,
    )
    .await?;

    super::utils::create_file(
        &migrations_dir.join("20240101_000001_initial.down.sql"),
        env,
        "migration_initial_down.sql",
        &context,
    )
    .await?;

    // Create data directory for SQLite
    let data_dir = service_path.join("data");
    fs::create_dir_all(&data_dir).await?;
    fs::write(data_dir.join(".gitkeep"), "").await?;

    // Create .rustwork directory for metadata
    let rustwork_dir = service_path.join(".rustwork");
    fs::create_dir_all(&rustwork_dir).await?;
    let manifest = serde_json::json!({
        "version": "0.1.0",
        "type": "service",
        "routes": [],
        "models": [],
        "controllers": ["health"],
    });
    fs::write(
        rustwork_dir.join("manifest.json"),
        serde_json::to_string_pretty(&manifest)?,
    )
    .await?;

    // Create service README
    super::utils::create_file(&service_path.join("README.md"), env, "readme.md", &context).await?;

    Ok(())
}
