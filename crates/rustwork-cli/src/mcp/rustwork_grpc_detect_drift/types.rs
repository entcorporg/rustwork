/// Types pour rustwork_grpc_detect_drift
use serde::{Deserialize, Serialize};

/// Résultat de la détection de drift
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftDetectionResult {
    pub has_drift: bool,
    pub drifts: Vec<DriftIssue>,
    pub impacted_services: Vec<String>,
}

/// Type de drift détecté
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DriftKind {
    /// Fichier .proto manquant alors que .rwk existe
    MissingProto,
    /// Fichier .proto présent mais .rwk absent
    OrphanedProto,
    /// build.rs manquant dans un service gRPC
    MissingBuildRs,
    /// Code Rust généré manquant
    MissingGeneratedCode,
    /// Timestamp .proto < .rwk (proto obsolète)
    OutdatedProto,
    /// Timestamp code généré < .proto (code obsolète)
    OutdatedGeneratedCode,
    /// Service défini dans .rwk mais supprimé du code
    DeletedService,
}

/// Problème de drift détecté
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftIssue {
    pub kind: DriftKind,
    pub severity: DriftSeverity,
    pub message: String,
    pub rwk_file: Option<String>,
    pub proto_file: Option<String>,
    pub generated_file: Option<String>,
    pub impacted_service: String,
}

/// Sévérité d'un drift
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum DriftSeverity {
    Info,
    Warning,
    Error,
}

impl DriftDetectionResult {
    /// Crée un résultat sans drift
    pub fn no_drift() -> Self {
        Self {
            has_drift: false,
            drifts: Vec::new(),
            impacted_services: Vec::new(),
        }
    }

    /// Ajoute un drift détecté
    pub fn add_drift(&mut self, drift: DriftIssue) {
        self.has_drift = true;

        if !self.impacted_services.contains(&drift.impacted_service) {
            self.impacted_services.push(drift.impacted_service.clone());
        }

        self.drifts.push(drift);
    }
}

/// Chemins attendus pour un service gRPC
#[derive(Debug, Clone)]
pub struct ServicePaths {
    pub rwk_file: String,
    pub proto_file: String,
    pub build_rs: String,
    pub generated_code: String,
}
