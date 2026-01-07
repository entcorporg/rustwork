use anyhow::Result;
use std::path::Path;
use tokio::fs;
use chrono::Utc;

use crate::templates::{create_env, TemplateContext};
use crate::commands::utils::{to_snake_case, is_rustwork_project, ensure_parent_dir};

pub async fn execute(name: &str) -> Result<()> {
    if !is_rustwork_project() {
        anyhow::bail!("Not in a Rustwork project. Run this command from a project created with 'rustwork new'");
    }

    let snake_name = to_snake_case(name);
    let table_name = format!("{}s", snake_name); // Simple pluralization

    println!("📝 Generating model: {}", name);

    let mut context = TemplateContext::new();
    context.insert("struct_name".to_string(), serde_json::json!(name));
    context.insert("snake_name".to_string(), serde_json::json!(snake_name));
    context.insert("table_name".to_string(), serde_json::json!(table_name));

    let env = create_env();

    // Create model file
    let model_path = Path::new("src/models").join(format!("{}.rs", snake_name));
    let template = env.get_template("model.rs")?;
    let content = template.render(&context)?;
    
    ensure_parent_dir(&model_path).await?;
    fs::write(&model_path, content).await?;
    println!("  Created: {}", model_path.display());

    // Update models/mod.rs
    update_mod_file("src/models/mod.rs", &snake_name).await?;

    // Create service file
    let service_path = Path::new("src/services").join(format!("{}_service.rs", snake_name));
    let template = env.get_template("service.rs")?;
    let content = template.render(&context)?;
    
    ensure_parent_dir(&service_path).await?;
    fs::write(&service_path, content).await?;
    println!("  Created: {}", service_path.display());

    // Update services/mod.rs
    update_mod_file("src/services/mod.rs", &format!("{}_service", snake_name)).await?;

    // Create migration
    create_migration(name, &snake_name, &table_name, &context).await?;

    // Update manifest
    update_manifest("models", name).await?;

    println!("✅ Model '{}' created successfully!", name);
    println!("\nGenerated files:");
    println!("  - src/models/{}.rs", snake_name);
    println!("  - src/services/{}_service.rs", snake_name);
    println!("  - migrations/<timestamp>_create_{}.rs", table_name);
    println!("\nNext steps:");
    println!("  1. Customize the model fields in src/models/{}.rs", snake_name);
    println!("  2. Run migrations to create the database table");

    Ok(())
}

async fn update_mod_file(path: &str, module_name: &str) -> Result<()> {
    let mod_path = Path::new(path);
    let content = fs::read_to_string(mod_path).await
        .unwrap_or_else(|_| String::new());

    let mod_line = format!("pub mod {};\n", module_name);
    
    if content.contains(&mod_line) {
        return Ok(()); // Already exists
    }

    let new_content = if content.contains("// Add your modules here") {
        content.replace("// Add your modules here\n", &format!("pub mod {};\n// Add your modules here\n", module_name))
    } else {
        format!("{}{}", content, mod_line)
    };

    fs::write(mod_path, new_content).await?;
    println!("  Updated: {}", path);

    Ok(())
}

async fn create_migration(
    _name: &str,
    _snake_name: &str,
    table_name: &str,
    context: &TemplateContext,
) -> Result<()> {
    let timestamp = Utc::now().format("%Y%m%d%H%M%S");
    let migration_name = format!("{}_create_{}", timestamp, table_name);
    let migration_path = Path::new("migrations").join(format!("{}.rs", migration_name));

    let env = create_env();
    let template = env.get_template("migration.rs")?;
    let content = template.render(context)?;

    ensure_parent_dir(&migration_path).await?;
    fs::write(&migration_path, content).await?;
    println!("  Created: {}", migration_path.display());

    Ok(())
}

async fn update_manifest(key: &str, value: &str) -> Result<()> {
    let manifest_path = Path::new(".rustwork/manifest.json");
    
    if !manifest_path.exists() {
        return Ok(());
    }

    let content = fs::read_to_string(manifest_path).await?;
    let mut manifest: serde_json::Value = serde_json::from_str(&content)?;

    if let Some(arr) = manifest.get_mut(key).and_then(|v| v.as_array_mut()) {
        if !arr.iter().any(|v| v.as_str() == Some(value)) {
            arr.push(serde_json::json!(value));
        }
    }

    fs::write(manifest_path, serde_json::to_string_pretty(&manifest)?).await?;
    
    Ok(())
}
