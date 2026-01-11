use super::types::Parser;
use crate::grpc::ast::{Field, FieldType, Message};
use crate::grpc::errors::ParseResult;

impl Parser {
    /// Parse tous les messages
    pub(super) fn parse_messages(&mut self) -> ParseResult<Vec<Message>> {
        let mut messages = Vec::new();

        while self.current_line < self.lines.len() {
            if let Some(line) = self.peek_next_non_empty_line() {
                if line.starts_with("message ") {
                    messages.push(self.parse_message()?);
                } else {
                    self.current_line += 1;
                }
            } else {
                break;
            }
        }

        Ok(messages)
    }

    /// Parse un message individuel
    fn parse_message(&mut self) -> ParseResult<Message> {
        let line = self.find_next_non_empty_line()?;
        self.current_line += 1;

        // Extraire le nom du message
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 || parts[0] != "message" {
            return Err(self.error(format!("Attendu 'message <Name>', trouvé '{}'", line), 0));
        }

        let message_name = parts[1].trim_end_matches('{').trim().to_string();

        // Parser les champs
        let fields = self.parse_fields()?;

        Ok(Message {
            name: message_name,
            fields,
        })
    }

    /// Parse les champs d'un message
    fn parse_fields(&mut self) -> ParseResult<Vec<Field>> {
        let mut fields = Vec::new();

        while let Some(line) = self.peek_next_non_empty_line() {
            // Si on trouve "}", on termine le message
            if line.trim() == "}" {
                self.current_line += 1;
                break;
            }

            // Si on trouve "message", on termine (pas de accolades)
            if line.starts_with("message") {
                break;
            }

            // Parser un champ
            fields.push(self.parse_field()?);
        }

        Ok(fields)
    }

    /// Parse un champ individuel
    /// Format: <name>: <type>
    fn parse_field(&mut self) -> ParseResult<Field> {
        let line = self.find_next_non_empty_line()?;
        let line_num = self.current_line;
        self.current_line += 1;

        let line = line.trim().trim_end_matches('}').trim();

        // Trouver le séparateur ':'
        let colon_pos = line.find(':').ok_or_else(|| {
            self.error_at("':' manquant dans la déclaration du champ", line_num, 0)
        })?;

        let name = line[..colon_pos].trim().to_string();
        let type_str = line[colon_pos + 1..].trim();

        if name.is_empty() {
            return Err(self.error_at("Nom de champ vide", line_num, 0));
        }

        let field_type = FieldType::parse(type_str).map_err(|e| {
            self.error_at(
                format!("Type invalide '{}': {}", type_str, e),
                line_num,
                colon_pos + 1,
            )
        })?;

        Ok(Field { name, field_type })
    }
}
