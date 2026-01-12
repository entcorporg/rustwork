/// Types pour rustwork_grpc_diff_versions
use serde::{Deserialize, Serialize};

/// Paramètres d'entrée du tool
#[derive(Debug, Clone, Deserialize)]
pub struct DiffVersionsParams {
    /// Chemin du fichier .rwk à analyser (relatif au workspace)
    pub rwk_path: String,
    /// Référence Git à comparer avec
    /// Valeurs autorisées: "main", "commit:<sha>", "tag:<name>"
    pub compare_with: String,
}

/// Résultat complet de la comparaison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffResult {
    pub breaking_changes: Vec<Change>,
    pub compatible_changes: Vec<Change>,
    pub migration_needed: bool,
    pub migration_suggestions: Vec<String>,
}

/// Représente un changement détecté
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Change {
    /// Type de changement (service_removed, rpc_removed, field_removed, etc.)
    pub change_type: ChangeType,
    /// Nom du service affecté
    pub service: Option<String>,
    /// Nom du RPC affecté (si applicable)
    pub rpc: Option<String>,
    /// Nom du message affecté (si applicable)
    pub message: Option<String>,
    /// Nom du field affecté (si applicable)
    pub field: Option<String>,
    /// Ancien type (si applicable)
    pub old_type: Option<String>,
    /// Nouveau type (si applicable)
    pub new_type: Option<String>,
    /// Description humaine du changement
    pub description: String,
    /// Sévérité
    pub severity: Severity,
}

/// Types de changements possibles
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ChangeType {
    ServiceRemoved,
    ServiceRenamed,
    ServiceAdded,
    RpcRemoved,
    RpcAdded,
    RpcSignatureChanged,
    MessageRemoved,
    MessageRenamed,
    MessageAdded,
    FieldRemoved,
    FieldAdded,
    FieldTypeChanged,
    FieldOptionalityChanged,
}

/// Sévérité du changement
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Breaking,
    Compatible,
    Neutral,
}
