use minijinja::Environment;
use serde_json::Value;
use std::collections::HashMap;

#[allow(dead_code)]
pub type TemplateContext = HashMap<String, Value>;

/// Crée un fichier à partir d'un template
/// Fonction utilitaire partagée par toutes les architectures
#[allow(dead_code)]
pub async fn render_template_to_string(
    env: &Environment<'_>,
    template_name: &str,
    context: &TemplateContext,
) -> anyhow::Result<String> {
    let template = env.get_template(template_name)?;
    let content = template.render(context)?;
    Ok(content)
}
