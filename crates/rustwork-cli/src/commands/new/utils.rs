use anyhow::{Context, Result};
use std::path::Path;
use tokio::fs;

use crate::templates::TemplateContext;

/// Crée un fichier à partir d'un template
pub(crate) async fn create_file(
    path: &Path,
    env: &minijinja::Environment<'_>,
    template_name: &str,
    context: &TemplateContext,
) -> Result<()> {
    crate::commands::utils::ensure_parent_dir(path).await?;

    let template = env
        .get_template(template_name)
        .with_context(|| format!("Failed to get template: {}", template_name))?;

    let content = template
        .render(context)
        .with_context(|| format!("Failed to render template: {}", template_name))?;

    fs::write(path, content)
        .await
        .with_context(|| format!("Failed to write file: {}", path.display()))?;

    Ok(())
}
