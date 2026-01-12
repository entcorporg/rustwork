use anyhow::Result;
use std::path::Path;
use tokio::fs;

use super::new::create_service_in_project;
use crate::mcp::common::workspace_root::WorkspaceRoot;
use crate::templates::create_micro_env;

pub async fn execute(service_name: &str, project_path: Option<&str>) -> Result<()> {
    // Detect workspace root
    let current_dir = std::env::current_dir()?;
    let workspace_root = if let Some(path) = project_path {
        WorkspaceRoot::detect_with_explicit(&current_dir, Some(Path::new(path)))?
    } else {
        WorkspaceRoot::detect(&current_dir)?
    };

    // Find the Backend/services directory
    let backend_services = workspace_root.path().join("Backend/services");
    let legacy_services = workspace_root.path().join("services");
    
    let services_dir = if backend_services.exists() {
        backend_services
    } else if legacy_services.exists() {
        legacy_services
    } else {
        anyhow::bail!(
            "This is not a valid Rustwork workspace.\n\
             Expected structure: Backend/services/ or services/\n\
             \n\
             To create a new workspace:\n\
             rustwork new auth,user,session"
        );
    };

    // Validate service name
    let service_name = service_name.trim().to_lowercase();
    if service_name.is_empty() {
        anyhow::bail!("Service name cannot be empty");
    }
    
    if service_name == "shared" {
        anyhow::bail!("'shared' is a reserved name for the shared library");
    }
    
    if !service_name.chars().next().map(|c| c.is_ascii_lowercase()).unwrap_or(false) {
        anyhow::bail!("Service name must start with a lowercase letter");
    }
    
    if !service_name.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_') {
        anyhow::bail!("Service name must contain only lowercase letters, digits, and underscores");
    }

    // Check if service already exists
    let service_path = services_dir.join(&service_name);
    if service_path.exists() {
        anyhow::bail!("Service '{}' already exists in this workspace", service_name);
    }

    println!("🔧 Adding service '{}' to the workspace...", service_name);
    println!("   Workspace: {}", workspace_root.path().display());

    // Use micro-services template environment
    let env = create_micro_env();

    // Count existing services to determine port
    let existing_services = count_existing_services(&services_dir).await?;
    let service_port = 3001 + existing_services as u16;

    // Create the service
    create_service_in_project(&service_path, &service_name, service_port, &env).await?;

    // Update Backend/Cargo.toml workspace
    let backend_cargo_toml = workspace_root.path().join("Backend/Cargo.toml");
    if backend_cargo_toml.exists() {
        update_workspace_cargo_toml(&backend_cargo_toml, &service_name).await?;
    }

    // Update Backend README if present
    let backend_readme = workspace_root.path().join("Backend/README.md");
    if backend_readme.exists() {
        update_readme(&backend_readme, &service_name).await?;
    }

    println!("✅ Service '{}' added successfully!", service_name);
    println!();
    println!("📁 Location: {}", service_path.display());
    println!();
    println!("🚀 Next steps:");
    println!("   cd Backend/services/{}", service_name);
    println!("   cp .env.example .env");
    println!("   cargo run");
    println!();
    println!("💡 Or start all services:");
    println!("   rustwork dev");

    Ok(())
}

async fn update_readme(readme_path: &Path, service_name: &str) -> Result<()> {
    if !readme_path.exists() {
        return Ok(());
    }

    let content = fs::read_to_string(&readme_path).await?;

    // Chercher la section Services et ajouter le nouveau service
    if content.contains("## Services") {
        let new_service_line = format!("- `services/{}` - {} service", service_name, service_name);

        // Éviter les doublons
        if content.contains(&new_service_line) {
            return Ok(());
        }

        // Trouver la fin de la section Services et ajouter avant la prochaine section
        let lines: Vec<&str> = content.lines().collect();
        let mut new_lines = Vec::new();
        let mut in_services_section = false;
        let mut service_added = false;

        for line in lines {
            if line.starts_with("## Services") {
                in_services_section = true;
                new_lines.push(line.to_string());
            } else if in_services_section && line.starts_with("## ") {
                // Nouvelle section détectée, ajouter le service avant
                if !service_added {
                    new_lines.push(new_service_line.clone());
                    service_added = true;
                }
                in_services_section = false;
                new_lines.push(line.to_string());
            } else if in_services_section && line.starts_with("- `services/") {
                // Ligne de service existante
                new_lines.push(line.to_string());
            } else if in_services_section
                && !line.starts_with("- `services/")
                && !line.trim().is_empty()
            {
                // On a quitté la liste des services (ligne non vide qui n'est pas un service)
                if !service_added {
                    new_lines.push(new_service_line.clone());
                    service_added = true;
                }
                in_services_section = false;
                new_lines.push(line.to_string());
            } else {
                new_lines.push(line.to_string());
            }
        }

        // Si on est toujours dans la section services (fin du fichier), ajouter maintenant
        if in_services_section && !service_added {
            new_lines.push(new_service_line);
        }

        let new_content = new_lines.join("\n") + "\n";
        fs::write(&readme_path, new_content).await?;
        println!("   Updated README.md");
    }

    Ok(())
}

/// Count existing services in the services directory (excluding 'shared')
async fn count_existing_services(services_dir: &Path) -> Result<usize> {
    let mut count = 0;
    let mut entries = fs::read_dir(services_dir).await?;
    
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.is_dir() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                // Skip 'shared' and hidden directories
                if name != "shared" && !name.starts_with('.') {
                    count += 1;
                }
            }
        }
    }
    
    Ok(count)
}

/// Update Backend/Cargo.toml to include the new service in workspace members
async fn update_workspace_cargo_toml(cargo_toml_path: &Path, service_name: &str) -> Result<()> {
    let content = fs::read_to_string(&cargo_toml_path).await?;
    
    // Find the members array and add the new service
    let new_member = format!("    \"services/{}\",", service_name);
    let new_migration = format!("    \"services/{}/migration\",", service_name);
    
    // Find where to insert (before the closing bracket of members)
    if let Some(members_end) = content.find("]\n") {
        let (before, after) = content.split_at(members_end);
        let new_content = format!("{}\n{}\n{}{}", before, new_member, new_migration, after);
        fs::write(&cargo_toml_path, new_content).await?;
        println!("   Updated Backend/Cargo.toml");
    }
    
    Ok(())
}
