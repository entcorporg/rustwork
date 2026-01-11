use anyhow::Result;
use std::path::Path;
use tokio::fs;

/// Met à jour un fichier mod.rs pour ajouter un module
pub(crate) async fn update_mod_file(path: &str, module_name: &str) -> Result<()> {
    let mod_path = Path::new(path);
    let content = fs::read_to_string(mod_path)
        .await
        .unwrap_or_else(|_| String::new());

    let mod_line = format!("pub mod {};\n", module_name);

    if content.contains(&mod_line) {
        return Ok(()); // Already exists
    }

    let new_content = if content.contains("// Add your modules here") {
        content.replace(
            "// Add your modules here\n",
            &format!("pub mod {};\n// Add your modules here\n", module_name),
        )
    } else {
        format!("{}{}", content, mod_line)
    };

    fs::write(mod_path, new_content).await?;
    println!("  Updated: {}", path);

    Ok(())
}

/// Met à jour le fichier manifest.json
pub(crate) async fn update_manifest(key: &str, value: &str) -> Result<()> {
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
