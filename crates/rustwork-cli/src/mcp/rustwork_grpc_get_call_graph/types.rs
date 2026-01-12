/// Types pour rustwork_grpc_get_call_graph
use serde::{Deserialize, Serialize};

/// Graphe d'appels gRPC complet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallGraph {
    pub nodes: Vec<ServiceNode>,
    pub edges: Vec<ServiceEdge>,
    pub analysis: GraphAnalysis,
}

/// Nœud du graphe (service)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceNode {
    pub service_name: String,
    pub source_file: String,
    pub role: ServiceRole,
    pub rpc_count: usize,
}

/// Rôle d'un service dans le graphe
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ServiceRole {
    /// Point d'entrée (pas de dépendance entrante)
    Entrypoint,
    /// Service central (beaucoup de dépendances entrantes)
    Central,
    /// Service intermédiaire
    Intermediate,
    /// Service isolé (pas de dépendances)
    Isolated,
}

/// Arête du graphe (dépendance)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEdge {
    pub from_service: String,
    pub to_service: String,
    pub rpcs_used: Vec<String>,
    pub messages_used: Vec<String>,
}

/// Analyse du graphe
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphAnalysis {
    pub total_services: usize,
    pub total_dependencies: usize,
    pub entrypoints: Vec<String>,
    pub central_services: Vec<String>,
    pub isolated_services: Vec<String>,
    pub max_dependency_depth: usize,
}

#[allow(dead_code)]
impl CallGraph {
    /// Crée un graphe vide
    pub fn empty() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            analysis: GraphAnalysis {
                total_services: 0,
                total_dependencies: 0,
                entrypoints: Vec::new(),
                central_services: Vec::new(),
                isolated_services: Vec::new(),
                max_dependency_depth: 0,
            },
        }
    }
}
