use std::path::Path;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectLayout {
    Monolith,
    Microservices,
}

/// Détecte le layout du projet (monolithe ou micro-services)
///
/// Un projet est considéré comme micro-services s'il possède :
/// - Un dossier services/ à la racine
/// - Au moins un service avec un .rustwork/manifest.json
pub fn detect_project_layout(project_root: &Path) -> ProjectLayout {
    let services_dir = project_root.join("services");

    // Vérifier la présence du dossier services/
    if !services_dir.exists() || !services_dir.is_dir() {
        return ProjectLayout::Monolith;
    }

    // Vérifier qu'il y a au moins un service avec manifest.json
    if let Ok(entries) = std::fs::read_dir(&services_dir) {
        for entry in entries.flatten() {
            let service_path = entry.path();
            if service_path.is_dir() {
                let manifest_path = service_path.join(".rustwork/manifest.json");
                if manifest_path.exists() {
                    return ProjectLayout::Microservices;
                }
            }
        }
    }

    ProjectLayout::Monolith
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

    #[test]
    fn test_project_layout_enum() {
        assert_ne!(ProjectLayout::Monolith, ProjectLayout::Microservices);
        assert_eq!(ProjectLayout::Monolith, ProjectLayout::Monolith);
    }

    #[test]
    fn test_detect_project_layout_no_services_dir() {
        let temp_dir = std::env::temp_dir().join("test_layout_1");
        let _ = std::fs::create_dir_all(&temp_dir);
        let layout = detect_project_layout(&temp_dir);
        assert_eq!(layout, ProjectLayout::Monolith);
        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_detect_project_layout_empty_services_dir() {
        let temp_dir = std::env::temp_dir().join("test_layout_2");
        let services_dir = temp_dir.join("services");
        let _ = std::fs::create_dir_all(&services_dir);
        let layout = detect_project_layout(&temp_dir);
        assert_eq!(layout, ProjectLayout::Monolith);
        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_detect_project_layout_with_manifest() {
        let temp_dir = std::env::temp_dir().join("test_layout_3");
        let service_dir = temp_dir.join("services/api");
        let rustwork_dir = service_dir.join(".rustwork");
        let _ = std::fs::create_dir_all(&rustwork_dir);
        let _ = std::fs::write(rustwork_dir.join("manifest.json"), "{}");
        let layout = detect_project_layout(&temp_dir);
        assert_eq!(layout, ProjectLayout::Microservices);
        let _ = std::fs::remove_dir_all(&temp_dir);
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

    #[test]
    fn test_project_layout_debug() {
        let layout = ProjectLayout::Monolith;
        let debug_str = format!("{:?}", layout);
        assert!(debug_str.contains("Monolith"));
    }

    #[test]
    fn test_project_layout_clone() {
        let layout1 = ProjectLayout::Microservices;
        let layout2 = layout1;
        assert_eq!(layout1, layout2);
    }
}
