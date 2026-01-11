pub mod connect_database;
pub mod paginator;

// Re-exports publics
pub use connect_database::connect_database;
pub use paginator::Paginator;

// Alias pour compatibilité avec l'ancien nom
pub use connect_database::connect_database as connect_db;
pub use connect_database::connect_database as init_database;
