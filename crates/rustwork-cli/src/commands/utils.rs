use std::path::Path;

/// Convertit PascalCase en snake_case
pub fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c.is_uppercase() && !result.is_empty() {
            result.push('_');
        }
        result.push(c.to_lowercase().next().unwrap());
    }
    
    result
}

/// Vérifie si on est dans un projet Rustwork (présence de Cargo.toml avec rustwork)
pub fn is_rustwork_project() -> bool {
    let cargo_toml = Path::new("Cargo.toml");
    if !cargo_toml.exists() {
        return false;
    }
    
    // Simple check: le fichier existe
    // On pourrait parser le Cargo.toml pour vérifier la dépendance rustwork
    true
}

/// Crée un fichier si le dossier parent n'existe pas
pub async fn ensure_parent_dir(path: &Path) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    Ok(())
}
