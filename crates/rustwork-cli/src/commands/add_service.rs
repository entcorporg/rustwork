use anyhow::Result;
use std::path::Path;
use tokio::fs;

use super::new::create_service_in_project;
use crate::templates::{create_micro_env, create_micro_shared_env};

pub async fn execute(service_name: &str, project_path: &str) -> Result<()> {
    let project_path = Path::new(project_path);

    // Vérifier que le projet existe
    if !project_path.exists() {
        anyhow::bail!(
            "Project directory '{}' does not exist",
            project_path.display()
        );
    }

    // Vérifier que c'est un projet microservices (présence du dossier services/)
    let services_dir = project_path.join("services");
    if !services_dir.exists() || !services_dir.is_dir() {
        anyhow::bail!(
            "This is not a microservices project. The 'services/' directory was not found.\n\
            To add a service, the project must have been created with:\n\
            rustwork new <name> --layout micro --services <service1>,<service2>..."
        );
    }

    // Vérifier que le service n'existe pas déjà
    let service_path = services_dir.join(service_name);
    if service_path.exists() {
        anyhow::bail!("Service '{}' already exists in this project", service_name);
    }

    println!("🔧 Adding service '{}' to the project...", service_name);

    // Détecter si le projet utilise shared/
    let has_shared = project_path.join("shared").exists();

    // Choisir le bon environnement de templates en fonction de l'architecture
    let env = if has_shared {
        create_micro_shared_env()
    } else {
        create_micro_env()
    };

    // Créer le service en réutilisant la logique existante
    create_service_in_project(&service_path, service_name, has_shared, &env).await?;

    // Mettre à jour le README si présent
    update_readme(project_path, service_name).await?;

    println!("✅ Service '{}' added successfully!", service_name);
    println!("\nNext steps:");
    println!("  cd services/{}", service_name);
    println!("  cp .env.example .env");
    println!("  cargo run");
    println!("\nThe MCP server will automatically detect the new service.");

    Ok(())
}

async fn update_readme(project_path: &Path, service_name: &str) -> Result<()> {
    let readme_path = project_path.join("README.md");

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
