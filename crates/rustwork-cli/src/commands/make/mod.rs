/// Commandes de génération (make)
mod common;
mod controller;
mod model;

pub use controller::execute as make_controller;
pub use model::execute as make_model;
