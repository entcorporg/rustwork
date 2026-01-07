use clap::{Parser, Subcommand};
use anyhow::Result;

mod commands;
mod templates;

#[derive(Parser)]
#[command(name = "rustwork")]
#[command(about = "Rustwork CLI - Laravel-style Rust framework", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Rustwork project
    New {
        /// Name of the project
        name: String,
    },
    /// Generate code from templates
    Make {
        #[command(subcommand)]
        generator: Generator,
    },
    /// Start development server with hot-reload
    Dev,
}

#[derive(Subcommand)]
enum Generator {
    /// Generate a new controller
    Controller {
        /// Name of the controller (PascalCase)
        name: String,
    },
    /// Generate a new model
    Model {
        /// Name of the model (PascalCase)
        name: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { name } => {
            commands::new::execute(&name).await?;
        }
        Commands::Make { generator } => match generator {
            Generator::Controller { name } => {
                commands::make_controller::execute(&name).await?;
            }
            Generator::Model { name } => {
                commands::make_model::execute(&name).await?;
            }
        },
        Commands::Dev => {
            commands::dev::execute().await?;
        }
    }

    Ok(())
}
