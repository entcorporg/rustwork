/// Générateur de fichiers .proto à partir de l'AST
use super::ast::{Contract, Field, Message, Rpc, Service};
use std::fmt::Write;

/// Génère un fichier .proto complet à partir d'un contrat
pub fn generate_proto(contract: &Contract) -> Result<String, std::fmt::Error> {
    let mut output = String::new();

    // En-tête
    writeln!(output, "syntax = \"proto3\";")?;
    writeln!(output)?;

    // Package - CRITIQUE: doit correspondre au nom utilisé dans include_proto!
    let package_name = format!("{}_service", to_snake_case(&contract.service.name));
    writeln!(output, "package {};", package_name)?;
    writeln!(output)?;

    // Service
    generate_service(&mut output, &contract.service)?;
    writeln!(output)?;

    // Messages
    for message in &contract.messages {
        generate_message(&mut output, message)?;
        writeln!(output)?;
    }

    Ok(output)
}

/// Génère la définition du service
fn generate_service(output: &mut String, service: &Service) -> Result<(), std::fmt::Error> {
    writeln!(output, "service {} {{", service.name)?;

    for rpc in &service.rpcs {
        generate_rpc(output, rpc)?;
    }

    writeln!(output, "}}")?;
    Ok(())
}

/// Génère un RPC
fn generate_rpc(output: &mut String, rpc: &Rpc) -> Result<(), std::fmt::Error> {
    writeln!(
        output,
        "  rpc {} ({}) returns ({});",
        rpc.name, rpc.input_type, rpc.output_type
    )?;
    Ok(())
}

/// Génère un message
fn generate_message(output: &mut String, message: &Message) -> Result<(), std::fmt::Error> {
    writeln!(output, "message {} {{", message.name)?;

    for (idx, field) in message.fields.iter().enumerate() {
        generate_field(output, field, idx + 1)?;
    }

    writeln!(output, "}}")?;
    Ok(())
}

/// Génère un champ
fn generate_field(
    output: &mut String,
    field: &Field,
    field_number: usize,
) -> Result<(), std::fmt::Error> {
    let proto_type = field.field_type.to_proto_type();

    // Pour les types optional, proto3 gère automatiquement avec 'optional'
    let optional_prefix = if matches!(field.field_type, super::ast::FieldType::Optional(_)) {
        "optional "
    } else {
        ""
    };

    writeln!(
        output,
        "  {}{} {} = {};",
        optional_prefix,
        proto_type,
        to_snake_case(&field.name),
        field_number
    )?;
    Ok(())
}

/// Convertit un nom PascalCase ou camelCase en snake_case
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grpc::ast::FieldType;

    #[test]
    fn test_to_snake_case() {
        assert_eq!(to_snake_case("UserService"), "user_service");
        assert_eq!(to_snake_case("GetUser"), "get_user");
        assert_eq!(to_snake_case("HTTPRequest"), "h_t_t_p_request");
    }

    #[test]
    fn test_generate_simple_proto() {
        let contract = Contract {
            service: Service {
                name: "UserService".to_string(),
                rpcs: vec![Rpc {
                    name: "GetUser".to_string(),
                    input_type: "GetUserRequest".to_string(),
                    output_type: "User".to_string(),
                }],
            },
            messages: vec![
                Message {
                    name: "GetUserRequest".to_string(),
                    fields: vec![Field {
                        name: "id".to_string(),
                        field_type: FieldType::Uuid,
                    }],
                },
                Message {
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
                },
            ],
        };

        let proto = generate_proto(&contract).unwrap();

        assert!(proto.contains("syntax = \"proto3\";"));
        assert!(proto.contains("package rustwork.user_service;"));
        assert!(proto.contains("service UserService"));
        assert!(proto.contains("rpc GetUser"));
        assert!(proto.contains("message User"));
    }

    #[test]
    fn test_generate_with_optional_and_list() {
        let message = Message {
            name: "TestMessage".to_string(),
            fields: vec![
                Field {
                    name: "tags".to_string(),
                    field_type: FieldType::List(Box::new(FieldType::String)),
                },
                Field {
                    name: "nickname".to_string(),
                    field_type: FieldType::Optional(Box::new(FieldType::String)),
                },
            ],
        };

        let mut output = String::new();
        generate_message(&mut output, &message).unwrap();

        assert!(output.contains("repeated string tags"));
        assert!(output.contains("optional string nickname"));
    }
}
