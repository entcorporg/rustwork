/// Comparateur structurel pour détecter les changements entre deux versions d'un contrat gRPC
use crate::grpc::ast::{Contract, Field, FieldType, Message, Rpc, Service};
use crate::mcp::rustwork_grpc_diff_versions::types::{Change, ChangeType, Severity};
use std::collections::HashMap;

/// Comparateur de contrats gRPC
pub struct ContractComparator;

impl ContractComparator {
    /// Compare deux contrats et retourne tous les changements détectés
    pub fn compare(old: &Contract, new: &Contract) -> Vec<Change> {
        let mut changes = Vec::new();

        // Comparer les services
        changes.extend(Self::compare_services(&old.service, &new.service));

        // Comparer les messages
        changes.extend(Self::compare_messages(&old.messages, &new.messages));

        changes
    }

    /// Compare deux services
    fn compare_services(old: &Service, new: &Service) -> Vec<Change> {
        let mut changes = Vec::new();

        // Vérifier le renommage du service
        if old.name != new.name {
            changes.push(Change {
                change_type: ChangeType::ServiceRenamed,
                service: Some(old.name.clone()),
                rpc: None,
                message: None,
                field: None,
                old_type: None,
                new_type: Some(new.name.clone()),
                description: format!("Service renamed from '{}' to '{}'", old.name, new.name),
                severity: Severity::Breaking,
            });
        }

        // Comparer les RPCs
        changes.extend(Self::compare_rpcs(&old.rpcs, &new.rpcs, &new.name));

        changes
    }

    /// Compare deux ensembles de RPCs
    fn compare_rpcs(old: &[Rpc], new: &[Rpc], service_name: &str) -> Vec<Change> {
        let mut changes = Vec::new();

        let old_map: HashMap<&str, &Rpc> = old.iter().map(|r| (r.name.as_str(), r)).collect();
        let new_map: HashMap<&str, &Rpc> = new.iter().map(|r| (r.name.as_str(), r)).collect();

        // Détecter les RPCs supprimés (BREAKING)
        for rpc_name in old_map.keys() {
            if !new_map.contains_key(rpc_name) {
                changes.push(Change {
                    change_type: ChangeType::RpcRemoved,
                    service: Some(service_name.to_string()),
                    rpc: Some(rpc_name.to_string()),
                    message: None,
                    field: None,
                    old_type: None,
                    new_type: None,
                    description: format!(
                        "RPC '{}' removed from service '{}'",
                        rpc_name, service_name
                    ),
                    severity: Severity::Breaking,
                });
            }
        }

        // Détecter les RPCs ajoutés (COMPATIBLE)
        for rpc_name in new_map.keys() {
            if !old_map.contains_key(rpc_name) {
                changes.push(Change {
                    change_type: ChangeType::RpcAdded,
                    service: Some(service_name.to_string()),
                    rpc: Some(rpc_name.to_string()),
                    message: None,
                    field: None,
                    old_type: None,
                    new_type: None,
                    description: format!("RPC '{}' added to service '{}'", rpc_name, service_name),
                    severity: Severity::Compatible,
                });
            }
        }

        // Détecter les changements de signature (BREAKING)
        for (rpc_name, old_rpc) in &old_map {
            if let Some(new_rpc) = new_map.get(rpc_name) {
                if old_rpc.input_type != new_rpc.input_type
                    || old_rpc.output_type != new_rpc.output_type
                {
                    changes.push(Change {
                        change_type: ChangeType::RpcSignatureChanged,
                        service: Some(service_name.to_string()),
                        rpc: Some(rpc_name.to_string()),
                        message: None,
                        field: None,
                        old_type: Some(format!(
                            "({}) -> ({})",
                            old_rpc.input_type, old_rpc.output_type
                        )),
                        new_type: Some(format!(
                            "({}) -> ({})",
                            new_rpc.input_type, new_rpc.output_type
                        )),
                        description: format!(
                            "RPC '{}' signature changed in service '{}'",
                            rpc_name, service_name
                        ),
                        severity: Severity::Breaking,
                    });
                }
            }
        }

        changes
    }

    /// Compare deux ensembles de messages
    fn compare_messages(old: &[Message], new: &[Message]) -> Vec<Change> {
        let mut changes = Vec::new();

        let old_map: HashMap<&str, &Message> = old.iter().map(|m| (m.name.as_str(), m)).collect();
        let new_map: HashMap<&str, &Message> = new.iter().map(|m| (m.name.as_str(), m)).collect();

        // Détecter les messages supprimés (BREAKING)
        for message_name in old_map.keys() {
            if !new_map.contains_key(message_name) {
                changes.push(Change {
                    change_type: ChangeType::MessageRemoved,
                    service: None,
                    rpc: None,
                    message: Some(message_name.to_string()),
                    field: None,
                    old_type: None,
                    new_type: None,
                    description: format!("Message '{}' removed", message_name),
                    severity: Severity::Breaking,
                });
            }
        }

        // Détecter les messages ajoutés (COMPATIBLE)
        for message_name in new_map.keys() {
            if !old_map.contains_key(message_name) {
                changes.push(Change {
                    change_type: ChangeType::MessageAdded,
                    service: None,
                    rpc: None,
                    message: Some(message_name.to_string()),
                    field: None,
                    old_type: None,
                    new_type: None,
                    description: format!("Message '{}' added", message_name),
                    severity: Severity::Compatible,
                });
            }
        }

        // Détecter les changements dans les messages existants
        for (message_name, old_msg) in &old_map {
            if let Some(new_msg) = new_map.get(message_name) {
                changes.extend(Self::compare_fields(
                    &old_msg.fields,
                    &new_msg.fields,
                    message_name,
                ));
            }
        }

        changes
    }

    /// Compare deux ensembles de fields dans un message
    fn compare_fields(old: &[Field], new: &[Field], message_name: &str) -> Vec<Change> {
        let mut changes = Vec::new();

        let old_map: HashMap<&str, &Field> = old.iter().map(|f| (f.name.as_str(), f)).collect();
        let new_map: HashMap<&str, &Field> = new.iter().map(|f| (f.name.as_str(), f)).collect();

        // Détecter les fields supprimés (BREAKING)
        for field_name in old_map.keys() {
            if !new_map.contains_key(field_name) {
                changes.push(Change {
                    change_type: ChangeType::FieldRemoved,
                    service: None,
                    rpc: None,
                    message: Some(message_name.to_string()),
                    field: Some(field_name.to_string()),
                    old_type: None,
                    new_type: None,
                    description: format!(
                        "Field '{}' removed from message '{}'",
                        field_name, message_name
                    ),
                    severity: Severity::Breaking,
                });
            }
        }

        // Détecter les fields ajoutés (COMPATIBLE)
        for field_name in new_map.keys() {
            if !old_map.contains_key(field_name) {
                changes.push(Change {
                    change_type: ChangeType::FieldAdded,
                    service: None,
                    rpc: None,
                    message: Some(message_name.to_string()),
                    field: Some(field_name.to_string()),
                    old_type: None,
                    new_type: None,
                    description: format!(
                        "Field '{}' added to message '{}'",
                        field_name, message_name
                    ),
                    severity: Severity::Compatible,
                });
            }
        }

        // Détecter les changements de type (BREAKING)
        for (field_name, old_field) in &old_map {
            if let Some(new_field) = new_map.get(field_name) {
                // Vérifier changement de type
                if !Self::types_are_equal(&old_field.field_type, &new_field.field_type) {
                    changes.push(Change {
                        change_type: ChangeType::FieldTypeChanged,
                        service: None,
                        rpc: None,
                        message: Some(message_name.to_string()),
                        field: Some(field_name.to_string()),
                        old_type: Some(Self::field_type_to_string(&old_field.field_type)),
                        new_type: Some(Self::field_type_to_string(&new_field.field_type)),
                        description: format!(
                            "Field '{}' type changed in message '{}'",
                            field_name, message_name
                        ),
                        severity: Severity::Breaking,
                    });
                }

                // Vérifier changement optionalité (BREAKING dans les deux sens)
                let old_optional = matches!(old_field.field_type, FieldType::Optional(_));
                let new_optional = matches!(new_field.field_type, FieldType::Optional(_));

                if old_optional != new_optional {
                    changes.push(Change {
                        change_type: ChangeType::FieldOptionalityChanged,
                        service: None,
                        rpc: None,
                        message: Some(message_name.to_string()),
                        field: Some(field_name.to_string()),
                        old_type: Some(if old_optional {
                            "optional".to_string()
                        } else {
                            "required".to_string()
                        }),
                        new_type: Some(if new_optional {
                            "optional".to_string()
                        } else {
                            "required".to_string()
                        }),
                        description: format!(
                            "Field '{}' optionality changed in message '{}'",
                            field_name, message_name
                        ),
                        severity: Severity::Breaking,
                    });
                }
            }
        }

        changes
    }

    /// Vérifie si deux types sont égaux (en ignorant l'optionalité pour comparaison séparée)
    fn types_are_equal(a: &FieldType, b: &FieldType) -> bool {
        match (a, b) {
            (FieldType::String, FieldType::String) => true,
            (FieldType::Int, FieldType::Int) => true,
            (FieldType::Bool, FieldType::Bool) => true,
            (FieldType::Uuid, FieldType::Uuid) => true,
            (FieldType::DateTime, FieldType::DateTime) => true,
            (FieldType::Optional(inner_a), FieldType::Optional(inner_b)) => {
                Self::types_are_equal(inner_a, inner_b)
            }
            (FieldType::Optional(inner), other) | (other, FieldType::Optional(inner)) => {
                Self::types_are_equal(inner, other)
            }
            (FieldType::List(inner_a), FieldType::List(inner_b)) => {
                Self::types_are_equal(inner_a, inner_b)
            }
            _ => false,
        }
    }

    /// Convertit un FieldType en string lisible
    fn field_type_to_string(field_type: &FieldType) -> String {
        match field_type {
            FieldType::String => "string".to_string(),
            FieldType::Int => "int".to_string(),
            FieldType::Bool => "bool".to_string(),
            FieldType::Uuid => "uuid".to_string(),
            FieldType::DateTime => "datetime".to_string(),
            FieldType::Optional(inner) => {
                format!("optional<{}>", Self::field_type_to_string(inner))
            }
            FieldType::List(inner) => format!("list<{}>", Self::field_type_to_string(inner)),
        }
    }
}
