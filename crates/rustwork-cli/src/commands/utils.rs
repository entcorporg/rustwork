use std::path::{Path, PathBuf};

/// Représente un service Rustwork détecté
#[derive(Debug, Clone)]
pub struct RustworkService {
    pub name: String,
    pub path: PathBuf,
}

/// Vérifie si un dossier est un service Rustwork valide
fn is_valid_rustwork_service(path: &Path) -> bool {
    path.join(".rustwork/manifest.json").exists()
        && path.join("Cargo.toml").exists()
        && path.join("src/main.rs").exists()
}

/// Détecte tous les services Rustwork à partir d'un dossier workspace
/// 
/// Scanne dans l'ordre:
/// 1. Backend/services/ (nouvelle structure)
/// 2. services/ (structure legacy)
/// 
/// Ignore le dossier shared/
pub fn detect_rustwork_services(workspace_root: &Path) -> anyhow::Result<Vec<RustworkService>> {
    let mut services = Vec::new();

    // Check Backend/services/ (new structure)
    let backend_services = workspace_root.join("Backend/services");
    if backend_services.exists() && backend_services.is_dir() {
        scan_services_directory(&backend_services, &mut services)?;
    }

    // Check services/ (legacy structure) if no services found yet
    if services.is_empty() {
        let legacy_services = workspace_root.join("services");
        if legacy_services.exists() && legacy_services.is_dir() {
            scan_services_directory(&legacy_services, &mut services)?;
        }
    }

    // If still empty, try scanning directly (for backward compatibility)
    if services.is_empty() {
        if is_valid_rustwork_service(workspace_root) {
            let name = workspace_root
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("service")
                .to_string();
            services.push(RustworkService {
                name,
                path: workspace_root.to_path_buf(),
            });
        }
    }

    Ok(services)
}

/// Scanne un dossier services/ pour trouver les services Rustwork
fn scan_services_directory(
    services_dir: &Path,
    services: &mut Vec<RustworkService>,
) -> anyhow::Result<()> {
    if let Ok(entries) = std::fs::read_dir(services_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                // Skip shared library and hidden directories
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name == "shared" || name.starts_with('.') || name == "target" {
                        continue;
                    }
                    
                    if is_valid_rustwork_service(&path) {
                        services.push(RustworkService {
                            name: name.to_string(),
                            path,
                        });
                    }
                }
            }
        }
    }

    Ok(())
}

/// Convertit PascalCase en snake_case
pub fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let chars = s.chars();

    for c in chars {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_snake_case_simple() {
        assert_eq!(to_snake_case("HelloWorld"), "hello_world");
        assert_eq!(to_snake_case("User"), "user");
        assert_eq!(to_snake_case("APIKey"), "a_p_i_key");
    }

    #[test]
    fn test_to_snake_case_already_lowercase() {
        assert_eq!(to_snake_case("hello"), "hello");
        assert_eq!(to_snake_case("test"), "test");
    }

    #[test]
    fn test_to_snake_case_consecutive_capitals() {
        assert_eq!(to_snake_case("HTTPRequest"), "h_t_t_p_request");
        assert_eq!(to_snake_case("XMLParser"), "x_m_l_parser");
    }

    #[test]
    fn test_to_snake_case_single_char() {
        assert_eq!(to_snake_case("A"), "a");
        assert_eq!(to_snake_case("X"), "x");
    }

    #[test]
    fn test_to_snake_case_empty() {
        assert_eq!(to_snake_case(""), "");
    }

    #[tokio::test]
    async fn test_ensure_parent_dir_creates_nested() {
        let temp_dir = std::env::temp_dir().join("test_ensure_parent");
        let nested_path = temp_dir.join("a/b/c/file.txt");
        ensure_parent_dir(&nested_path).await.unwrap();
        assert!(nested_path.parent().unwrap().exists());
        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    #[tokio::test]
    async fn test_ensure_parent_dir_no_parent() {
        let path = Path::new("file.txt");
        let result = ensure_parent_dir(path).await;
        assert!(result.is_ok());
    }
}
