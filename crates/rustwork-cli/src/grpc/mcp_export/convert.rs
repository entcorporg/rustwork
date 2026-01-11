use super::types::{FieldExport, GrpcContractExport, MessageExport, RpcExport};
use crate::grpc::ast::Contract;
use std::path::Path;

/// Convertit un contrat en export
#[allow(dead_code)]
pub(crate) fn contract_to_export(contract: &Contract, file_path: &Path) -> GrpcContractExport {
    let package = format!("rustwork.{}", to_snake_case(&contract.service.name));

    let rpcs = contract
        .service
        .rpcs
        .iter()
        .map(|rpc| RpcExport {
            name: rpc.name.clone(),
            input_type: rpc.input_type.clone(),
            output_type: rpc.output_type.clone(),
            description: None,
        })
        .collect();

    let messages = contract
        .messages
        .iter()
        .map(|msg| MessageExport {
            name: msg.name.clone(),
            fields: msg
                .fields
                .iter()
                .map(|field| FieldExport {
                    name: field.name.clone(),
                    field_type: field.field_type.to_string(),
                })
                .collect(),
        })
        .collect();

    GrpcContractExport {
        service_name: contract.service.name.clone(),
        package,
        rpcs,
        messages,
        file_path: file_path.display().to_string(),
    }
}

/// Convertit un nom PascalCase en snake_case
#[allow(dead_code)]
fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut prev_is_lower = false;

    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 && prev_is_lower {
                result.push('_');
            }
            result.push(c.to_ascii_lowercase());
            prev_is_lower = false;
        } else {
            result.push(c);
            prev_is_lower = c.is_lowercase();
        }
    }

    result
}
