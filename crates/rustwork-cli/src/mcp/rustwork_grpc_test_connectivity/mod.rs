mod tester;
mod types;

use crate::mcp::common::protocol::RpcError;
use crate::mcp::common::state::LiveProjectState;
use serde::Deserialize;
use serde_json::{json, Value};
use tester::ConnectivityTester;
use types::TestConfig;

/// rustwork_grpc_test_connectivity - Tester la connectivité gRPC réelle
///
/// Objectif :
/// - Tester la connexion TCP vers un service gRPC
/// - Mesurer la latence
/// - Retourner une erreur claire si échec
///
/// Contraintes :
/// - Timeout court (5s par défaut, configurable)
/// - Aucun retry implicite
/// - Aucun panic possible
/// - Ne démarre aucun service
pub async fn rustwork_grpc_test_connectivity(
    arguments: &Option<Value>,
    state: Option<&LiveProjectState>,
) -> Result<Value, RpcError> {
    // Parser les paramètres
    #[derive(Deserialize)]
    struct Params {
        service_name: String,
        address: String,
        #[serde(default)]
        timeout_ms: Option<u64>,
    }

    let params: Params = if let Some(args) = arguments {
        serde_json::from_value(args.clone())
            .map_err(|e| RpcError::invalid_params(format!("Invalid arguments: {}", e)))?
    } else {
        return Err(RpcError::invalid_params(
            "Missing required parameters: service_name, address",
        ));
    };

    // Vérifier l'état (optionnel pour ce tool)
    let workspace = state.map(|s| s.workspace_root.path().display().to_string());

    // Configuration du test
    let config = if let Some(timeout) = params.timeout_ms {
        TestConfig {
            timeout_ms: timeout,
        }
    } else {
        TestConfig::default()
    };

    // Tester la connectivité
    let tester = ConnectivityTester::new(config);
    let result = tester.test(&params.service_name, &params.address);

    Ok(json!({
        "confidence": "high",
        "context": {
            "workspace": workspace,
            "service_name": params.service_name,
            "address": params.address,
        },
        "result": result,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_invalid_address() {
        let args = json!({
            "service_name": "TestService",
            "address": "invalid"
        });

        let result = rustwork_grpc_test_connectivity(&Some(args), None).await;
        assert!(result.is_ok());

        let value = result.unwrap();
        let status = value["result"]["status"].as_str().unwrap();
        assert_eq!(status, "failed");
    }

    #[tokio::test]
    async fn test_missing_params() {
        let result = rustwork_grpc_test_connectivity(&None, None).await;
        assert!(result.is_err());
    }
}
