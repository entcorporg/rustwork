use super::content::{generate_build_rs_content, generate_service_build_rs_content};
use std::fs;
use std::path::Path;

/// Génère build.rs pour un service spécifique
pub fn generate_service_build_rs(
    service_path: &Path,
    proto_dir: &Path,
) -> Result<(), std::io::Error> {
    let build_rs_path = service_path.join("build.rs");

    // Vérifier si build.rs existe déjà
    if build_rs_path.exists() {
        let content = fs::read_to_string(&build_rs_path)?;

        // Si c'est un build.rs géré par Rustwork, on peut le mettre à jour
        if !content.contains("Généré automatiquement par Rustwork") {
            eprintln!(
                "⚠  build.rs existe déjà et n'est pas géré par Rustwork dans {}",
                service_path.display()
            );
            eprintln!("   Veuillez l'intégrer manuellement ou le supprimer");
            return Ok(());
        }
    }

    // Créer/mettre à jour build.rs
    fs::write(&build_rs_path, generate_service_build_rs_content(proto_dir))?;
    println!("  ✓ build.rs généré");

    Ok(())
}

/// Assure l'existence du fichier build.rs
#[allow(dead_code)]
pub fn ensure_build_rs(project_root: &Path) -> Result<bool, std::io::Error> {
    let build_rs_path = project_root.join("build.rs");

    // Si le fichier existe déjà, vérifier s'il est géré par Rustwork
    if build_rs_path.exists() {
        let content = fs::read_to_string(&build_rs_path)?;

        // Si c'est un build.rs géré par Rustwork, on peut le mettre à jour
        if content.contains("Généré automatiquement par Rustwork") {
            fs::write(&build_rs_path, generate_build_rs_content())?;
            println!("✓ build.rs mis à jour");
            return Ok(true);
        } else {
            // build.rs existe mais n'est pas géré par Rustwork
            eprintln!("⚠ build.rs existe déjà et n'est pas géré par Rustwork");
            eprintln!("  Veuillez ajouter manuellement la compilation proto ou supprimer build.rs");
            return Ok(false);
        }
    }

    // Créer un nouveau build.rs
    fs::write(&build_rs_path, generate_build_rs_content())?;
    println!("✓ build.rs créé");

    Ok(true)
}

/// Vérifie et crée le dossier pour le code généré
#[allow(dead_code)]
pub fn ensure_generated_dir(project_root: &Path) -> Result<(), std::io::Error> {
    let generated_dir = project_root.join("src/grpc/generated");

    if !generated_dir.exists() {
        fs::create_dir_all(&generated_dir)?;

        // Créer un .gitignore pour ignorer le code généré
        let gitignore_path = generated_dir.join(".gitignore");
        fs::write(gitignore_path, "*\n!.gitignore\n")?;
    }

    Ok(())
}
