#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {
    use super::super::loader::ConventionLoader;
    use super::super::types::ConventionScope;
    use tempfile::TempDir;

    #[test]
    fn test_load_framework_conventions() {
        let mut loader = ConventionLoader::new();
        let result = loader.load_framework_conventions();

        assert!(
            result.is_ok(),
            "Should load framework conventions successfully"
        );

        let root_categories = loader.get_root_categories();
        assert!(!root_categories.is_empty(), "Should have root categories");

        // Vérifier que les catégories principales existent
        let category_ids: Vec<String> = root_categories.iter().map(|c| c.id.clone()).collect();
        assert!(category_ids.contains(&"http".to_string()));
        assert!(category_ids.contains(&"errors".to_string()));
        assert!(category_ids.contains(&"responses".to_string()));
        assert!(category_ids.contains(&"database".to_string()));
    }

    #[test]
    fn test_get_category() {
        let mut loader = ConventionLoader::new();
        loader.load_framework_conventions().unwrap();

        let category = loader.get_category("http");
        assert!(category.is_some(), "Should find 'http' category");

        let category = category.unwrap();
        assert_eq!(category.id, "http");
        assert!(
            category.children.is_some(),
            "HTTP category should have children"
        );
    }

    #[test]
    fn test_get_by_path() {
        let mut loader = ConventionLoader::new();
        loader.load_framework_conventions().unwrap();

        // Test path simple
        let convention = loader.get_by_path("http");
        assert!(convention.is_some(), "Should find convention at 'http'");

        // Test path imbriqué
        let convention = loader.get_by_path("http.handlers");
        assert!(
            convention.is_some(),
            "Should find convention at 'http.handlers'"
        );

        let convention = convention.unwrap();
        assert_eq!(convention.id, "handlers");
        assert!(convention.rules.is_some(), "Handlers should have rules");
    }

    #[test]
    fn test_project_overrides_framework() {
        let temp_dir = TempDir::new().unwrap();
        let rustwork_dir = temp_dir.path().join(".rustwork");
        std::fs::create_dir_all(&rustwork_dir).unwrap();

        // Créer un fichier de conventions projet qui écrase "http"
        let project_conventions = r#"[
  {
    "id": "http",
    "label": "HTTP Project Override",
    "description": "Cette catégorie écrase celle du framework",
    "scope": "project",
    "rules": [
      {
        "id": "project_rule",
        "description": "Une règle spécifique au projet"
      }
    ]
  }
]"#;

        let conventions_path = rustwork_dir.join("conventions.json");
        std::fs::write(&conventions_path, project_conventions).unwrap();

        // Charger les conventions
        let mut loader = ConventionLoader::new();
        loader.load_framework_conventions().unwrap();
        loader.load_project_conventions(temp_dir.path()).unwrap();

        // Vérifier la fusion
        let merged = loader.merge_conventions();

        // Trouver la convention "http"
        let http_convention = merged.iter().find(|c| c.id == "http");
        assert!(http_convention.is_some(), "Should have 'http' convention");

        let http_convention = http_convention.unwrap();
        assert_eq!(
            http_convention.scope,
            ConventionScope::Project,
            "Should be project scope"
        );
        assert_eq!(
            http_convention.label, "HTTP Project Override",
            "Should use project label"
        );

        // Vérifier que les conventions framework non écrasées sont toujours là
        let errors_convention = merged.iter().find(|c| c.id == "errors");
        assert!(
            errors_convention.is_some(),
            "Framework 'errors' should still exist"
        );
        assert_eq!(errors_convention.unwrap().scope, ConventionScope::Framework);
    }

    #[test]
    fn test_no_project_conventions_returns_framework_only() {
        let temp_dir = TempDir::new().unwrap();

        let mut loader = ConventionLoader::new();
        loader.load_framework_conventions().unwrap();
        loader.load_project_conventions(temp_dir.path()).unwrap();

        let merged = loader.merge_conventions();

        // Toutes les conventions devraient être framework
        assert!(merged.iter().all(|c| c.scope == ConventionScope::Framework));
    }

    #[test]
    fn test_path_navigation() {
        let mut loader = ConventionLoader::new();
        loader.load_framework_conventions().unwrap();

        // Test navigation profonde
        let convention = loader.get_by_path("database.migrations");
        assert!(convention.is_some(), "Should find 'database.migrations'");

        let convention = convention.unwrap();
        assert_eq!(convention.id, "migrations");
    }

    #[test]
    fn test_invalid_path_returns_none() {
        let mut loader = ConventionLoader::new();
        loader.load_framework_conventions().unwrap();

        let convention = loader.get_by_path("nonexistent");
        assert!(convention.is_none(), "Should return None for invalid path");

        let convention = loader.get_by_path("http.nonexistent");
        assert!(
            convention.is_none(),
            "Should return None for invalid nested path"
        );
    }

    #[test]
    fn test_root_categories_structure() {
        let mut loader = ConventionLoader::new();
        loader.load_framework_conventions().unwrap();

        let root_categories = loader.get_root_categories();

        for category in root_categories {
            assert!(!category.id.is_empty(), "Category ID should not be empty");
            assert!(
                !category.label.is_empty(),
                "Category label should not be empty"
            );
            assert!(
                !category.description.is_empty(),
                "Category description should not be empty"
            );
        }
    }
}
