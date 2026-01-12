mod detector;
mod types;

use crate::mcp::common::protocol::RpcError;
use crate::mcp::common::state::LiveProjectState;
use crate::mcp::rustwork_grpc_list_definitions;
use detector::DriftDetector;
use serde_json::{json, Value};

/// rustwork_grpc_detect_drift - Détection désynchronisations gRPC
///
/// Objectif :
/// - Détecter désynchronisations entre DSL .rwk, .proto et code généré
/// - Identifier génération manquante, build.rs obsolète, proto non aligné
/// - Ne déclenche PAS de build ni de génération
///
/// Source de vérité : DSL .rwk + système de fichiers
pub async fn rustwork_grpc_detect_drift(
    state: Option<&LiveProjectState>,
) -> Result<Value, RpcError> {
    // Vérifier l'état
    let state = state.ok_or_else(|| {
        RpcError::internal_error("No project state available. MCP must be started in workspace.")
    })?;

    // Récupérer les définitions via rustwork_grpc_list_definitions
    let definitions_result =
        rustwork_grpc_list_definitions::rustwork_grpc_list_definitions(Some(state)).await?;

    // Extraire definitions
    let definitions = serde_json::from_value(
        definitions_result
            .get("definitions")
            .ok_or_else(|| RpcError::internal_error("Missing definitions in response"))?
            .clone(),
    )
    .map_err(|e| RpcError::internal_error(format!("Failed to parse definitions: {}", e)))?;

    // Détecter les drifts
    let detector = DriftDetector::new(state.workspace_root.path().to_path_buf(), definitions);
    let drift_result = detector.detect();

    Ok(json!({
        "confidence": "high",
        "context": {
            "workspace": state.workspace_root.path().display().to_string(),
            "scope": "grpc",
        },
        "has_drift": drift_result.has_drift,
        "drifts": drift_result.drifts,
        "impacted_services": drift_result.impacted_services,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mcp::rustwork_grpc_list_definitions::types::GrpcDefinition;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_detect_missing_proto() {
        let temp_dir = TempDir::new().unwrap();
        let workspace = temp_dir.path().to_path_buf();

        // Créer un .rwk sans .proto
        let grpc_dir = workspace.join("grpc");
        fs::create_dir_all(&grpc_dir).unwrap();
        fs::write(grpc_dir.join("user.rwk"), "service UserService {}").unwrap();

        let definitions = vec![GrpcDefinition {
            service_name: "UserService".to_string(),
            source_file: "grpc/user.rwk".to_string(),
            rpcs: vec![],
            messages: vec![],
        }];

        let detector = DriftDetector::new(workspace, definitions);
        let result = detector.detect();

        assert!(result.has_drift);
        assert!(result
            .drifts
            .iter()
            .any(|d| d.kind == types::DriftKind::MissingProto));
    }

    #[test]
    fn test_no_drift_when_all_present() {
        let temp_dir = TempDir::new().unwrap();
        let workspace = temp_dir.path().to_path_buf();

        // Créer structure complète microservices: Backend/services/user/
        // Le detector attend proto/ au niveau du service, pas dans grpc/
        let service_dir = workspace.join("Backend/services/user");
        let grpc_dir = service_dir.join("grpc");
        let proto_dir = service_dir.join("proto");  // proto/ at service level
        let src_grpc_dir = service_dir.join("src/grpc");

        fs::create_dir_all(&grpc_dir).unwrap();
        fs::create_dir_all(&proto_dir).unwrap();
        fs::create_dir_all(&src_grpc_dir).unwrap();

        fs::write(grpc_dir.join("user.rwk"), "service UserService {}").unwrap();
        fs::write(proto_dir.join("user.proto"), "syntax = \"proto3\";").unwrap();
        fs::write(src_grpc_dir.join("user.rs"), "// Generated code").unwrap();
        fs::write(service_dir.join("build.rs"), "fn main() {}").unwrap();

        let definitions = vec![GrpcDefinition {
            service_name: "UserService".to_string(),
            source_file: "Backend/services/user/grpc/user.rwk".to_string(),
            rpcs: vec![],
            messages: vec![],
        }];

        let detector = DriftDetector::new(workspace, definitions);
        let result = detector.detect();

        // Peut avoir des warnings de timestamp mais pas d'erreurs critiques
        let critical_drifts = result
            .drifts
            .iter()
            .filter(|d| d.severity == types::DriftSeverity::Error)
            .count();

        assert_eq!(critical_drifts, 0);
    }
}
