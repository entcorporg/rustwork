use crate::config::types::CorsConfig;
use anyhow::Result;

/// Valide la configuration CORS au démarrage
pub fn validate_cors_config(config: &CorsConfig) -> Result<()> {
    if !config.enabled {
        return Ok(());
    }

    // Si CORS est activé, il DOIT y avoir des origines configurées
    if config.allowed_origins.is_empty() {
        anyhow::bail!(
            "CORS is enabled but no allowed_origins are configured. \
             Either set cors.enabled=false or provide at least one origin in cors.allowed_origins"
        );
    }

    // Valider que chaque origine est une URL valide
    for origin in &config.allowed_origins {
        if origin.trim().is_empty() {
            anyhow::bail!("CORS allowed_origins contains an empty string");
        }

        // Vérifier que l'origine commence par http:// ou https://
        if !origin.starts_with("http://") && !origin.starts_with("https://") {
            anyhow::bail!(
                "Invalid CORS origin '{}': must start with http:// or https://",
                origin
            );
        }

        // Interdire les wildcards
        if origin.contains('*') {
            anyhow::bail!(
                "Wildcard origins are not allowed in CORS configuration. Found: '{}'",
                origin
            );
        }
    }

    Ok(())
}
