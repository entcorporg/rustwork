/// Analyseur de graphe d'appels gRPC
use super::types::{CallGraph, GraphAnalysis, ServiceEdge, ServiceNode, ServiceRole};
use crate::mcp::rustwork_grpc_list_definitions::types::{GrpcDefinition, ServiceDependency};
use std::collections::{HashMap, HashSet};

pub struct CallGraphAnalyzer {
    definitions: Vec<GrpcDefinition>,
    dependencies: Vec<ServiceDependency>,
}

impl CallGraphAnalyzer {
    pub fn new(definitions: Vec<GrpcDefinition>, dependencies: Vec<ServiceDependency>) -> Self {
        Self {
            definitions,
            dependencies,
        }
    }

    /// Construit le graphe d'appels complet
    pub fn build_graph(&self) -> CallGraph {
        // Construire les nœuds
        let nodes = self.build_nodes();

        // Construire les arêtes
        let edges = self.build_edges();

        // Analyser le graphe
        let analysis = self.analyze_graph(&nodes, &edges);

        CallGraph {
            nodes,
            edges,
            analysis,
        }
    }

    /// Construit les nœuds du graphe
    fn build_nodes(&self) -> Vec<ServiceNode> {
        // Compter les dépendances entrantes pour chaque service
        let mut incoming_deps: HashMap<String, usize> = HashMap::new();

        for dep in &self.dependencies {
            *incoming_deps.entry(dep.to_service.clone()).or_insert(0) += 1;
        }

        // Compter les dépendances sortantes
        let mut outgoing_deps: HashMap<String, usize> = HashMap::new();

        for dep in &self.dependencies {
            *outgoing_deps.entry(dep.from_service.clone()).or_insert(0) += 1;
        }

        // Créer les nœuds
        self.definitions
            .iter()
            .map(|def| {
                let incoming = incoming_deps.get(&def.service_name).unwrap_or(&0);
                let outgoing = outgoing_deps.get(&def.service_name).unwrap_or(&0);

                let role = self.determine_role(&def.service_name, *incoming, *outgoing);

                ServiceNode {
                    service_name: def.service_name.clone(),
                    source_file: def.source_file.clone(),
                    role,
                    rpc_count: def.rpcs.len(),
                }
            })
            .collect()
    }

    /// Détermine le rôle d'un service
    fn determine_role(&self, _service_name: &str, incoming: usize, outgoing: usize) -> ServiceRole {
        if incoming == 0 && outgoing == 0 {
            ServiceRole::Isolated
        } else if incoming == 0 {
            ServiceRole::Entrypoint
        } else if incoming >= 3 {
            // Seuil arbitraire pour "central"
            ServiceRole::Central
        } else {
            ServiceRole::Intermediate
        }
    }

    /// Construit les arêtes du graphe
    fn build_edges(&self) -> Vec<ServiceEdge> {
        self.dependencies
            .iter()
            .map(|dep| {
                // Trouver les RPCs utilisés
                let rpcs_used = self.find_rpcs_using_messages(&dep.to_service, &dep.used_messages);

                ServiceEdge {
                    from_service: dep.from_service.clone(),
                    to_service: dep.to_service.clone(),
                    rpcs_used,
                    messages_used: dep.used_messages.clone(),
                }
            })
            .collect()
    }

    /// Trouve les RPCs qui utilisent certains messages
    fn find_rpcs_using_messages(&self, service_name: &str, messages: &[String]) -> Vec<String> {
        let mut rpcs = Vec::new();

        if let Some(def) = self
            .definitions
            .iter()
            .find(|d| d.service_name == service_name)
        {
            for rpc in &def.rpcs {
                if messages.contains(&rpc.input_type) || messages.contains(&rpc.output_type) {
                    rpcs.push(rpc.name.clone());
                }
            }
        }

        rpcs
    }

    /// Analyse le graphe
    fn analyze_graph(&self, nodes: &[ServiceNode], edges: &[ServiceEdge]) -> GraphAnalysis {
        let total_services = nodes.len();
        let total_dependencies = edges.len();

        // Identifier les différents types de services
        let mut entrypoints = Vec::new();
        let mut central_services = Vec::new();
        let mut isolated_services = Vec::new();

        for node in nodes {
            match node.role {
                ServiceRole::Entrypoint => entrypoints.push(node.service_name.clone()),
                ServiceRole::Central => central_services.push(node.service_name.clone()),
                ServiceRole::Isolated => isolated_services.push(node.service_name.clone()),
                ServiceRole::Intermediate => {}
            }
        }

        // Calculer la profondeur maximale des dépendances
        let max_dependency_depth = self.calculate_max_depth(edges);

        GraphAnalysis {
            total_services,
            total_dependencies,
            entrypoints,
            central_services,
            isolated_services,
            max_dependency_depth,
        }
    }

    /// Calcule la profondeur maximale du graphe de dépendances
    fn calculate_max_depth(&self, edges: &[ServiceEdge]) -> usize {
        // Construire le graphe
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();

        for edge in edges {
            graph
                .entry(edge.from_service.clone())
                .or_default()
                .push(edge.to_service.clone());
        }

        // Trouver la profondeur max via DFS
        let mut max_depth = 0;
        let mut visited = HashSet::new();

        for node in graph.keys() {
            let depth = self.dfs_depth(&graph, node, &mut visited);
            max_depth = max_depth.max(depth);
        }

        max_depth
    }

    /// DFS pour calculer la profondeur
    fn dfs_depth(
        &self,
        graph: &HashMap<String, Vec<String>>,
        node: &str,
        visited: &mut HashSet<String>,
    ) -> usize {
        if visited.contains(node) {
            return 0; // Éviter les cycles
        }

        visited.insert(node.to_string());

        let max_child_depth = graph
            .get(node)
            .map(|neighbors| {
                neighbors
                    .iter()
                    .map(|neighbor| self.dfs_depth(graph, neighbor, visited))
                    .max()
                    .unwrap_or(0)
            })
            .unwrap_or(0);

        visited.remove(node);

        1 + max_child_depth
    }
}
