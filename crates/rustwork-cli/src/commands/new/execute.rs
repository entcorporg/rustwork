use anyhow::Result;

/// Point d'entrée de la commande new
///
/// Usage: rustwork new auth,user,session
///
/// Crée un workspace micro-services avec la structure:
/// ```text
/// ./
/// ├── .vscode/
/// │   ├── settings.json
/// │   └── mcp.example.json
/// ├── Backend/
/// │   ├── services/
/// │   │   ├── auth/
/// │   │   ├── user/
/// │   │   ├── session/
/// │   │   └── shared/
/// │   └── README.md
/// ```
pub async fn execute(services: Vec<String>, create_shared: bool) -> Result<()> {
    // Filtrer les services vides et valider
    let services: Vec<String> = services
        .into_iter()
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty())
        .collect();

    if services.is_empty() {
        anyhow::bail!(
            "At least one service must be specified.\n\
             \n\
             Usage: rustwork new auth,user,session\n\
             \n\
             Each comma-separated value becomes an independent service."
        );
    }

    // Vérifier les noms de services valides
    for service in &services {
        if !is_valid_service_name(service) {
            anyhow::bail!(
                "Invalid service name: '{}'\n\
                 Service names must be lowercase alphanumeric with underscores only.",
                service
            );
        }

        // Interdire 'shared' comme nom de service
        if service == "shared" {
            anyhow::bail!(
                "'shared' is a reserved name for the shared library.\n\
                 Please choose a different service name."
            );
        }
    }

    // Vérifier les doublons
    let unique_count = {
        let mut unique = services.clone();
        unique.sort();
        unique.dedup();
        unique.len()
    };

    if unique_count != services.len() {
        anyhow::bail!("Duplicate service names detected. Each service must have a unique name.");
    }

    super::microservices::create_microservices_workspace(services, create_shared).await
}

/// Vérifie si un nom de service est valide
fn is_valid_service_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    // Doit commencer par une lettre
    if !name
        .chars()
        .next()
        .map(|c| c.is_ascii_lowercase())
        .unwrap_or(false)
    {
        return false;
    }

    // Ne doit contenir que des lettres minuscules, chiffres et underscores
    name.chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_service_names() {
        assert!(is_valid_service_name("auth"));
        assert!(is_valid_service_name("user_service"));
        assert!(is_valid_service_name("api_v2"));
        assert!(is_valid_service_name("a"));
    }

    #[test]
    fn test_invalid_service_names() {
        assert!(!is_valid_service_name(""));
        assert!(!is_valid_service_name("Auth")); // Uppercase
        assert!(!is_valid_service_name("123abc")); // Starts with number
        assert!(!is_valid_service_name("user-service")); // Hyphen
        assert!(!is_valid_service_name("user.service")); // Dot
    }
}
