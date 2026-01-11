/// AST (Abstract Syntax Tree) pour le DSL Rustwork (.rwk)
use std::fmt;

/// Représente un fichier de contrat gRPC complet
#[derive(Debug, Clone, PartialEq)]
pub struct Contract {
    pub service: Service,
    pub messages: Vec<Message>,
}

/// Représente un service gRPC
#[derive(Debug, Clone, PartialEq)]
pub struct Service {
    pub name: String,
    pub rpcs: Vec<Rpc>,
}

/// Représente un RPC (Remote Procedure Call)
#[derive(Debug, Clone, PartialEq)]
pub struct Rpc {
    pub name: String,
    pub input_type: String,
    pub output_type: String,
}

/// Représente un message (structure de données)
#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    pub name: String,
    pub fields: Vec<Field>,
}

/// Représente un champ dans un message
#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub name: String,
    pub field_type: FieldType,
}

/// Types supportés dans le DSL v0
#[derive(Debug, Clone, PartialEq)]
pub enum FieldType {
    String,
    Int,
    Bool,
    Uuid,
    DateTime,
    Optional(Box<FieldType>),
    List(Box<FieldType>),
}

impl FieldType {
    /// Convertit le type DSL en type proto
    pub fn to_proto_type(&self) -> String {
        match self {
            FieldType::String => "string".to_string(),
            FieldType::Int => "int32".to_string(),
            FieldType::Bool => "bool".to_string(),
            FieldType::Uuid => "string".to_string(), // UUID → string en proto
            FieldType::DateTime => "string".to_string(), // DateTime → string (RFC3339)
            FieldType::Optional(inner) => inner.to_proto_type(), // optional géré par proto3
            FieldType::List(inner) => format!("repeated {}", inner.to_proto_type()),
        }
    }

    /// Convertit le type DSL en type Rust
    #[allow(dead_code)]
    pub fn to_rust_type(&self) -> String {
        match self {
            FieldType::String => "String".to_string(),
            FieldType::Int => "i32".to_string(),
            FieldType::Bool => "bool".to_string(),
            FieldType::Uuid => "String".to_string(), // Peut être uuid::Uuid si on veut
            FieldType::DateTime => "String".to_string(), // Peut être chrono::DateTime si on veut
            FieldType::Optional(inner) => format!("Option<{}>", inner.to_rust_type()),
            FieldType::List(inner) => format!("Vec<{}>", inner.to_rust_type()),
        }
    }

    /// Parse un type à partir du DSL
    pub fn parse(type_str: &str) -> Result<Self, String> {
        let type_str = type_str.trim();

        // Gestion de optional<T>
        if type_str.starts_with("optional<") && type_str.ends_with('>') {
            let inner = &type_str[9..type_str.len() - 1];
            return Ok(FieldType::Optional(Box::new(FieldType::parse(inner)?)));
        }

        // Gestion de list<T>
        if type_str.starts_with("list<") && type_str.ends_with('>') {
            let inner = &type_str[5..type_str.len() - 1];
            return Ok(FieldType::List(Box::new(FieldType::parse(inner)?)));
        }

        // Types simples
        match type_str {
            "string" => Ok(FieldType::String),
            "int" => Ok(FieldType::Int),
            "bool" => Ok(FieldType::Bool),
            "uuid" => Ok(FieldType::Uuid),
            "datetime" => Ok(FieldType::DateTime),
            _ => Err(format!("Type inconnu: '{}'", type_str)),
        }
    }
}

impl fmt::Display for FieldType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FieldType::String => write!(f, "string"),
            FieldType::Int => write!(f, "int"),
            FieldType::Bool => write!(f, "bool"),
            FieldType::Uuid => write!(f, "uuid"),
            FieldType::DateTime => write!(f, "datetime"),
            FieldType::Optional(inner) => write!(f, "optional<{}>", inner),
            FieldType::List(inner) => write!(f, "list<{}>", inner),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_type_parse_simple() {
        assert_eq!(FieldType::parse("string").unwrap(), FieldType::String);
        assert_eq!(FieldType::parse("int").unwrap(), FieldType::Int);
        assert_eq!(FieldType::parse("uuid").unwrap(), FieldType::Uuid);
    }

    #[test]
    fn test_field_type_parse_optional() {
        let result = FieldType::parse("optional<string>").unwrap();
        assert_eq!(result, FieldType::Optional(Box::new(FieldType::String)));
    }

    #[test]
    fn test_field_type_parse_list() {
        let result = FieldType::parse("list<int>").unwrap();
        assert_eq!(result, FieldType::List(Box::new(FieldType::Int)));
    }

    #[test]
    fn test_field_type_to_proto() {
        assert_eq!(FieldType::String.to_proto_type(), "string");
        assert_eq!(FieldType::Int.to_proto_type(), "int32");
        assert_eq!(
            FieldType::List(Box::new(FieldType::String)).to_proto_type(),
            "repeated string"
        );
    }

    #[test]
    fn test_field_type_to_rust() {
        assert_eq!(FieldType::String.to_rust_type(), "String");
        assert_eq!(FieldType::Int.to_rust_type(), "i32");
        assert_eq!(
            FieldType::Optional(Box::new(FieldType::String)).to_rust_type(),
            "Option<String>"
        );
    }
}
