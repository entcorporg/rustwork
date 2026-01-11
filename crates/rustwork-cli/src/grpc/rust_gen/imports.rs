/// Générateur de code Rust (traits, serveurs, clients) à partir de l'AST
use crate::grpc::ast::Service;
use std::fmt::Write;

/// Génère les imports nécessaires
pub(crate) fn generate_imports(
    output: &mut String,
    service: &Service,
) -> Result<(), std::fmt::Error> {
    let service_snake = super::utils::to_snake_case(&service.name);

    writeln!(
        output,
        "// Généré automatiquement par Rustwork - Ne pas modifier"
    )?;
    writeln!(
        output,
        "#![allow(unused_imports, dead_code, non_snake_case)]"
    )?;
    writeln!(output)?;
    writeln!(output, "use async_trait::async_trait;")?;
    writeln!(output, "use tonic::{{Request, Response, Status}};")?;
    writeln!(output, "use tonic::transport::Channel;")?;
    writeln!(output)?;
    writeln!(output, "// Code généré par tonic depuis OUT_DIR")?;
    writeln!(
        output,
        "// Convention tonic: <service_name>_server et <service_name>_client"
    )?;
    writeln!(output, "pub mod proto {{")?;
    writeln!(
        output,
        "    tonic::include_proto!(\"{}_service\");",
        service_snake
    )?;
    writeln!(output, "}}")?;
    writeln!(output)?;
    writeln!(output, "// Ré-export pour simplifier l'usage")?;
    writeln!(output, "pub use proto::*;")?;
    Ok(())
}
