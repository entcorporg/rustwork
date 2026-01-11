use crate::grpc::ast::{Rpc, Service};
use std::fmt::Write;

/// Génère l'implémentation du serveur qui délègue au handler
pub(crate) fn generate_server_impl(
    output: &mut String,
    service: &Service,
) -> Result<(), std::fmt::Error> {
    let handler_trait = format!("{}Handler", service.name);
    let server_name = format!("{}GrpcServer", service.name);
    let service_snake = super::utils::to_snake_case(&service.name);

    writeln!(output, "/// Serveur gRPC qui délègue au handler")?;
    writeln!(
        output,
        "pub struct {}<H: {}> {{",
        server_name, handler_trait
    )?;
    writeln!(output, "    handler: std::sync::Arc<H>,")?;
    writeln!(output, "}}")?;
    writeln!(output)?;

    // Implémentation du trait tonic généré
    // Convention tonic: <service>_server::<Service>
    writeln!(output, "#[async_trait]")?;
    writeln!(
        output,
        "impl<H: {}> proto::{}_server::{} for {}<H> {{",
        handler_trait, service_snake, service.name, server_name
    )?;

    for rpc in &service.rpcs {
        generate_server_method(output, rpc)?;
    }

    writeln!(output, "}}")?;
    Ok(())
}

/// Génère une méthode du serveur
fn generate_server_method(output: &mut String, rpc: &Rpc) -> Result<(), std::fmt::Error> {
    let method_name = super::utils::to_snake_case(&rpc.name);

    writeln!(output)?;
    writeln!(output, "    async fn {}(", method_name)?;
    writeln!(output, "        &self,")?;
    writeln!(output, "        request: Request<{}>,", rpc.input_type)?;
    writeln!(
        output,
        "    ) -> Result<Response<{}>, Status> {{",
        rpc.output_type
    )?;
    writeln!(output, "        let req = request.into_inner();")?;
    writeln!(
        output,
        "        let result = self.handler.{}(req).await?;",
        method_name
    )?;
    writeln!(output, "        Ok(Response::new(result))")?;
    writeln!(output, "    }}")?;

    Ok(())
}

/// Génère la fonction d'initialisation du serveur
pub(crate) fn generate_server_init(
    output: &mut String,
    service: &Service,
) -> Result<(), std::fmt::Error> {
    let handler_trait = format!("{}Handler", service.name);
    let server_name = format!("{}GrpcServer", service.name);
    let service_snake = super::utils::to_snake_case(&service.name);

    writeln!(output, "/// Crée un serveur gRPC à partir d'un handler")?;
    writeln!(output, "///")?;
    writeln!(output, "/// # Exemple")?;
    writeln!(output, "/// ```no_run")?;
    writeln!(output, "/// let handler = MyHandler::new();")?;
    writeln!(output, "/// let service = grpc_service(handler);")?;
    writeln!(output, "/// // Utiliser avec tonic::transport::Server")?;
    writeln!(output, "/// ```")?;
    writeln!(
        output,
        "pub fn grpc_service<H: {}>(handler: H) -> proto::{}_server::{}Server<{}<H>> {{",
        handler_trait, service_snake, service.name, server_name
    )?;
    writeln!(output, "    let handler = std::sync::Arc::new(handler);")?;
    writeln!(
        output,
        "    proto::{}_server::{}Server::new({} {{ handler }})",
        service_snake, service.name, server_name
    )?;
    writeln!(output, "}}")?;

    Ok(())
}
