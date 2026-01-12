/// Analyseur de dépendances gRPC inter-services
use super::types::{GrpcDefinition, ServiceDependency};
use std::collections::{HashMap, HashSet};

/// Analyseur de dépendances gRPC
pub struct DependencyAnalyzer;

impl DependencyAnalyzer {
    /// Analyse les dépendances entre services gRPC
    ///
    /// Une dépendance existe si un RPC utilise un message défini dans un autre service
    pub fn analyze(definitions: &[GrpcDefinition]) -> Vec<ServiceDependency> {
        let mut dependencies = Vec::new();

        // Créer un index message_name -> service_name
        let mut message_to_service: HashMap<String, String> = HashMap::new();
        for def in definitions {
            for msg in &def.messages {
                message_to_service.insert(msg.name.clone(), def.service_name.clone());
            }
        }

        // Analyser chaque service
        for def in definitions {
            let mut used_messages_by_service: HashMap<String, HashSet<String>> = HashMap::new();

            // Pour chaque RPC, identifier les messages utilisés
            for rpc in &def.rpcs {
                // Input type
                if let Some(defining_service) = message_to_service.get(&rpc.input_type) {
                    if defining_service != &def.service_name {
                        used_messages_by_service
                            .entry(defining_service.clone())
                            .or_default()
                            .insert(rpc.input_type.clone());
                    }
                }

                // Output type
                if let Some(defining_service) = message_to_service.get(&rpc.output_type) {
                    if defining_service != &def.service_name {
                        used_messages_by_service
                            .entry(defining_service.clone())
                            .or_default()
                            .insert(rpc.output_type.clone());
                    }
                }
            }

            // Créer les dépendances
            for (to_service, messages) in used_messages_by_service {
                dependencies.push(ServiceDependency {
                    from_service: def.service_name.clone(),
                    to_service,
                    used_messages: messages.into_iter().collect(),
                });
            }
        }

        dependencies
    }
}
