use anyhow::Result;

/// Point d'entrée de la commande new
pub async fn execute(
    project_name: &str,
    layout: &str,
    services: Option<Vec<String>>,
    shared: bool,
) -> Result<()> {
    match layout {
        "monolith" => {
            if services.is_some() {
                println!("⚠️  Warning: --services is ignored for monolith layout");
            }
            if shared {
                println!("⚠️  Warning: --shared is ignored for monolith layout");
            }
            super::monolith::create_monolith_project(project_name).await
        }
        "micro" => {
            let services = services.ok_or_else(|| {
                anyhow::anyhow!("--services is required when using --layout micro")
            })?;
            let services: Vec<String> = services.into_iter().filter(|s| !s.is_empty()).collect();
            if services.is_empty() {
                anyhow::bail!("At least one service must be specified with --services");
            }
            super::microservices::create_microservices_project(project_name, services, shared).await
        }
        _ => anyhow::bail!("Invalid layout '{}'. Must be 'monolith' or 'micro'", layout),
    }
}
