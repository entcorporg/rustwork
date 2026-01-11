use crate::grpc::ast::{Rpc, Service};
use std::fmt::Write;

/// Génère le trait que l'utilisateur doit implémenter
pub(crate) fn generate_handler_trait(
    output: &mut String,
    service: &Service,
) -> Result<(), std::fmt::Error> {
    let trait_name = format!("{}Handler", service.name);

    writeln!(
        output,
        "/// Trait à implémenter pour gérer les requêtes {}",
        service.name
    )?;
    writeln!(output, "#[async_trait]")?;
    writeln!(output, "pub trait {}: Send + Sync + 'static {{", trait_name)?;

    for rpc in &service.rpcs {
        generate_handler_method(output, rpc)?;
    }

    writeln!(output, "}}")?;
    Ok(())
}

/// Génère une méthode du trait handler
fn generate_handler_method(output: &mut String, rpc: &Rpc) -> Result<(), std::fmt::Error> {
    let method_name = super::utils::to_snake_case(&rpc.name);

    writeln!(output, "    /// Handler pour {}", rpc.name)?;
    writeln!(
        output,
        "    async fn {}(&self, request: {}) -> Result<{}, Status>;",
        method_name, rpc.input_type, rpc.output_type
    )?;
    writeln!(output)?;
    Ok(())
}
