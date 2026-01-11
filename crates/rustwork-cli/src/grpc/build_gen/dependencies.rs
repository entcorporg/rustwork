use std::fs;
use std::path::Path;

/// Vérifie les dépendances requises dans Cargo.toml
#[allow(dead_code)]
pub fn check_dependencies(project_root: &Path) -> Result<(), String> {
    let cargo_toml_path = project_root.join("Cargo.toml");

    if !cargo_toml_path.exists() {
        return Err("Cargo.toml introuvable".to_string());
    }

    let content = fs::read_to_string(&cargo_toml_path)
        .map_err(|e| format!("Erreur lecture Cargo.toml: {}", e))?;

    let mut missing_deps = Vec::new();

    // Vérifier les dépendances principales
    if !content.contains("tonic = ") {
        missing_deps.push("tonic");
    }
    if !content.contains("prost = ") {
        missing_deps.push("prost");
    }
    if !content.contains("tokio = ") {
        missing_deps.push("tokio");
    }

    // Vérifier les build-dependencies
    if !content.contains("[build-dependencies]") || !content.contains("tonic-build = ") {
        missing_deps.push("tonic-build (build-dependencies)");
    }

    if !missing_deps.is_empty() {
        return Err(format!(
            "Dépendances manquantes dans Cargo.toml:\n  - {}\n\n\
            Ajoutez-les avec:\n  \
            cargo add tonic prost tokio --features tokio/full\n  \
            cargo add tonic-build --build",
            missing_deps.join("\n  - ")
        ));
    }

    Ok(())
}

/// Ajoute les dépendances gRPC au Cargo.toml si elles sont manquantes
pub fn add_grpc_dependencies(project_root: &Path) -> Result<(), String> {
    let cargo_toml_path = project_root.join("Cargo.toml");

    if !cargo_toml_path.exists() {
        return Err("Cargo.toml introuvable".to_string());
    }

    let content = fs::read_to_string(&cargo_toml_path)
        .map_err(|e| format!("Erreur lecture Cargo.toml: {}", e))?;

    let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    let mut modified = false;

    // Trouver ou créer la section [dependencies]
    let deps_idx = lines.iter().position(|l| l.trim() == "[dependencies]");
    let deps_section_start = if let Some(idx) = deps_idx {
        idx
    } else {
        // Ajouter [dependencies] à la fin
        lines.push("".to_string());
        lines.push("[dependencies]".to_string());
        lines.len() - 1
    };

    // Vérifier et ajouter les dépendances
    let deps_to_add = vec![
        ("tonic", "tonic = \"0.11\""),
        ("prost", "prost = \"0.12\""),
        ("async-trait", "async-trait = \"0.1\""),
    ];

    for (dep_name, dep_line) in deps_to_add {
        if !content.contains(&format!("{} = ", dep_name)) {
            // Trouver la fin de la section [dependencies]
            let mut insert_pos = deps_section_start + 1;
            while insert_pos < lines.len() {
                let line = lines[insert_pos].trim();
                if line.starts_with('[') && line != "[dependencies]" {
                    break;
                }
                if !line.is_empty() {
                    insert_pos += 1;
                } else {
                    break;
                }
            }
            lines.insert(insert_pos, dep_line.to_string());
            modified = true;
        }
    }

    // Trouver ou créer la section [build-dependencies]
    let build_deps_idx = lines
        .iter()
        .position(|l| l.trim() == "[build-dependencies]");
    let build_deps_section_start = if let Some(idx) = build_deps_idx {
        idx
    } else {
        // Ajouter [build-dependencies] à la fin
        lines.push("".to_string());
        lines.push("[build-dependencies]".to_string());
        lines.len() - 1
    };

    // Ajouter tonic-build si nécessaire
    if !content.contains("tonic-build = ") {
        let mut insert_pos = build_deps_section_start + 1;
        while insert_pos < lines.len() {
            let line = lines[insert_pos].trim();
            if line.starts_with('[') && line != "[build-dependencies]" {
                break;
            }
            if !line.is_empty() {
                insert_pos += 1;
            } else {
                break;
            }
        }
        lines.insert(insert_pos, "tonic-build = \"0.11\"".to_string());
        modified = true;
    }

    if modified {
        let new_content = lines.join("\n") + "\n";
        fs::write(&cargo_toml_path, new_content)
            .map_err(|e| format!("Erreur écriture Cargo.toml: {}", e))?;
        println!("✓ Dépendances gRPC ajoutées au Cargo.toml");
    }

    Ok(())
}
