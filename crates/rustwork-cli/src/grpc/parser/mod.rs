/// Parser pour le DSL Rustwork (.rwk)
use crate::grpc::ast::Contract;
use crate::grpc::errors::ParseResult;

mod message;
mod service;
mod types;
mod utils;

pub use types::Parser;

impl Parser {
    /// Parse le fichier complet
    pub fn parse(&mut self) -> ParseResult<Contract> {
        let service = self.parse_service()?;
        let messages = self.parse_messages()?;

        Ok(Contract { service, messages })
    }
}

/// Parse un fichier .rwk
pub fn parse_contract(source: impl Into<String>) -> ParseResult<Contract> {
    let mut parser = Parser::new(source);
    parser.parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_contract() {
        let source = r#"
service UserService

rpc GetUser (GetUserRequest) returns (User)

message GetUserRequest {
  id: uuid
}

message User {
  id: uuid
  email: string
}
"#;

        let contract = parse_contract(source).unwrap();
        assert_eq!(contract.service.name, "UserService");
        assert_eq!(contract.service.rpcs.len(), 1);
        assert_eq!(contract.messages.len(), 2);
    }

    #[test]
    fn test_parse_rpc() {
        let source = r#"
service TestService
rpc CreateUser (CreateUserRequest) returns (User)
message CreateUserRequest { email: string }
message User { id: uuid }
"#;

        let contract = parse_contract(source).unwrap();
        let rpc = &contract.service.rpcs[0];
        assert_eq!(rpc.name, "CreateUser");
        assert_eq!(rpc.input_type, "CreateUserRequest");
        assert_eq!(rpc.output_type, "User");
    }

    #[test]
    fn test_parse_field_types() {
        let source = r#"
service TestService
rpc Test (Req) returns (Res)
message Req { id: uuid }
message Res {
  name: string
  age: int
  active: bool
  tags: list<string>
  nickname: optional<string>
}
"#;

        let contract = parse_contract(source).unwrap();
        let message = &contract.messages[1];
        assert_eq!(message.fields.len(), 5);
        assert_eq!(
            message.fields[3].field_type,
            crate::grpc::ast::FieldType::List(Box::new(crate::grpc::ast::FieldType::String))
        );
    }

    #[test]
    fn test_parse_error() {
        let source = "service\nrpc";
        let result = parse_contract(source);
        assert!(result.is_err());
    }
}
