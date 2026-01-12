use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Scope d'une convention : framework ou projet
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ConventionScope {
    Framework,
    Project,
}

/// Niveau de criticité d'une convention
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Criticality {
    Required,
    Recommended,
    Optional,
}

/// Contexte d'application d'une convention
/// 
/// Rustwork est 100% microservices
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ConventionContext {
    Microservice,
    Shared,
    All,
}

/// Exemple illustrant une convention
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConventionExample {
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

/// Règle atomique au sein d'une convention
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConventionRule {
    pub id: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rationale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<Vec<ConventionExample>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai_note: Option<String>,
}

/// Nœud de convention (peut être une catégorie ou une règle terminale)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Convention {
    /// Identifiant stable (utilisé pour le path)
    pub id: String,

    /// Label lisible par un humain
    pub label: String,

    /// Description courte
    pub description: String,

    /// Scope : framework ou project
    pub scope: ConventionScope,

    /// Criticité
    #[serde(skip_serializing_if = "Option::is_none")]
    pub criticality: Option<Criticality>,

    /// Contexte d'application
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<ConventionContext>,

    /// Règles atomiques
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<ConventionRule>>,

    /// Sous-conventions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Convention>>,

    /// Note destinée à l'IA
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai_note: Option<String>,

    /// Métadonnées supplémentaires
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

#[allow(dead_code)]
impl Convention {
    /// Créer une catégorie racine
    pub fn category(
        id: impl Into<String>,
        label: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            description: description.into(),
            scope: ConventionScope::Framework,
            criticality: None,
            context: None,
            rules: None,
            children: Some(Vec::new()),
            ai_note: None,
            metadata: None,
        }
    }

    /// Ajouter un enfant à cette convention
    pub fn with_child(mut self, child: Convention) -> Self {
        if let Some(ref mut children) = self.children {
            children.push(child);
        } else {
            self.children = Some(vec![child]);
        }
        self
    }

    /// Définir le scope
    pub fn with_scope(mut self, scope: ConventionScope) -> Self {
        self.scope = scope;
        self
    }

    /// Définir la criticité
    pub fn with_criticality(mut self, criticality: Criticality) -> Self {
        self.criticality = Some(criticality);
        self
    }

    /// Définir le contexte
    pub fn with_context(mut self, context: ConventionContext) -> Self {
        self.context = Some(context);
        self
    }

    /// Ajouter des règles
    pub fn with_rules(mut self, rules: Vec<ConventionRule>) -> Self {
        self.rules = Some(rules);
        self
    }

    /// Ajouter une note IA
    pub fn with_ai_note(mut self, note: impl Into<String>) -> Self {
        self.ai_note = Some(note.into());
        self
    }

    /// Ajouter des métadonnées
    pub fn with_metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.metadata = Some(metadata);
        self
    }
}
