use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

/// Commande `rustwork conventions init`
/// Génère un fichier `.rustwork/conventions.json` avec une structure de base
pub fn conventions_init(workspace_root: Option<PathBuf>) -> Result<()> {
    let workspace_root = workspace_root
        .or_else(|| std::env::current_dir().ok())
        .context("Failed to determine workspace root")?;

    let rustwork_dir = workspace_root.join(".rustwork");
    let conventions_path = rustwork_dir.join("conventions.json");

    // Vérifier si le fichier existe déjà
    if conventions_path.exists() {
        println!("❌ Le fichier .rustwork/conventions.json existe déjà.");
        println!("   Pour le régénérer, supprimez-le d'abord.");
        return Ok(());
    }

    // Créer le dossier .rustwork s'il n'existe pas
    if !rustwork_dir.exists() {
        fs::create_dir_all(&rustwork_dir).context("Failed to create .rustwork directory")?;
    }

    // Charger le template de conventions projet
    let template_content = include_str!("../../data/conventions/template_project_conventions.json");

    // Écrire le fichier
    fs::write(&conventions_path, template_content).context("Failed to write conventions.json")?;

    println!("✅ Fichier .rustwork/conventions.json créé avec succès !");
    println!();
    println!("📝 Ce fichier contient des exemples de conventions projet.");
    println!("   Les conventions projet ont PRIORITÉ ABSOLUE sur celles du framework.");
    println!();
    println!("💡 Vous pouvez :");
    println!("   - Modifier les conventions existantes");
    println!("   - Ajouter vos propres catégories");
    println!("   - Désactiver des conventions en les supprimant");
    println!();
    println!("🔍 Pour explorer les conventions disponibles, utilisez le tool MCP rustwork_get_conventions");

    Ok(())
}
