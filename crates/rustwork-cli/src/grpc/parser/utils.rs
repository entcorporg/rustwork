use super::types::Parser;
use crate::grpc::errors::ParseResult;

impl Parser {
    /// Trouve la prochaine ligne non vide
    pub(super) fn find_next_non_empty_line(&mut self) -> ParseResult<String> {
        while self.current_line < self.lines.len() {
            let line = self.lines[self.current_line].trim();
            if !line.is_empty() && !line.starts_with("//") {
                return Ok(line.to_string());
            }
            self.current_line += 1;
        }
        Err(self.error("Fin de fichier inattendue", 0))
    }

    /// Regarde la prochaine ligne sans avancer
    pub(super) fn peek_next_non_empty_line(&self) -> Option<String> {
        let mut idx = self.current_line;
        while idx < self.lines.len() {
            let line = self.lines[idx].trim();
            if !line.is_empty() && !line.starts_with("//") {
                return Some(line.to_string());
            }
            idx += 1;
        }
        None
    }

    /// Crée une erreur à la ligne courante
    pub(super) fn error(
        &self,
        message: impl Into<String>,
        column: usize,
    ) -> crate::grpc::errors::ParseError {
        crate::grpc::errors::ParseError::new(message, self.current_line + 1, column)
            .with_source(&self.source)
    }

    /// Crée une erreur à une ligne spécifique
    pub(super) fn error_at(
        &self,
        message: impl Into<String>,
        line: usize,
        column: usize,
    ) -> crate::grpc::errors::ParseError {
        crate::grpc::errors::ParseError::new(message, line + 1, column).with_source(&self.source)
    }
}
