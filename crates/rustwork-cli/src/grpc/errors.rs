/// Gestion des erreurs du parser DSL
use std::fmt;

/// Erreur de parsing du DSL .rwk
#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub source: Option<String>,
}

impl ParseError {
    pub fn new(message: impl Into<String>, line: usize, column: usize) -> Self {
        Self {
            message: message.into(),
            line,
            column,
            source: None,
        }
    }

    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    /// Format l'erreur avec contexte
    pub fn format_with_context(&self) -> String {
        let mut output = format!(
            "Erreur de parsing à la ligne {}, colonne {}:\n  {}",
            self.line, self.column, self.message
        );

        if let Some(source) = &self.source {
            output.push_str("\n\n");
            output.push_str(&self.format_source_context(source));
        }

        output
    }

    fn format_source_context(&self, source: &str) -> String {
        let lines: Vec<&str> = source.lines().collect();
        let mut output = String::new();

        // Afficher la ligne problématique et quelques lignes autour
        let start = self.line.saturating_sub(2);
        let end = (self.line + 1).min(lines.len());

        for line_num in start..end {
            let line_content = lines.get(line_num).unwrap_or(&"");
            let marker = if line_num == self.line - 1 {
                "→"
            } else {
                " "
            };

            output.push_str(&format!(
                "{} {:4} | {}\n",
                marker,
                line_num + 1,
                line_content
            ));

            // Ajouter un indicateur ^ sous l'erreur
            if line_num == self.line - 1 && self.column > 0 {
                let spaces = " ".repeat(self.column + 7); // 7 = "→ 1234 | "
                output.push_str(&format!("{}^\n", spaces));
            }
        }

        output
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Parse error at line {}, column {}: {}",
            self.line, self.column, self.message
        )
    }
}

impl std::error::Error for ParseError {}

/// Type résultat pour le parsing
pub type ParseResult<T> = Result<T, ParseError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_error_display() {
        let err = ParseError::new("Type invalide", 5, 10);
        let display = format!("{}", err);
        assert!(display.contains("line 5"));
        assert!(display.contains("column 10"));
    }

    #[test]
    fn test_parse_error_with_context() {
        let source = "service UserService\nrpc GetUser (Invalid) returns (User)";
        let err = ParseError::new("Type invalide", 2, 13).with_source(source);

        let formatted = err.format_with_context();
        assert!(formatted.contains("ligne 2"));
        assert!(formatted.contains("Invalid"));
        assert!(formatted.contains("^"));
    }
}
