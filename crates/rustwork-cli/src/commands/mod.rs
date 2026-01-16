pub mod add_service;
pub mod conventions;
pub mod dev;
pub mod grpc_build;
pub mod make;
pub mod new;

// Utilities
pub mod utils;

// Re-exports pour compatibilité
pub use make::{make_controller, make_model};
