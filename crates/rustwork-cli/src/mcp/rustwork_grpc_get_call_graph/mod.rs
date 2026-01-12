mod analyzer;
mod types;

use crate::mcp::common::protocol::RpcError;
use crate::mcp::common::state::LiveProjectState;
use crate::mcp::rustwork_grpc_list_definitions;
use analyzer::CallGraphAnalyzer;
use serde_json::{json, Value};

/// rustwork_grpc_get_call_graph - Cartographie inter-services gRPC
///
/// Objectif :
/// - Fournir la cartographie complète des appels gRPC
/// - Exposer les dépendances entre services
/// - Identifier les services centraux, points d'entrée, isolés
/// - Analyse purement statique basée sur DSL .rwk
///
/// Source de vérité : DSL .rwk uniquement
pub async fn rustwork_grpc_get_call_graph(
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

    // Construire le graphe
    let analyzer = CallGraphAnalyzer::new(definitions, dependencies);
    let call_graph = analyzer.build_graph();

    Ok(json!({
        "confidence": "high",
        "context": {
            "workspace": state.workspace_root.path().display().to_string(),
            "scope": "grpc",
        },
        "graph": {
            "nodes": call_graph.nodes,
            "edges": call_graph.edges,
        },
        "analysis": call_graph.analysis,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mcp::rustwork_grpc_list_definitions::types::{
        GrpcDefinition, RpcDefinition, ServiceDependency,
    };

    #[test]
    fn test_call_graph_simple() {
        let definitions = vec![
            GrpcDefinition {
                service_name: "API".to_string(),
                source_file: "grpc/api.rwk".to_string(),
                rpcs: vec![RpcDefinition {
                    name: "GetData".to_string(),
                    input_type: "GetDataRequest".to_string(),
                    output_type: "GetDataResponse".to_string(),
                }],
                messages: vec![],
            },
            GrpcDefinition {
                service_name: "Database".to_string(),
                source_file: "grpc/database.rwk".to_string(),
                rpcs: vec![RpcDefinition {
                    name: "Query".to_string(),
                    input_type: "QueryRequest".to_string(),
                    output_type: "QueryResponse".to_string(),
                }],
                messages: vec![],
            },
        ];

        let dependencies = vec![ServiceDependency {
            from_service: "API".to_string(),
            to_service: "Database".to_string(),
            used_messages: vec!["QueryRequest".to_string()],
        }];

        let analyzer = CallGraphAnalyzer::new(definitions, dependencies);
        let graph = analyzer.build_graph();

        assert_eq!(graph.nodes.len(), 2);
        assert_eq!(graph.edges.len(), 1);
        assert_eq!(graph.analysis.total_services, 2);
        assert_eq!(graph.analysis.total_dependencies, 1);

        // API doit être un entrypoint (pas de dépendances entrantes)
        let api_node = graph.nodes.iter().find(|n| n.service_name == "API");
        assert!(api_node.is_some());
        assert_eq!(api_node.unwrap().role, types::ServiceRole::Entrypoint);
    }

    #[test]
    fn test_call_graph_isolated_service() {
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

        let dependencies = vec![ServiceDependency {
            from_service: "ServiceA".to_string(),
            to_service: "ServiceB".to_string(),
            used_messages: vec![],
        }];

        let analyzer = CallGraphAnalyzer::new(definitions, dependencies);
        let graph = analyzer.build_graph();

        assert_eq!(graph.analysis.entrypoints.len(), 1);
        assert!(graph.analysis.entrypoints.contains(&"ServiceA".to_string()));
    }
}
