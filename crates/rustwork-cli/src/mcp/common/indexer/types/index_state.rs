use serde::{Deserialize, Serialize};
use std::fmt;

/// État du cycle de vie de l'index MCP
///
/// RÈGLE P0 : L'index DOIT avoir un état explicite, observable et déterministe.
/// Cet état DOIT être vérifié avant toute opération dépendant de l'index.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IndexState {
    /// Index pas encore initialisé
    /// → Aucune donnée disponible
    /// → get_file_doc DOIT refuser
    NotStarted,

    /// Scan en cours
    /// → Données partielles, non fiables
    /// → get_file_doc DOIT refuser
    Scanning,

    /// Index complet et validé
    /// → Seul état où get_file_doc peut répondre
    /// → Garantit cohérence des données
    Ready,

    /// Index invalidé par changements filesystem
    /// → Rescan nécessaire
    /// → get_file_doc DOIT refuser
    Invalidated,

    /// Échec du scan
    /// → Erreur critique
    /// → get_file_doc DOIT refuser avec suggestion diagnostics
    Failed,
}

impl IndexState {
    /// Vérifie si l'index est utilisable (uniquement READY)
    pub fn is_ready(&self) -> bool {
        matches!(self, IndexState::Ready)
    }

    /// Code d'erreur à retourner si l'état n'est pas READY
    pub fn error_code(&self) -> &'static str {
        match self {
            IndexState::NotStarted => "INDEX_NOT_STARTED",
            IndexState::Scanning => "INDEX_SCANNING",
            IndexState::Ready => "INDEX_READY", // Ne devrait jamais être appelé
            IndexState::Invalidated => "INDEX_INVALIDATED",
            IndexState::Failed => "INDEX_FAILED",
        }
    }

    /// Message d'erreur explicite
    pub fn error_message(&self) -> String {
        match self {
            IndexState::NotStarted => {
                "Index not initialized - initial scan has not completed yet".to_string()
            }
            IndexState::Scanning => {
                "Index is currently being scanned - data is incomplete".to_string()
            }
            IndexState::Ready => "Index is ready".to_string(),
            IndexState::Invalidated => {
                "Index has been invalidated by file changes - rescan in progress".to_string()
            }
            IndexState::Failed => "Index scan failed - check diagnostics for errors".to_string(),
        }
    }
}

impl fmt::Display for IndexState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IndexState::NotStarted => write!(f, "not_started"),
            IndexState::Scanning => write!(f, "scanning"),
            IndexState::Ready => write!(f, "ready"),
            IndexState::Invalidated => write!(f, "invalidated"),
            IndexState::Failed => write!(f, "failed"),
        }
    }
}
