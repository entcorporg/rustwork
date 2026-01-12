mod comparator;
mod git_loader;
mod suggester;
pub mod types;

use crate::grpc::parse_contract;
use crate::mcp::common::protocol::RpcError;
use crate::mcp::common::state::LiveProjectState;
use comparator::ContractComparator;
use git_loader::{get_file_at_ref, verify_git_repository, GitRef};
use serde_json::{json, Value};
use suggester::MigrationSuggester;
use types::{DiffResult, DiffVersionsParams, Severity};

/// rustwork_grpc_diff_versions - Détection de breaking changes gRPC
///
/// Objectif :
/// - Comparer deux versions d'un fichier .rwk (workspace actuel vs référence Git)
/// - Classifier les changements en breaking/compatible/neutral
/// - Générer des suggestions de migration
///
/// Contraintes strictes :
/// - AUCUNE génération de code
/// - AUCUNE modification de fichier
/// - AUCUN build
/// - Analyse statique uniquement
pub async fn rustwork_grpc_diff_versions(
    state: Option<&LiveProjectState>,
    params: DiffVersionsParams,
) -> Result<Value, RpcError> {
    // Vérifier l'état
    let state = state.ok_or_else(|| {
        RpcError::internal_error("No project state available. MCP must be started in workspace.")
    })?;

    let workspace_root = state.workspace_root.path();

    // Vérifier que le workspace est dans un dépôt Git
    verify_git_repository(workspace_root)
        .map_err(|e| RpcError::invalid_params(format!("Git repository error: {}", e)))?;

    // Parser le paramètre compare_with
    let git_ref = GitRef::parse(&params.compare_with)
        .map_err(|e| RpcError::invalid_params(format!("Invalid compare_with: {}", e)))?;

    // Construire le chemin absolu du fichier .rwk
    let rwk_path = workspace_root.join(&params.rwk_path);

    // Vérifier que le fichier existe dans le workspace actuel
    if !rwk_path.exists() {
        return Err(RpcError::invalid_params(format!(
            "File '{}' does not exist in workspace",
            params.rwk_path
        )));
    }

    if !rwk_path
        .extension()
        .map(|ext| ext == "rwk")
        .unwrap_or(false)
    {
        return Err(RpcError::invalid_params(format!(
            "File '{}' is not a .rwk file",
            params.rwk_path
        )));
    }

    // Charger la version actuelle (workspace)
    let current_source = std::fs::read_to_string(&rwk_path)
        .map_err(|e| RpcError::internal_error(format!("Failed to read current version: {}", e)))?;

    let current_contract = parse_contract(&current_source)
        .map_err(|e| RpcError::internal_error(format!("Failed to parse current version: {}", e)))?;

    // Charger la version de référence (Git)
    let reference_source = get_file_at_ref(workspace_root, &rwk_path, &git_ref).map_err(|e| {
        RpcError::internal_error(format!("Failed to load reference version: {}", e))
    })?;

    let reference_contract = parse_contract(&reference_source).map_err(|e| {
        RpcError::internal_error(format!("Failed to parse reference version: {}", e))
    })?;

    // Comparer les deux versions
    let all_changes = ContractComparator::compare(&reference_contract, &current_contract);

    // Séparer breaking et compatible
    let breaking_changes: Vec<_> = all_changes
        .iter()
        .filter(|c| c.severity == Severity::Breaking)
        .cloned()
        .collect();

    let compatible_changes: Vec<_> = all_changes
        .iter()
        .filter(|c| c.severity == Severity::Compatible)
        .cloned()
        .collect();

    let migration_needed = !breaking_changes.is_empty();

    // Générer les suggestions de migration
    let migration_suggestions = if migration_needed {
        MigrationSuggester::generate_suggestions(&all_changes)
    } else {
        vec!["No migration needed. All changes are backward compatible.".to_string()]
    };

    // Construire le résultat
    let result = DiffResult {
        breaking_changes,
        compatible_changes,
        migration_needed,
        migration_suggestions,
    };

    Ok(json!({
        "confidence": "high",
        "context": {
            "workspace": workspace_root.display().to_string(),
            "file": params.rwk_path,
            "compared_with": params.compare_with,
        },
        "result": result,
    }))
}
