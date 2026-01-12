/// Générateur de suggestions de migration intelligentes
use crate::mcp::rustwork_grpc_diff_versions::types::{Change, ChangeType, Severity};

/// Générateur de suggestions
pub struct MigrationSuggester;

impl MigrationSuggester {
    /// Génère des suggestions de migration basées sur les changements détectés
    pub fn generate_suggestions(changes: &[Change]) -> Vec<String> {
        let mut suggestions = Vec::new();

        // Grouper les changements par type
        let breaking: Vec<_> = changes
            .iter()
            .filter(|c| c.severity == Severity::Breaking)
            .collect();

        if breaking.is_empty() {
            suggestions.push("No breaking changes detected. Migration should be safe.".to_string());
            return suggestions;
        }

        suggestions.push(format!(
            "⚠️  {} breaking change(s) detected. Manual migration required.",
            breaking.len()
        ));
        suggestions.push(String::new()); // ligne vide

        // Suggestions par catégorie
        Self::suggest_service_changes(&breaking, &mut suggestions);
        Self::suggest_rpc_changes(&breaking, &mut suggestions);
        Self::suggest_message_changes(&breaking, &mut suggestions);
        Self::suggest_field_changes(&breaking, &mut suggestions);

        // Recommandations générales
        suggestions.push(String::new());
        suggestions.push("General recommendations:".to_string());
        suggestions.push("- Update all gRPC clients consuming this service".to_string());
        suggestions
            .push("- Run `rustwork grpc validate` to check workspace consistency".to_string());
        suggestions
            .push("- Consider versioning your API (e.g., v1, v2) for major changes".to_string());

        suggestions
    }

    fn suggest_service_changes(breaking: &[&Change], suggestions: &mut Vec<String>) {
        let service_changes: Vec<_> = breaking
            .iter()
            .filter(|c| {
                matches!(
                    c.change_type,
                    ChangeType::ServiceRemoved | ChangeType::ServiceRenamed
                )
            })
            .collect();

        if service_changes.is_empty() {
            return;
        }

        suggestions.push("Service-level changes:".to_string());
        for change in service_changes {
            match change.change_type {
                ChangeType::ServiceRemoved => {
                    if let Some(service) = &change.service {
                        suggestions.push(format!(
                            "  - Service '{}' was removed. Remove all client references.",
                            service
                        ));
                    }
                }
                ChangeType::ServiceRenamed => {
                    if let Some(old_name) = &change.service {
                        if let Some(new_name) = &change.new_type {
                            suggestions.push(format!(
                                "  - Service renamed '{}' → '{}'. Update all import paths.",
                                old_name, new_name
                            ));
                        }
                    }
                }
                _ => {}
            }
        }
        suggestions.push(String::new());
    }

    fn suggest_rpc_changes(breaking: &[&Change], suggestions: &mut Vec<String>) {
        let rpc_changes: Vec<_> = breaking
            .iter()
            .filter(|c| {
                matches!(
                    c.change_type,
                    ChangeType::RpcRemoved | ChangeType::RpcSignatureChanged
                )
            })
            .collect();

        if rpc_changes.is_empty() {
            return;
        }

        suggestions.push("RPC-level changes:".to_string());
        for change in rpc_changes {
            match change.change_type {
                ChangeType::RpcRemoved => {
                    if let (Some(service), Some(rpc)) = (&change.service, &change.rpc) {
                        suggestions.push(format!(
                            "  - RPC '{}.{}' was removed. Remove all calls to this endpoint.",
                            service, rpc
                        ));
                    }
                }
                ChangeType::RpcSignatureChanged => {
                    if let (Some(service), Some(rpc), Some(old_sig), Some(new_sig)) = (
                        &change.service,
                        &change.rpc,
                        &change.old_type,
                        &change.new_type,
                    ) {
                        suggestions
                            .push(format!("  - RPC '{}.{}' signature changed:", service, rpc));
                        suggestions.push(format!("      Old: {}", old_sig));
                        suggestions.push(format!("      New: {}", new_sig));
                        suggestions
                            .push("    Update all call sites to match new signature.".to_string());
                    }
                }
                _ => {}
            }
        }
        suggestions.push(String::new());
    }

    fn suggest_message_changes(breaking: &[&Change], suggestions: &mut Vec<String>) {
        let message_changes: Vec<_> = breaking
            .iter()
            .filter(|c| {
                matches!(
                    c.change_type,
                    ChangeType::MessageRemoved | ChangeType::MessageRenamed
                )
            })
            .collect();

        if message_changes.is_empty() {
            return;
        }

        suggestions.push("Message-level changes:".to_string());
        for change in message_changes {
            match change.change_type {
                ChangeType::MessageRemoved => {
                    if let Some(message) = &change.message {
                        suggestions.push(format!(
                            "  - Message '{}' was removed. Remove all usages.",
                            message
                        ));
                    }
                }
                ChangeType::MessageRenamed => {
                    if let (Some(old_name), Some(new_name)) = (&change.message, &change.new_type) {
                        suggestions.push(format!(
                            "  - Message renamed '{}' → '{}'. Update all type references.",
                            old_name, new_name
                        ));
                    }
                }
                _ => {}
            }
        }
        suggestions.push(String::new());
    }

    fn suggest_field_changes(breaking: &[&Change], suggestions: &mut Vec<String>) {
        let field_changes: Vec<_> = breaking
            .iter()
            .filter(|c| {
                matches!(
                    c.change_type,
                    ChangeType::FieldRemoved
                        | ChangeType::FieldTypeChanged
                        | ChangeType::FieldOptionalityChanged
                )
            })
            .collect();

        if field_changes.is_empty() {
            return;
        }

        suggestions.push("Field-level changes:".to_string());
        for change in field_changes {
            match change.change_type {
                ChangeType::FieldRemoved => {
                    if let (Some(message), Some(field)) = (&change.message, &change.field) {
                        suggestions.push(format!(
                            "  - Field '{}.{}' was removed. Remove all references.",
                            message, field
                        ));
                    }
                }
                ChangeType::FieldTypeChanged => {
                    if let (Some(message), Some(field), Some(old_type), Some(new_type)) = (
                        &change.message,
                        &change.field,
                        &change.old_type,
                        &change.new_type,
                    ) {
                        suggestions.push(format!(
                            "  - Field '{}.{}' type changed: {} → {}",
                            message, field, old_type, new_type
                        ));
                        suggestions.push(
                            "    Add type conversion logic where this field is used.".to_string(),
                        );
                    }
                }
                ChangeType::FieldOptionalityChanged => {
                    if let (Some(message), Some(field), Some(old_opt), Some(new_opt)) = (
                        &change.message,
                        &change.field,
                        &change.old_type,
                        &change.new_type,
                    ) {
                        suggestions.push(format!(
                            "  - Field '{}.{}' optionality changed: {} → {}",
                            message, field, old_opt, new_opt
                        ));
                        if new_opt == "required" {
                            suggestions.push("    ⚠️  Field is now required. Ensure all clients provide this value.".to_string());
                        } else {
                            suggestions.push(
                                "    Field is now optional. Handle None cases in client code."
                                    .to_string(),
                            );
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
