/// Types pour rustwork_grpc_validate_workspace
use serde::{Deserialize, Serialize};

/// Statut global de validation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ValidationStatus {
    Valid,
    Warning,
    Invalid,
}

/// Résultat de validation du workspace gRPC
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub status: ValidationStatus,
    pub issues: Vec<ValidationIssue>,
    pub impacted_services: Vec<String>,
}

/// Type de problème détecté
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum IssueKind {
    /// Fichier .rwk absent
    MissingRwkFile,
    /// Service exposé mais pas consommé
    OrphanedService,
    /// RPC exposé mais jamais appelé
    OrphanedRpc,
    /// Service consommé mais pas exposé
    UndefinedDependency,
    /// Dépendance circulaire entre services
    CircularDependency,
    /// Incohérence entre DSL et structure
    StructureInconsistency,
}

/// Problème de validation détecté
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationIssue {
    pub kind: IssueKind,
    pub severity: IssueSeverity,
    pub message: String,
    pub location: Option<String>,
    pub impacted_services: Vec<String>,
}

/// Sévérité d'un problème
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum IssueSeverity {
    Info,
    Warning,
    Error,
}

impl ValidationResult {
    /// Crée un résultat vide (valid)
    pub fn valid() -> Self {
        Self {
            status: ValidationStatus::Valid,
            issues: Vec::new(),
            impacted_services: Vec::new(),
        }
    }

    /// Ajoute un problème et recalcule le status
    pub fn add_issue(&mut self, issue: ValidationIssue) {
        // Ajouter les services impactés
        for service in &issue.impacted_services {
            if !self.impacted_services.contains(service) {
                self.impacted_services.push(service.clone());
            }
        }

        // Mettre à jour le status global
        if issue.severity == IssueSeverity::Error {
            self.status = ValidationStatus::Invalid;
        } else if issue.severity == IssueSeverity::Warning && self.status == ValidationStatus::Valid
        {
            self.status = ValidationStatus::Warning;
        }

        self.issues.push(issue);
    }
}
