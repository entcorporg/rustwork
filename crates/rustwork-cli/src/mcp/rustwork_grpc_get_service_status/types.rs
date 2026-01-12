/// Types pour rustwork_grpc_get_service_status
use serde::{Deserialize, Serialize};

/// État d'un service gRPC
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceStatus {
    pub service_name: String,
    pub status: ServiceState,
    pub rwk_file: Option<RwkFileStatus>,
    pub generated_code: Option<GeneratedCodeStatus>,
    pub inconsistencies: Vec<Inconsistency>,
}

/// État général du service
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ServiceState {
    /// Service connu et opérationnel
    Known,
    /// Service introuvable
    Unknown,
    /// Service présent mais avec des problèmes
    Degraded,
}

/// État du fichier .rwk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RwkFileStatus {
    pub path: String,
    pub exists: bool,
    pub parsable: bool,
    pub parse_error: Option<String>,
}

/// État du code généré
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedCodeStatus {
    pub proto_file: Option<String>,
    pub rust_client: Option<String>,
    pub rust_server: Option<String>,
    pub proto_exists: bool,
    pub client_exists: bool,
    pub server_exists: bool,
}

/// Incohérence détectée
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inconsistency {
    pub kind: InconsistencyKind,
    pub message: String,
}

/// Types d'incohérences
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InconsistencyKind {
    /// .rwk absent
    MissingRwkFile,
    /// .rwk non parsable
    InvalidRwkFile,
    /// Code généré absent
    MissingGeneratedCode,
    /// Code généré obsolète (nécessite rebuild)
    OutdatedGeneratedCode,
    /// Fichiers incohérents
    InconsistentFiles,
}
