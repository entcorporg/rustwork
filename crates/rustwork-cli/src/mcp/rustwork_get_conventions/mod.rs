mod loader;
mod types;

#[cfg(test)]
mod tests;

use crate::mcp::common::protocol::RpcError;
use crate::mcp::common::state::LiveProjectState;
use loader::ConventionLoader;
use serde::Deserialize;
use serde_json::{json, Value};

/// Paramètres pour rustwork_get_conventions
#[derive(Deserialize, Debug)]
struct ConventionParams {
    /// Catégorie à explorer (optionnel)
    #[serde(default)]
    category: Option<String>,

    /// Chemin complet vers une convention (ex: "database.migrations.naming")
    #[serde(default)]
    path: Option<String>,
}

/// rustwork.getConventions - Get Rustwork framework conventions de manière hiérarchique
///
/// Modes d'utilisation :
/// 1. Sans paramètre : retourne les catégories racines uniquement
/// 2. Avec `category` : retourne les sous-catégories de cette catégorie
/// 3. Avec `path` : retourne la convention exacte à ce chemin
///
/// Règle de priorité : conventions projet > conventions framework
pub async fn rustwork_get_conventions(
    arguments: &Option<Value>,
    state: Option<&LiveProjectState>,
) -> Result<Value, RpcError> {
    // Parser les paramètres
    let params: ConventionParams = if let Some(args) = arguments {
        serde_json::from_value(args.clone())
            .map_err(|e| RpcError::invalid_params(format!("Invalid arguments: {}", e)))?
    } else {
        ConventionParams {
            category: None,
            path: None,
        }
    };

    // Déterminer le workspace root
    let workspace_root = if let Some(state) = state {
        state.workspace_root.path().to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|e| RpcError::internal_error(format!("Failed to get current dir: {}", e)))?
    };

    // Charger les conventions
    let mut loader = ConventionLoader::new();

    loader.load_framework_conventions().map_err(|e| {
        RpcError::internal_error(format!("Failed to load framework conventions: {}", e))
    })?;

    loader
        .load_project_conventions(&workspace_root)
        .map_err(|e| {
            RpcError::internal_error(format!("Failed to load project conventions: {}", e))
        })?;

    // Router selon les paramètres
    let result = if let Some(path) = params.path {
        // Mode 3 : récupérer une convention précise par son chemin
        let convention = loader.get_by_path(&path).ok_or_else(|| {
            RpcError::invalid_params(format!("Convention not found at path: {}", path))
        })?;

        json!({
            "mode": "path",
            "path": path,
            "convention": convention
        })
    } else if let Some(category) = params.category {
        // Mode 2 : récupérer une catégorie et ses enfants directs
        let category_view = loader
            .get_category(&category)
            .ok_or_else(|| RpcError::invalid_params(format!("Category not found: {}", category)))?;

        json!({
            "mode": "category",
            "category": category_view
        })
    } else {
        // Mode 1 : récupérer uniquement les catégories racines
        let root_categories = loader.get_root_categories();

        json!({
            "mode": "root",
            "categories": root_categories,
            "hint": "Use 'category' parameter to explore a specific category, or 'path' to get a precise convention"
        })
    };

    Ok(result)
}
