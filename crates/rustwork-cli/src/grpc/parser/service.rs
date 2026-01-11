use super::types::Parser;
use crate::grpc::ast::{Rpc, Service};
use crate::grpc::errors::ParseResult;

impl Parser {
    /// Parse la déclaration du service
    pub(super) fn parse_service(&mut self) -> ParseResult<Service> {
        // Trouver la ligne "service <Name>"
        let service_line = self.find_next_non_empty_line()?;
        let parts: Vec<&str> = service_line.split_whitespace().collect();

        if parts.len() != 2 || parts[0] != "service" {
            return Err(self.error(
                format!("Attendu 'service <Name>', trouvé '{}'", service_line),
                0,
            ));
        }

        let service_name = parts[1].to_string();
        self.current_line += 1;

        // Parser les RPCs
        let rpcs = self.parse_rpcs()?;

        Ok(Service {
            name: service_name,
            rpcs,
        })
    }

    /// Parse les RPCs
    fn parse_rpcs(&mut self) -> ParseResult<Vec<Rpc>> {
        let mut rpcs = Vec::new();

        while let Some(line) = self.peek_next_non_empty_line() {
            // Si on trouve "message", on arrête les RPCs
            if line.starts_with("message") {
                break;
            }

            // Parser un RPC
            if line.starts_with("rpc ") {
                rpcs.push(self.parse_rpc()?);
            } else {
                self.current_line += 1;
            }
        }

        if rpcs.is_empty() {
            return Err(self.error("Le service doit avoir au moins un RPC", 0));
        }

        Ok(rpcs)
    }

    /// Parse un RPC individuel
    /// Format: rpc <Name> (<InputType>) returns (<OutputType>)
    fn parse_rpc(&mut self) -> ParseResult<Rpc> {
        let line = self.find_next_non_empty_line()?;
        self.current_line += 1;

        // Extraire les composants avec regex simple
        let line = line.trim();
        if !line.starts_with("rpc ") {
            return Err(self.error(format!("Attendu 'rpc', trouvé '{}'", line), 0));
        }

        // Supprimer "rpc "
        let rest = &line[4..];

        // Trouver le nom (jusqu'à la première parenthèse)
        let open_paren = rest
            .find('(')
            .ok_or_else(|| self.error("Parenthèse ouvrante manquante", 0))?;

        let name = rest[..open_paren].trim().to_string();

        // Extraire le type d'entrée
        let input_end = rest
            .find(')')
            .ok_or_else(|| self.error("Parenthèse fermante manquante", 0))?;
        let input_type = rest[open_paren + 1..input_end].trim().to_string();

        // Trouver "returns"
        let returns_pos = rest
            .find("returns")
            .ok_or_else(|| self.error("'returns' manquant", 0))?;

        // Extraire le type de sortie
        let output_start = rest[returns_pos..]
            .find('(')
            .ok_or_else(|| self.error("Parenthèse ouvrante manquante après 'returns'", 0))?
            + returns_pos;
        let output_end = rest[output_start..]
            .find(')')
            .ok_or_else(|| self.error("Parenthèse fermante manquante après 'returns'", 0))?
            + output_start;

        let output_type = rest[output_start + 1..output_end].trim().to_string();

        if name.is_empty() || input_type.is_empty() || output_type.is_empty() {
            return Err(self.error("RPC incomplet", 0));
        }

        Ok(Rpc {
            name,
            input_type,
            output_type,
        })
    }
}
