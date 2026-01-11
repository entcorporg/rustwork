use anyhow::Result;
use clap::{Parser, Subcommand};

mod commands;
mod grpc;
mod mcp;
mod templates;

#[derive(Parser)]
#[command(name = "rustwork")]
#[command(about = "Rustwork CLI - Laravel-style Rust framework", long_about = None)]
#[command(version)]
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
        /// Project layout: monolith (default) or micro
        #[arg(long, default_value = "monolith")]
        layout: String,
        /// Services to create (comma-separated, required if layout=micro)
        #[arg(long, value_delimiter = ',')]
        services: Option<Vec<String>>,
        /// Create a shared library (for microservices layout)
        #[arg(long)]
        shared: bool,
    },
    /// Add a new service to an existing microservices project
    AddService {
        /// Name of the service to add
        name: String,
        /// Path to the microservices project (default: current directory)
        #[arg(long, default_value = ".")]
        project: String,
    },
    /// Generate code from templates
    Make {
        #[command(subcommand)]
        generator: Generator,
    },
    /// Database management commands
    Db {
        #[command(subcommand)]
        action: DbAction,
    },
    /// Start development server with hot-reload
    Dev {
        /// Enable MCP (Model Context Protocol) server
        #[arg(long)]
        mcp: bool,
    },
    /// Start MCP (Model Context Protocol) server for IDE integration
    Mcp {
        /// Use stdio transport (for VS Code integration)
        #[arg(long)]
        stdio: bool,
        /// Host address to bind to (default: 127.0.0.1)
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
        /// Port to listen on (default: 4000)
        #[arg(long, default_value_t = 4000)]
        port: u16,
        /// Path to the Rustwork project (default: current directory)
        #[arg(long, default_value = ".")]
        project: String,
    },
    /// gRPC service management
    Grpc {
        #[command(subcommand)]
        action: GrpcAction,
    },
}

#[derive(Subcommand)]
enum GrpcAction {
    /// Build gRPC services from .rwk files
    Build {
        /// Path to the project (default: current directory)
        #[arg(long)]
        project: Option<String>,
    },
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

#[derive(Subcommand)]
enum DbAction {
    /// Run pending database migrations
    Migrate {
        /// Number of migrations to run (default: all)
        #[arg(short, long)]
        steps: Option<u32>,
    },
    /// Rollback the last migration(s)
    Rollback {
        /// Number of migrations to rollback (default: 1)
        #[arg(short, long, default_value = "1")]
        steps: u32,
    },
    /// Show migration status
    Status,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New {
            name,
            layout,
            services,
            shared,
        } => {
            commands::new::execute(&name, &layout, services, shared).await?;
        }
        Commands::AddService { name, project } => {
            commands::add_service::execute(&name, &project).await?;
        }
        Commands::Make { generator } => match generator {
            Generator::Controller { name } => {
                commands::make_controller(&name).await?;
            }
            Generator::Model { name } => {
                commands::make_model(&name).await?;
            }
        },
        Commands::Db { action } => match action {
            DbAction::Migrate { steps } => {
                commands::db::migrate(steps).await?;
            }
            DbAction::Rollback { steps } => {
                commands::db::rollback(Some(steps)).await?;
            }
            DbAction::Status => {
                commands::db::status().await?;
            }
        },
        Commands::Dev { mcp } => {
            commands::dev::execute(mcp).await?;
        }
        Commands::Mcp {
            stdio,
            host,
            port,
            project,
        } => {
            let project_path = std::path::PathBuf::from(project);
            if !project_path.exists() {
                anyhow::bail!("Project path does not exist: {}", project_path.display());
            }

            if stdio {
                mcp::run_stdio_server(project_path).await?;
            } else {
                mcp::run_server(&host, port, project_path).await?;
            }
        }
        Commands::Grpc { action } => match action {
            GrpcAction::Build { project } => {
                commands::grpc_build::execute(project).await?;
            }
        },
    }

    Ok(())
}
