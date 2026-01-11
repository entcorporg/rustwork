/// Commande de création de projet
mod execute;
mod microservices;
mod monolith;
mod utils;

pub use execute::execute;
pub use microservices::create_service_in_project;
