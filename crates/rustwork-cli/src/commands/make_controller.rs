use anyhow::Result;
use std::path::Path;
use tokio::fs;

use crate::templates::{create_env, TemplateContext};
use crate::commands::utils::{to_snake_case, is_rustwork_project, ensure_parent_dir};

pub async fn execute(name: &str) -> Result<()> {
    if !is_rustwork_project() {
        anyhow::bail!("Not in a Rustwork project. Run this command from a project created with 'rustwork new'");
    }

    let snake_name = to_snake_case(name);
    let plural_name = format!("{}s", snake_name); // Simple pluralization

    println!("📝 Generating controller: {}", name);

    let mut context = TemplateContext::new();
    context.insert("struct_name".to_string(), serde_json::json!(name));
    context.insert("snake_name".to_string(), serde_json::json!(snake_name));
    context.insert("plural_name".to_string(), serde_json::json!(plural_name));

    let env = create_env();

    // Create controller file
    let controller_path = Path::new("src/controllers").join(format!("{}.rs", snake_name));
    let template = env.get_template("controller.rs")?;
    let content = template.render(&context)?;
    
    ensure_parent_dir(&controller_path).await?;
    fs::write(&controller_path, content).await?;

    println!("  Created: {}", controller_path.display());

    // Update controllers/mod.rs
    update_mod_file("src/controllers/mod.rs", &snake_name).await?;

    // Update routes.rs to add the new routes
    update_routes_file(&snake_name, &plural_name).await?;

    // Update manifest
    update_manifest("controllers", name).await?;

    println!("✅ Controller '{}' created successfully!", name);
    println!("\nRoutes added to src/routes.rs:");
    println!("  GET    /api/{}", plural_name);
    println!("  GET    /api/{}/:id", plural_name);
    println!("  POST   /api/{}", plural_name);
    println!("  PUT    /api/{}/:id", plural_name);
    println!("  DELETE /api/{}/:id", plural_name);

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

async fn update_routes_file(snake_name: &str, plural_name: &str) -> Result<()> {
    let routes_path = Path::new("src/routes.rs");
    let content = fs::read_to_string(routes_path).await?;

    // Check if already exists
    if content.contains(&format!("controllers::{}", snake_name)) {
        println!("  Routes already exist in src/routes.rs");
        return Ok(());
    }

    // Add use statement
    let use_line = format!("use crate::controllers::{};\n", snake_name);
    let new_content = if content.contains("use crate::controllers::health;") {
        content.replace(
            "use crate::controllers::health;\n",
            &format!("use crate::controllers::health;\n{}", use_line),
        )
    } else {
        content.replace(
            "use crate::controllers",
            &format!("{}use crate::controllers", use_line),
        )
    };

    // Add routes
    let routes_block = format!(
        r#"        .route("/api/{}", get({}::index).post({}::create))
        .route("/api/{}/:id", get({}::show).put({}::update).delete({}::delete))"#,
        plural_name, snake_name, snake_name,
        plural_name, snake_name, snake_name, snake_name
    );

    let final_content = if new_content.contains("// Add your routes here") {
        new_content.replace(
            "// Add your routes here",
            &format!("{}\n        // Add your routes here", routes_block),
        )
    } else {
        // Insert before .with_state
        new_content.replace(
            ".with_state(state)",
            &format!("{}\n        .with_state(state)", routes_block),
        )
    };

    // Add routing imports if not present
    let final_content = if !final_content.contains("routing::{get, post, put, delete}") {
        final_content.replace(
            "use axum::{Router, routing::get};",
            "use axum::{Router, routing::{get, post, put, delete}};",
        )
    } else {
        final_content
    };

    fs::write(routes_path, final_content).await?;
    println!("  Updated: src/routes.rs");

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
