mod analyzer;
mod types;

use crate::mcp::common::protocol::RpcError;
use crate::mcp::common::state::LiveProjectState;
use analyzer::ServiceStatusAnalyzer;
use serde::Deserialize;
use serde_json::{json, Value};

/// rustwork_grpc_get_service_status - Exposer l'état réel d'un service gRPC
///
/// Objectif :
/// - Vérifier si un service gRPC est connu
/// - Vérifier la présence et validité du fichier .rwk
/// - Vérifier la présence du code généré
/// - Identifier les incohérences
///
/// Ce tool ne démarre rien, ne modifie rien, n'appelle rien
pub async fn rustwork_grpc_get_service_status(
    arguments: &Option<Value>,
    state: Option<&LiveProjectState>,
) -> Result<Value, RpcError> {
    // Parser les paramètres
    #[derive(Deserialize)]
    struct Params {
        service_name: String,
    }

    let params: Params = if let Some(args) = arguments {
        serde_json::from_value(args.clone())
            .map_err(|e| RpcError::invalid_params(format!("Invalid arguments: {}", e)))?
    } else {
        return Err(RpcError::invalid_params(
            "Missing required parameter: service_name",
        ));
    };

    // Vérifier l'état
    let state = state.ok_or_else(|| {
        RpcError::internal_error("No project state available. MCP must be started in workspace.")
    })?;

    // Analyser l'état du service
    let analyzer = ServiceStatusAnalyzer::new(state.workspace_root.clone());
    let status = analyzer.analyze(&params.service_name);

    Ok(json!({
        "confidence": "high",
        "context": {
            "workspace": state.workspace_root.path().display().to_string(),
            "service_name": params.service_name,
        },
        "status": status,
    }))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_unknown_service() {
        // Ce test nécessite un workspace valide
        // On le désactive pour l'instant car WorkspaceRoot::new est privé
        // TODO: améliorer les tests avec des fixtures appropriées
    }
}
