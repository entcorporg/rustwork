mod types;
mod validator;

use crate::mcp::common::protocol::RpcError;
use crate::mcp::common::state::LiveProjectState;
use crate::mcp::rustwork_grpc_list_definitions;
use serde_json::{json, Value};
use validator::GrpcValidator;

/// rustwork_grpc_validate_workspace - Validation cohérence globale gRPC
///
/// Objectif :
/// - Valider la cohérence globale gRPC du workspace
/// - Détecter services/RPCs orphelins
/// - Détecter dépendances manquantes ou circulaires
/// - NE génère PAS de code, NE compile PAS
///
/// Source de vérité : DSL .rwk uniquement
pub async fn rustwork_grpc_validate_workspace(
    state: Option<&LiveProjectState>,
) -> Result<Value, RpcError> {
    // Vérifier l'état
    let state = state.ok_or_else(|| {
        RpcError::internal_error("No project state available. MCP must be started in workspace.")
    })?;

    // Récupérer les définitions via rustwork_grpc_list_definitions
    let definitions_result =
        rustwork_grpc_list_definitions::rustwork_grpc_list_definitions(Some(state)).await?;

    // Extraire definitions et dependencies
    let definitions = serde_json::from_value(
        definitions_result
            .get("definitions")
            .ok_or_else(|| RpcError::internal_error("Missing definitions in response"))?
            .clone(),
    )
    .map_err(|e| RpcError::internal_error(format!("Failed to parse definitions: {}", e)))?;

    let dependencies = serde_json::from_value(
        definitions_result
            .get("dependencies")
            .ok_or_else(|| RpcError::internal_error("Missing dependencies in response"))?
            .clone(),
    )
    .map_err(|e| RpcError::internal_error(format!("Failed to parse dependencies: {}", e)))?;

    // Valider
    let validator = GrpcValidator::new(definitions, dependencies);
    let validation_result = validator.validate();

    Ok(json!({
        "confidence": "high",
        "context": {
            "workspace": state.workspace_root.path().display().to_string(),
            "scope": "grpc",
        },
        "status": validation_result.status,
        "issues": validation_result.issues,
        "impacted_services": validation_result.impacted_services,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mcp::rustwork_grpc_list_definitions::types::{
        GrpcDefinition, RpcDefinition, ServiceDependency,
    };

    #[test]
    fn test_validation_valid_workspace() {
        let definitions = vec![
            GrpcDefinition {
                service_name: "UserService".to_string(),
                source_file: "grpc/user.rwk".to_string(),
                rpcs: vec![RpcDefinition {
                    name: "GetUser".to_string(),
                    input_type: "GetUserRequest".to_string(),
                    output_type: "User".to_string(),
                }],
                messages: vec![],
            },
            GrpcDefinition {
                service_name: "AuthService".to_string(),
                source_file: "grpc/auth.rwk".to_string(),
                rpcs: vec![RpcDefinition {
                    name: "Login".to_string(),
                    input_type: "LoginRequest".to_string(),
                    output_type: "LoginResponse".to_string(),
                }],
                messages: vec![],
            },
        ];

        let dependencies = vec![ServiceDependency {
            from_service: "AuthService".to_string(),
            to_service: "UserService".to_string(),
            used_messages: vec!["GetUserRequest".to_string()],
        }];

        let validator = GrpcValidator::new(definitions, dependencies);
        let result = validator.validate();

        assert_eq!(result.status, types::ValidationStatus::Valid);
    }

    #[test]
    fn test_validation_circular_dependency() {
        let definitions = vec![
            GrpcDefinition {
                service_name: "ServiceA".to_string(),
                source_file: "grpc/a.rwk".to_string(),
                rpcs: vec![],
                messages: vec![],
            },
            GrpcDefinition {
                service_name: "ServiceB".to_string(),
                source_file: "grpc/b.rwk".to_string(),
                rpcs: vec![],
                messages: vec![],
            },
        ];

        let dependencies = vec![
            ServiceDependency {
                from_service: "ServiceA".to_string(),
                to_service: "ServiceB".to_string(),
                used_messages: vec![],
            },
            ServiceDependency {
                from_service: "ServiceB".to_string(),
                to_service: "ServiceA".to_string(),
                used_messages: vec![],
            },
        ];

        let validator = GrpcValidator::new(definitions, dependencies);
        let result = validator.validate();

        assert_eq!(result.status, types::ValidationStatus::Invalid);
        assert!(result
            .issues
            .iter()
            .any(|i| i.kind == types::IssueKind::CircularDependency));
    }

    #[test]
    fn test_validation_undefined_dependency() {
        let definitions = vec![GrpcDefinition {
            service_name: "ServiceA".to_string(),
            source_file: "grpc/a.rwk".to_string(),
            rpcs: vec![],
            messages: vec![],
        }];

        let dependencies = vec![ServiceDependency {
            from_service: "ServiceA".to_string(),
            to_service: "NonExistentService".to_string(),
            used_messages: vec![],
        }];

        let validator = GrpcValidator::new(definitions, dependencies);
        let result = validator.validate();

        assert_eq!(result.status, types::ValidationStatus::Invalid);
        assert!(result
            .issues
            .iter()
            .any(|i| i.kind == types::IssueKind::UndefinedDependency));
    }
}
