use crate::config::builders::resolve_database_url::resolve_database_url;
use crate::config::types::DatabaseConfig;
use crate::errors::AppError;
use anyhow::Result;

/// Retourne une version "sanitisée" de l'URL (password masqué)
pub fn sanitize_database_url(config: &DatabaseConfig) -> Result<String, AppError> {
    let url = resolve_database_url(config)?;

    // Remplacer le password par ***
    if let Some(at_pos) = url.find('@') {
        if let Some(proto_end) = url.find("://") {
            let proto = &url[..proto_end + 3];
            let rest = &url[at_pos..];

            // Chercher le : avant le @
            let between = &url[proto_end + 3..at_pos];
            if let Some(colon_pos) = between.find(':') {
                let username = &between[..colon_pos];
                return Ok(format!("{}{}:***{}", proto, username, rest));
            }
        }
    }

    // Si pas de password détecté, retourner tel quel (ex: SQLite)
    Ok(url)
}
