/// Export des contrats gRPC pour MCP (Model Context Protocol)
use std::path::Path;

mod convert;
mod scan;
mod types;

/// Exporte tous les contrats gRPC d'un projet
#[allow(dead_code)]
pub fn export_grpc_contracts(
    project_root: &Path,
) -> Result<Vec<types::GrpcContractExport>, String> {
    let mut exports = Vec::new();

    // Trouver tous les fichiers .rwk
    let rwk_files = scan::find_rwk_files(project_root)?;

    for rwk_path in rwk_files {
        // Lire et parser le fichier
        let source = std::fs::read_to_string(&rwk_path)
            .map_err(|e| format!("Erreur lecture {}: {}", rwk_path.display(), e))?;

        let contract = crate::grpc::parse_contract(&source)
            .map_err(|e| format!("Erreur parsing {}: {}", rwk_path.display(), e))?;

        // Convertir en export
        let export = convert::contract_to_export(&contract, &rwk_path);
        exports.push(export);
    }

    Ok(exports)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grpc::ast::{Contract, Field, FieldType, Message, Rpc, Service};

    #[test]
    fn test_contract_to_export() {
        let contract = Contract {
            service: Service {
                name: "UserService".to_string(),
                rpcs: vec![Rpc {
                    name: "GetUser".to_string(),
                    input_type: "GetUserRequest".to_string(),
                    output_type: "User".to_string(),
                }],
            },
            messages: vec![Message {
                name: "User".to_string(),
                fields: vec![
                    Field {
                        name: "id".to_string(),
                        field_type: FieldType::Uuid,
                    },
                    Field {
                        name: "email".to_string(),
                        field_type: FieldType::String,
                    },
                ],
            }],
        };

        let export = convert::contract_to_export(&contract, Path::new("test.rwk"));

        assert_eq!(export.service_name, "UserService");
        assert_eq!(export.package, "rustwork.user_service");
        assert_eq!(export.rpcs.len(), 1);
        assert_eq!(export.messages.len(), 1);
    }
}
