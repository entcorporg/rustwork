use super::types::{Convention, ConventionScope};
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::Path;

/// Gestionnaire de chargement et fusion des conventions
pub struct ConventionLoader {
    framework_conventions: Vec<Convention>,
    project_conventions: Option<Vec<Convention>>,
}

impl ConventionLoader {
    /// Créer un nouveau loader
    pub fn new() -> Self {
        Self {
            framework_conventions: Vec::new(),
            project_conventions: None,
        }
    }

    /// Charger les conventions framework (embedded dans le binaire)
    pub fn load_framework_conventions(&mut self) -> Result<()> {
        // Les conventions framework sont chargées depuis des fichiers embarqués
        let framework_json = include_str!("../../../data/conventions/framework.json");
        self.framework_conventions = serde_json::from_str(framework_json)
            .context("Failed to parse framework conventions")?;
        Ok(())
    }

    /// Charger les conventions projet depuis .rustwork/conventions.json
    pub fn load_project_conventions(&mut self, workspace_root: &Path) -> Result<()> {
        let project_conventions_path = workspace_root.join(".rustwork/conventions.json");

        if !project_conventions_path.exists() {
            // Pas de conventions projet, c'est normal
            return Ok(());
        }

        let content = std::fs::read_to_string(&project_conventions_path)
            .context("Failed to read project conventions")?;

        self.project_conventions =
            Some(serde_json::from_str(&content).context("Failed to parse project conventions")?);

        Ok(())
    }

    /// Fusionner les conventions selon la règle de priorité : projet > framework
    /// IMPORTANT : Si une convention projet existe avec le même ID, elle remplace totalement celle du framework
    pub fn merge_conventions(&self) -> Vec<Convention> {
        if let Some(ref project_convs) = self.project_conventions {
            // Créer un index des IDs de conventions projet
            let project_ids: HashMap<String, &Convention> =
                project_convs.iter().map(|c| (c.id.clone(), c)).collect();

            // Fusionner : projet écrase framework
            let mut merged = Vec::new();

            // 1. Ajouter toutes les conventions framework non surchargées
            for framework_conv in &self.framework_conventions {
                if !project_ids.contains_key(&framework_conv.id) {
                    merged.push(framework_conv.clone());
                }
            }

            // 2. Ajouter toutes les conventions projet (qui écrasent ou complètent)
            merged.extend(project_convs.clone());

            merged
        } else {
            // Pas de conventions projet, on retourne uniquement celles du framework
            self.framework_conventions.clone()
        }
    }

    /// Récupérer uniquement les catégories racines (sans leur contenu)
    pub fn get_root_categories(&self) -> Vec<RootCategory> {
        let conventions = self.merge_conventions();

        conventions
            .iter()
            .map(|c| RootCategory {
                id: c.id.clone(),
                label: c.label.clone(),
                description: c.description.clone(),
                scope: c.scope.clone(),
                has_children: c.children.is_some() && !c.children.as_ref().unwrap().is_empty(),
                has_rules: c.rules.is_some() && !c.rules.as_ref().unwrap().is_empty(),
            })
            .collect()
    }

    /// Récupérer une convention par son chemin (ex: "database.migrations.naming")
    pub fn get_by_path(&self, path: &str) -> Option<Convention> {
        let conventions = self.merge_conventions();
        let parts: Vec<&str> = path.split('.').collect();

        self.find_convention_by_parts(&conventions, &parts)
    }

    /// Récupérer une catégorie et ses enfants directs (sans les règles atomiques)
    pub fn get_category(&self, category_id: &str) -> Option<CategoryView> {
        let conventions = self.merge_conventions();

        let convention = conventions.iter().find(|c| c.id == category_id)?;

        Some(CategoryView {
            id: convention.id.clone(),
            label: convention.label.clone(),
            description: convention.description.clone(),
            scope: convention.scope.clone(),
            children: convention.children.as_ref().map(|children| {
                children
                    .iter()
                    .map(|c| ChildSummary {
                        id: c.id.clone(),
                        label: c.label.clone(),
                        description: c.description.clone(),
                        has_children: c.children.is_some()
                            && !c.children.as_ref().unwrap().is_empty(),
                        has_rules: c.rules.is_some() && !c.rules.as_ref().unwrap().is_empty(),
                    })
                    .collect()
            }),
        })
    }

    /// Fonction récursive pour naviguer dans l'arbre
    fn find_convention_by_parts(
        &self,
        conventions: &[Convention],
        parts: &[&str],
    ) -> Option<Convention> {
        if parts.is_empty() {
            return None;
        }

        let current_id = parts[0];
        let convention = conventions.iter().find(|c| c.id == current_id)?;

        if parts.len() == 1 {
            // Trouvé
            return Some(convention.clone());
        }

        // Récursion sur les enfants
        if let Some(ref children) = convention.children {
            return self.find_convention_by_parts(children, &parts[1..]);
        }

        None
    }
}

/// Vue simplifiée d'une catégorie racine
#[derive(Debug, serde::Serialize)]
pub struct RootCategory {
    pub id: String,
    pub label: String,
    pub description: String,
    pub scope: ConventionScope,
    pub has_children: bool,
    pub has_rules: bool,
}

/// Vue d'une catégorie avec ses enfants directs
#[derive(Debug, serde::Serialize)]
pub struct CategoryView {
    pub id: String,
    pub label: String,
    pub description: String,
    pub scope: ConventionScope,
    pub children: Option<Vec<ChildSummary>>,
}

/// Résumé d'un enfant
#[derive(Debug, serde::Serialize)]
pub struct ChildSummary {
    pub id: String,
    pub label: String,
    pub description: String,
    pub has_children: bool,
    pub has_rules: bool,
}
