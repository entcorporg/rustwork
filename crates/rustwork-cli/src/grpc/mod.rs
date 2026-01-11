/// Module gRPC pour Rustwork
/// Génération automatique de services gRPC à partir de fichiers DSL .rwk
pub mod ast;
pub mod build_gen;
pub mod errors;
pub mod mcp_export;
pub mod parser;
pub mod proto_gen;
pub mod rust_gen;

pub use build_gen::{add_grpc_dependencies, generate_service_build_rs};
pub use parser::parse_contract;
pub use proto_gen::generate_proto;
pub use rust_gen::generate_grpc_mod;
