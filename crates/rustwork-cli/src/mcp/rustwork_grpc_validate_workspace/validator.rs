/// Validateur de cohérence gRPC
use super::types::{IssueKind, IssueSeverity, ValidationIssue, ValidationResult};
use crate::mcp::rustwork_grpc_list_definitions::types::{GrpcDefinition, ServiceDependency};
use std::collections::{HashMap, HashSet};

pub struct GrpcValidator {
    definitions: Vec<GrpcDefinition>,
    dependencies: Vec<ServiceDependency>,
}

impl GrpcValidator {
    pub fn new(definitions: Vec<GrpcDefinition>, dependencies: Vec<ServiceDependency>) -> Self {
        Self {
            definitions,
            dependencies,
        }
    }

    /// Valide l'ensemble du workspace gRPC
    pub fn validate(&self) -> ValidationResult {
        let mut result = ValidationResult::valid();

        // 1. Vérifier présence des fichiers .rwk (via definitions)
        self.check_missing_rwk(&mut result);

        // 2. Détecter services/RPCs orphelins
        self.check_orphaned_elements(&mut result);

        // 3. Détecter dépendances non définies
        self.check_undefined_dependencies(&mut result);

        // 4. Détecter dépendances circulaires
        self.check_circular_dependencies(&mut result);

        result
    }

    /// Vérifie présence des fichiers .rwk
    fn check_missing_rwk(&self, result: &mut ValidationResult) {
        if self.definitions.is_empty() {
            result.add_issue(ValidationIssue {
                kind: IssueKind::MissingRwkFile,
                severity: IssueSeverity::Error,
                message: "No .rwk files found in workspace".to_string(),
                location: None,
                impacted_services: Vec::new(),
            });
        }
    }

    /// Détecte services/RPCs exposés mais jamais consommés
    fn check_orphaned_elements(&self, result: &mut ValidationResult) {
        // Construire la liste des services consommés
        let consumed_services: HashSet<String> = self
            .dependencies
            .iter()
            .map(|dep| dep.to_service.clone())
            .collect();

        // Trouver les services exposés mais non consommés
        for def in &self.definitions {
            if !consumed_services.contains(&def.service_name) {
                // Service orphelin (pas forcément une erreur si c'est un point d'entrée)
                result.add_issue(ValidationIssue {
                    kind: IssueKind::OrphanedService,
                    severity: IssueSeverity::Info,
                    message: format!(
                        "Service '{}' is exposed but not consumed by any other service",
                        def.service_name
                    ),
                    location: Some(def.source_file.clone()),
                    impacted_services: vec![def.service_name.clone()],
                });
            }
        }

        // Détecter RPCs orphelins (exposés mais jamais appelés)
        // Pour l'instant, on se base sur les messages utilisés dans les dépendances
        let consumed_messages: HashSet<String> = self
            .dependencies
            .iter()
            .flat_map(|dep| dep.used_messages.iter().cloned())
            .collect();

        for def in &self.definitions {
            for rpc in &def.rpcs {
                let input_consumed = consumed_messages.contains(&rpc.input_type);
                let output_consumed = consumed_messages.contains(&rpc.output_type);

                if !input_consumed && !output_consumed {
                    result.add_issue(ValidationIssue {
                        kind: IssueKind::OrphanedRpc,
                        severity: IssueSeverity::Info,
                        message: format!(
                            "RPC '{}.{}' is exposed but its messages are not consumed",
                            def.service_name, rpc.name
                        ),
                        location: Some(def.source_file.clone()),
                        impacted_services: vec![def.service_name.clone()],
                    });
                }
            }
        }
    }

    /// Détecte dépendances vers services non définis
    fn check_undefined_dependencies(&self, result: &mut ValidationResult) {
        // Construire la liste des services définis
        let defined_services: HashSet<String> = self
            .definitions
            .iter()
            .map(|def| def.service_name.clone())
            .collect();

        // Vérifier chaque dépendance
        for dep in &self.dependencies {
            if !defined_services.contains(&dep.to_service) {
                result.add_issue(ValidationIssue {
                    kind: IssueKind::UndefinedDependency,
                    severity: IssueSeverity::Error,
                    message: format!(
                        "Service '{}' depends on undefined service '{}'",
                        dep.from_service, dep.to_service
                    ),
                    location: None,
                    impacted_services: vec![dep.from_service.clone()],
                });
            }
        }
    }

    /// Détecte dépendances circulaires
    fn check_circular_dependencies(&self, result: &mut ValidationResult) {
        // Construire le graphe de dépendances
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();

        for dep in &self.dependencies {
            graph
                .entry(dep.from_service.clone())
                .or_default()
                .push(dep.to_service.clone());
        }

        // Détection de cycles via DFS
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();

        for service in graph.keys() {
            if let Some(cycle) = self.detect_cycle(&graph, service, &mut visited, &mut rec_stack) {
                result.add_issue(ValidationIssue {
                    kind: IssueKind::CircularDependency,
                    severity: IssueSeverity::Error,
                    message: format!("Circular dependency detected: {}", cycle.join(" -> ")),
                    location: None,
                    impacted_services: cycle,
                });
            }
        }
    }

    /// Détection de cycle via DFS
    fn detect_cycle(
        &self,
        graph: &HashMap<String, Vec<String>>,
        node: &str,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
    ) -> Option<Vec<String>> {
        if rec_stack.contains(node) {
            // Cycle détecté
            return Some(vec![node.to_string()]);
        }

        if visited.contains(node) {
            return None;
        }

        visited.insert(node.to_string());
        rec_stack.insert(node.to_string());

        if let Some(neighbors) = graph.get(node) {
            for neighbor in neighbors {
                if let Some(mut cycle) = self.detect_cycle(graph, neighbor, visited, rec_stack) {
                    cycle.insert(0, node.to_string());
                    return Some(cycle);
                }
            }
        }

        rec_stack.remove(node);
        None
    }
}
