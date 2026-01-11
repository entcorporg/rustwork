use crate::grpc::ast::Service;
use std::fmt::Write;

/// Génère le client
pub(crate) fn generate_client(
    output: &mut String,
    service: &Service,
) -> Result<(), std::fmt::Error> {
    let service_snake = super::utils::to_snake_case(&service.name);

    writeln!(output, "/// Client gRPC pour {}", service.name)?;
    writeln!(output, "///")?;
    writeln!(output, "/// # Exemple")?;
    writeln!(output, "/// ```no_run")?;
    writeln!(
        output,
        "/// let mut client = {}_client(\"http://localhost:50051\").await?;",
        service_snake
    )?;
    writeln!(output, "/// ```")?;
    writeln!(
        output,
        "pub async fn {}_client(addr: impl Into<String>) -> Result<proto::{}_client::{}Client<Channel>, Box<dyn std::error::Error>> {{",
        service_snake, service_snake, service.name
    )?;
    writeln!(output, "    let addr = addr.into();")?;
    writeln!(
        output,
        "    let client = proto::{}_client::{}Client::connect(addr).await?;",
        service_snake, service.name
    )?;
    writeln!(output, "    Ok(client)")?;
    writeln!(output, "}}")?;

    Ok(())
}
