use anyhow::Result;
use chrono::Utc;
use std::path::Path;
use tokio::fs;

use crate::commands::utils::{ensure_parent_dir, is_rustwork_project, to_snake_case};
use crate::templates::{create_monolith_env, TemplateContext};

/// Génère un modèle
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

    let env = create_monolith_env();

    // Create model file
    let model_path = Path::new("src/models").join(format!("{}.rs", snake_name));
    let template = env.get_template("model.rs")?;
    let content = template.render(&context)?;

    ensure_parent_dir(&model_path).await?;
    fs::write(&model_path, content).await?;
    println!("  Created: {}", model_path.display());

    // Update models/mod.rs
    super::common::update_mod_file("src/models/mod.rs", &snake_name).await?;

    // Create service file
    let service_path = Path::new("src/services").join(format!("{}_service.rs", snake_name));
    let template = env.get_template("service.rs")?;
    let content = template.render(&context)?;

    ensure_parent_dir(&service_path).await?;
    fs::write(&service_path, content).await?;
    println!("  Created: {}", service_path.display());

    // Update services/mod.rs
    super::common::update_mod_file("src/services/mod.rs", &format!("{}_service", snake_name))
        .await?;

    // Create migration
    create_migration(name, &snake_name, &table_name, &context).await?;

    // Update manifest
    super::common::update_manifest("models", name).await?;

    println!("✅ Model '{}' created successfully!", name);
    println!("\nGenerated files:");
    println!("  - src/models/{}.rs", snake_name);
    println!("  - src/services/{}_service.rs", snake_name);
    println!("  - migration/src/m<timestamp>_create_{}.rs", table_name);
    println!("\nNext steps:");
    println!(
        "  1. Customize the model fields in src/models/{}.rs",
        snake_name
    );
    println!("  2. Update the migration if needed in migration/src/");
    println!("  3. Run migrations: rustwork db migrate");

    Ok(())
}

async fn create_migration(
    _name: &str,
    _snake_name: &str,
    table_name: &str,
    context: &TemplateContext,
) -> Result<()> {
    // Vérifier que le crate migration existe
    let migration_dir = Path::new("migration");
    if !migration_dir.exists() {
        anyhow::bail!(
            "Migration directory not found. This project may have been created with an older version of Rustwork.\n\
            Please create a 'migration' directory with the proper structure."
        );
    }

    let timestamp = Utc::now().format("%Y%m%d%H%M%S");
    let migration_name = format!("m{}_create_{}", timestamp, table_name);
    let migration_path = migration_dir
        .join("src")
        .join(format!("{}.rs", migration_name));

    let env = create_monolith_env();
    let template = env.get_template("migration.rs")?;
    let content = template.render(context)?;

    ensure_parent_dir(&migration_path).await?;
    fs::write(&migration_path, content).await?;
    println!("  Created: {}", migration_path.display());

    // Mettre à jour migration/src/lib.rs
    update_migration_lib(&migration_name).await?;

    Ok(())
}

async fn update_migration_lib(migration_name: &str) -> Result<()> {
    let lib_path = Path::new("migration/src/lib.rs");

    if !lib_path.exists() {
        anyhow::bail!("migration/src/lib.rs not found");
    }

    let content = fs::read_to_string(lib_path).await?;

    // Vérifier si la migration est déjà référencée
    if content.contains(&format!("mod {};", migration_name)) {
        return Ok(()); // Déjà référencée
    }

    // Trouver où insérer le mod
    let mod_line = format!("mod {};\n", migration_name);

    // Insérer après "pub use sea_orm_migration::prelude::*;" et avant "pub struct Migrator;"
    let new_content = if let Some(pos) = content.find("pub struct Migrator;") {
        let (before, after) = content.split_at(pos);

        // Ajouter le mod avant "pub struct Migrator;"
        let before_with_mod = if before.ends_with('\n') {
            format!("{}{}\n", before, mod_line.trim_end())
        } else {
            format!("{}\n{}\n", before, mod_line.trim_end())
        };

        format!("{}{}", before_with_mod, after)
    } else {
        // Fallback: ajouter à la fin de la section des mods
        content.replace(
            "pub use sea_orm_migration::prelude::*;",
            &format!("pub use sea_orm_migration::prelude::*;\n\n{}", mod_line),
        )
    };

    // Maintenant, ajouter la migration à la liste dans migrations()
    let final_content = add_migration_to_vector(&new_content, migration_name)?;

    fs::write(lib_path, final_content).await?;
    println!("  Updated: migration/src/lib.rs");

    Ok(())
}

fn add_migration_to_vector(content: &str, migration_name: &str) -> Result<String> {
    // Trouver le vecteur dans migrations()
    let migration_box = format!("            Box::new({}::Migration),\n", migration_name);

    // Chercher la dernière Box::new avant le ]
    if let Some(pos) = content.rfind("Box::new(") {
        // Trouver la fin de cette ligne
        if let Some(end_pos) = content[pos..].find("),") {
            let insert_pos = pos + end_pos + 2; // Après "),\n"

            // Vérifier si on a déjà un saut de ligne
            let (before, after) = content.split_at(insert_pos);

            return Ok(format!("{}{}{}", before, migration_box, after));
        }
    }

    // Fallback: remplacer le dernier ] avant }
    Ok(content.replace(
        "        ]\n    }",
        &format!("{}        ]\n    }}", migration_box.trim_end()),
    ))
}
