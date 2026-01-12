/// Types pour rustwork_grpc_list_definitions
use serde::{Deserialize, Serialize};

/// Représente une définition gRPC complète
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrpcDefinition {
    pub service_name: String,
    pub source_file: String,
    pub rpcs: Vec<RpcDefinition>,
    pub messages: Vec<MessageDefinition>,
}

/// Définition d'un RPC
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcDefinition {
    pub name: String,
    pub input_type: String,
    pub output_type: String,
}

/// Définition d'un message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageDefinition {
    pub name: String,
    pub fields: Vec<FieldDefinition>,
}

/// Définition d'un champ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDefinition {
    pub name: String,
    pub field_type: String,
}

/// Dépendance inter-services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDependency {
    pub from_service: String,
    pub to_service: String,
    pub used_messages: Vec<String>,
}
