/// Types pour rustwork_grpc_test_connectivity
use serde::{Deserialize, Serialize};

/// Résultat du test de connectivité
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectivityTestResult {
    pub service_name: String,
    pub target_address: String,
    pub status: ConnectivityStatus,
    pub latency_ms: Option<u64>,
    pub error: Option<String>,
}

/// État de la connectivité
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConnectivityStatus {
    /// Connexion réussie
    Connected,
    /// Connexion échouée
    Failed,
    /// Timeout dépassé
    Timeout,
}

/// Configuration du test
#[derive(Debug, Clone)]
pub struct TestConfig {
    pub timeout_ms: u64,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            timeout_ms: 5000, // 5 secondes par défaut
        }
    }
}
