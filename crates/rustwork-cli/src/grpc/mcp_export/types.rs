/// Types pour l'export MCP des contrats gRPC
use serde::{Deserialize, Serialize};

/// Représentation d'un contrat gRPC pour export MCP
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct GrpcContractExport {
    pub service_name: String,
    pub package: String,
    pub rpcs: Vec<RpcExport>,
    pub messages: Vec<MessageExport>,
    pub file_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct RpcExport {
    pub name: String,
    pub input_type: String,
    pub output_type: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct MessageExport {
    pub name: String,
    pub fields: Vec<FieldExport>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct FieldExport {
    pub name: String,
    pub field_type: String,
}
