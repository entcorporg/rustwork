use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Configuration CORS sécurisée (fail-by-default)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorsConfig {
    /// Active ou désactive CORS
    #[serde(default)]
    pub enabled: bool,

    /// Liste des origines autorisées (REQUIS si enabled=true)
    #[serde(default)]
    pub allowed_origins: Vec<String>,

    /// Méthodes HTTP autorisées
    #[serde(default = "default_cors_methods")]
    pub allowed_methods: Vec<String>,

    /// Headers autorisés
    #[serde(default = "default_cors_headers")]
    pub allowed_headers: Vec<String>,

    /// Autoriser les credentials (cookies, auth headers)
    #[serde(default)]
    pub allow_credentials: bool,

    /// Durée de cache des preflight (en secondes)
    #[serde(default = "default_max_age")]
    pub max_age_seconds: u64,
}

fn default_cors_methods() -> Vec<String> {
    vec![
        "GET".to_string(),
        "POST".to_string(),
        "PUT".to_string(),
        "PATCH".to_string(),
        "DELETE".to_string(),
    ]
}

fn default_cors_headers() -> Vec<String> {
    vec!["Content-Type".to_string(), "Accept".to_string()]
}

fn default_max_age() -> u64 {
    3600
}

impl Default for CorsConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            allowed_origins: Vec::new(),
            allowed_methods: default_cors_methods(),
            allowed_headers: default_cors_headers(),
            allow_credentials: false,
            max_age_seconds: default_max_age(),
        }
    }
}

impl CorsConfig {
    /// Valide la configuration CORS au démarrage
    pub fn validate(&self) -> Result<()> {
        crate::config::builders::validate_cors_config::validate_cors_config(self)
    }
}
