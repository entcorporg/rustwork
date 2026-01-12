use serde::{Deserialize, Serialize};

/// Service overview representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceOverview {
    pub name: String,
    pub path: String,
    pub port: Option<u16>,
    pub responsibility: Option<String>,
    pub status: ServiceStatus,
    pub routes_count: usize,
    pub grpc_services_count: usize,
    pub models_count: usize,
    pub middleware_count: usize,
    pub tests_count: usize,
    pub lines_of_code: usize,
    pub database: Option<DatabaseInfo>,
    pub depends_on: Vec<String>,
    pub called_by: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ServiceStatus {
    Running,
    Stopped,
    Unknown,
}

/// Database information for a service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseInfo {
    pub db_type: String,
    pub tables: Vec<String>,
}

/// Service dependency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDependency {
    pub from: String,
    pub to: String,
    pub dependency_type: DependencyType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DependencyType {
    HttpCall,
    GrpcCall,
    SharedDatabase,
    Unknown,
}
