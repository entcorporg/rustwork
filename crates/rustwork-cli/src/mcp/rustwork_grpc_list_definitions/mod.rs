mod analyzer;
mod scanner;
pub mod types;

use crate::mcp::common::path_normalization::NormalizedPath;
use crate::mcp::common::protocol::RpcError;
use crate::mcp::common::state::LiveProjectState;
use analyzer::DependencyAnalyzer;
use scanner::RwkScanner;
use serde_json::{json, Value};
use types::{FieldDefinition, GrpcDefinition, MessageDefinition, RpcDefinition};

/// rustwork_grpc_list_definitions - Cartographie complète des définitions gRPC
///
/// Objectif :
/// - Scanner tous les fichiers .rwk du workspace
/// - Exposer services, RPCs et messages
/// - Identifier les dépendances inter-services
///
/// Source de vérité : DSL .rwk uniquement
pub async fn rustwork_grpc_list_definitions(
    state: Option<&LiveProjectState>,
) -> Result<Value, RpcError> {
    // Vérifier l'état
    let state = state.ok_or_else(|| {
        RpcError::internal_error("No project state available. MCP must be started in workspace.")
    })?;

    // Scanner les fichiers .rwk
    let scanner = RwkScanner::new(state.workspace_root.clone());
    let rwk_files = scanner
        .scan_rwk_files()
        .map_err(|e| RpcError::internal_error(format!("Failed to scan .rwk files: {}", e)))?;

    if rwk_files.is_empty() {
        return Ok(json!({
            "confidence": "high",
            "context": {
                "workspace": state.workspace_root.path().display().to_string(),
            },
            "definitions": [],
            "dependencies": [],
            "note": "No .rwk files found in workspace"
        }));
    }

    // Parser chaque fichier .rwk
    let mut definitions = Vec::new();

    for rwk_path in &rwk_files {
        // Lire le contenu
        let source = match std::fs::read_to_string(rwk_path) {
            Ok(s) => s,
            Err(e) => {
                // Ne pas bloquer sur un fichier illisible
                eprintln!("Warning: Failed to read {}: {}", rwk_path.display(), e);
                continue;
            }
        };

        // Parser le contrat
        let contract = match crate::grpc::parse_contract(&source) {
            Ok(c) => c,
            Err(e) => {
                // Ne pas bloquer sur une erreur de parsing
                eprintln!("Warning: Failed to parse {}: {}", rwk_path.display(), e);
                continue;
            }
        };

        // Convertir en définition
        let normalized_path = match NormalizedPath::from_path(rwk_path, state.workspace_root.path())
        {
            Ok(path) => path.as_str().to_string(),
            Err(_) => rwk_path.display().to_string(), // Fallback
        };

        let rpcs = contract
            .service
            .rpcs
            .iter()
            .map(|rpc| RpcDefinition {
                name: rpc.name.clone(),
                input_type: rpc.input_type.clone(),
                output_type: rpc.output_type.clone(),
            })
            .collect();

        let messages = contract
            .messages
            .iter()
            .map(|msg| MessageDefinition {
                name: msg.name.clone(),
                fields: msg
                    .fields
                    .iter()
                    .map(|field| FieldDefinition {
                        name: field.name.clone(),
                        field_type: field.field_type.to_string(),
                    })
                    .collect(),
            })
            .collect();

        definitions.push(GrpcDefinition {
            service_name: contract.service.name.clone(),
            source_file: normalized_path,
            rpcs,
            messages,
        });
    }

    // Analyser les dépendances
    let dependencies = DependencyAnalyzer::analyze(&definitions);

    Ok(json!({
        "confidence": "high",
        "context": {
            "workspace": state.workspace_root.path().display().to_string(),
            "scanned_files": rwk_files.len(),
            "valid_definitions": definitions.len(),
        },
        "definitions": definitions,
        "dependencies": dependencies,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dependency_analysis() {
        let definitions = vec![
            GrpcDefinition {
                service_name: "UserService".to_string(),
                source_file: "grpc/user.rwk".to_string(),
                rpcs: vec![RpcDefinition {
                    name: "GetUser".to_string(),
                    input_type: "GetUserRequest".to_string(),
                    output_type: "User".to_string(),
                }],
                messages: vec![
                    MessageDefinition {
                        name: "GetUserRequest".to_string(),
                        fields: vec![],
                    },
                    MessageDefinition {
                        name: "User".to_string(),
                        fields: vec![],
                    },
                ],
            },
            GrpcDefinition {
                service_name: "OrderService".to_string(),
                source_file: "grpc/order.rwk".to_string(),
                rpcs: vec![RpcDefinition {
                    name: "CreateOrder".to_string(),
                    input_type: "CreateOrderRequest".to_string(),
                    output_type: "Order".to_string(),
                }],
                messages: vec![
                    MessageDefinition {
                        name: "CreateOrderRequest".to_string(),
                        fields: vec![],
                    },
                    MessageDefinition {
                        name: "Order".to_string(),
                        fields: vec![],
                    },
                ],
            },
        ];

        let deps = DependencyAnalyzer::analyze(&definitions);

        // Aucune dépendance car chaque service utilise ses propres messages
        assert_eq!(deps.len(), 0);
    }
}
