use super::types::{Diagnostic, Severity};
use serde::{Deserialize, Serialize};

/// Collection of diagnostics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticCollection {
    pub diagnostics: Vec<Diagnostic>,
    pub errors: usize,
    pub warnings: usize,
    pub last_build_success: bool,
}

impl DiagnosticCollection {
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
            errors: 0,
            warnings: 0,
            last_build_success: true,
        }
    }

    pub fn add(&mut self, diagnostic: Diagnostic) {
        match diagnostic.severity {
            Severity::Error => self.errors += 1,
            Severity::Warning => self.warnings += 1,
            _ => {}
        }
        self.diagnostics.push(diagnostic);
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.diagnostics.clear();
        self.errors = 0;
        self.warnings = 0;
    }

    /// Get diagnostics for a specific file
    #[allow(dead_code)]
    pub fn for_file(&self, file: &str) -> Vec<&Diagnostic> {
        self.diagnostics
            .iter()
            .filter(|d| d.file.as_ref().map(|f| f.contains(file)).unwrap_or(false))
            .collect()
    }

    /// Get all errors
    #[allow(dead_code)]
    pub fn errors_only(&self) -> Vec<&Diagnostic> {
        self.diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect()
    }

    /// Get all warnings
    #[allow(dead_code)]
    pub fn warnings_only(&self) -> Vec<&Diagnostic> {
        self.diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Warning)
            .collect()
    }
}
