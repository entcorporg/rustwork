/// Générateur de code Rust (traits, serveurs, clients) à partir de l'AST
use crate::grpc::ast::Contract;
use std::fmt::Write;

mod client;
mod handler;
mod imports;
mod server;
mod utils;

pub use utils::generate_grpc_mod;

/// Génère le code Rust complet pour un service gRPC
pub fn generate_rust_service(contract: &Contract) -> Result<String, std::fmt::Error> {
    let mut output = String::new();

    // En-tête et imports
    imports::generate_imports(&mut output, &contract.service)?;
    writeln!(&mut output)?;

    // Trait du handler
    handler::generate_handler_trait(&mut output, &contract.service)?;
    writeln!(&mut output)?;

    // Implémentation du serveur
    server::generate_server_impl(&mut output, &contract.service)?;
    writeln!(&mut output)?;

    // Fonction d'initialisation du serveur
    server::generate_server_init(&mut output, &contract.service)?;
    writeln!(&mut output)?;

    // Client
    client::generate_client(&mut output, &contract.service)?;

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::utils::to_snake_case;
    use super::*;
    use crate::grpc::ast::{Rpc, Service};

    #[test]
    fn test_to_snake_case() {
        assert_eq!(to_snake_case("UserService"), "user_service");
        assert_eq!(to_snake_case("GetUser"), "get_user");
    }

    #[test]
    fn test_generate_handler_trait() {
        let service = Service {
            name: "UserService".to_string(),
            rpcs: vec![Rpc {
                name: "GetUser".to_string(),
                input_type: "GetUserRequest".to_string(),
                output_type: "User".to_string(),
            }],
        };

        let mut output = String::new();
        handler::generate_handler_trait(&mut output, &service).unwrap();

        assert!(output.contains("trait UserServiceHandler"));
        assert!(output.contains("async fn get_user"));
        assert!(output.contains("GetUserRequest"));
        assert!(output.contains("User"));
    }
}
