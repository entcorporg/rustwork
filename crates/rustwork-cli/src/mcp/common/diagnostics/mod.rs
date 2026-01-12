mod collection;
mod collector;
mod parsers;
mod types;

pub use collection::DiagnosticCollection;
pub use collector::DiagnosticCollector;

#[cfg(test)]
mod tests {
    use super::parsers::parse_text_diagnostic;
    use super::types::{Diagnostic, Severity};
    use super::DiagnosticCollection;

    #[test]
    fn test_parse_text_error() {
        let line = "error[E0425]: cannot find value `x` in this scope";
        let diag = parse_text_diagnostic(line, "rustc").unwrap();

        assert_eq!(diag.severity, Severity::Error);
        assert_eq!(diag.code, Some("E0425".to_string()));
        assert!(diag.message.contains("cannot find value"));
    }

    #[test]
    fn test_parse_text_warning() {
        let line = "warning: unused variable: `y`";
        let diag = parse_text_diagnostic(line, "rustc").unwrap();

        assert_eq!(diag.severity, Severity::Warning);
        assert!(diag.message.contains("unused variable"));
    }

    #[test]
    fn test_diagnostic_collection() {
        let mut collection = DiagnosticCollection::new();

        collection.add(Diagnostic {
            severity: Severity::Error,
            message: "test error".to_string(),
            file: Some("test.rs".to_string()),
            line: Some(10),
            column: Some(5),
            code: None,
            source: "test".to_string(),
            timestamp: 0,
        });

        assert_eq!(collection.errors, 1);
        assert_eq!(collection.diagnostics.len(), 1);

        let file_diags = collection.for_file("test.rs");
        assert_eq!(file_diags.len(), 1);
    }
}
