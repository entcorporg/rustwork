use anyhow::Result;
use clap::{Parser, Subcommand};

mod commands;
mod grpc;
mod mcp;
mod templates;

#[derive(Parser)]
#[command(name = "rustwork")]
#[command(about = "Rustwork CLI - Microservices Backend Framework for AI-native applications", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Rustwork microservices workspace
    ///
    /// Usage: rustwork new auth,user,session
    ///
    /// Each argument (comma-separated) creates an independent service.
    /// A shared/ library is always created for cross-service code.
    New {
        /// Services to create (comma-separated)
        /// Example: auth,user,session
        #[arg(value_delimiter = ',', required = true)]
        services: Vec<String>,
        /// Skip creating the shared library
        #[arg(long)]
        no_shared: bool,
    },
    /// Add a new service to an existing Rustwork workspace
    AddService {
        /// Name of the service to add
        name: String,
        /// Path to the workspace root (default: auto-detect from current directory)
        #[arg(long)]
        project: Option<String>,
    },
    /// Generate code from templates
    Make {
        #[command(subcommand)]
        generator: Generator,
    },
    /// Start development server with hot-reload
    Dev {
        /// Enable MCP (Model Context Protocol) server
        #[arg(long)]
        mcp: bool,
        /// Explicit path to the workspace root (optional)
        #[arg(long)]
        path: Option<String>,
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
    /// Manage Rustwork conventions
    Conventions {
        #[command(subcommand)]
        action: ConventionsAction,
    },
}

#[derive(Subcommand)]
enum ConventionsAction {
    /// Initialize project conventions file
    Init {
        /// Path to the project (default: current directory)
        #[arg(long)]
        project: Option<String>,
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

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New {
            services,
            no_shared,
        } => {
            commands::new::execute(services, !no_shared).await?;
        }
        Commands::AddService { name, project } => {
            commands::add_service::execute(&name, project.as_deref()).await?;
        }
        Commands::Make { generator } => match generator {
            Generator::Controller { name } => {
                commands::make_controller(&name).await?;
            }
            Generator::Model { name } => {
                commands::make_model(&name).await?;
            }
        },
        Commands::Dev { mcp, path } => {
            let explicit_path = path.as_deref().map(std::path::Path::new);
            commands::dev::execute(mcp, explicit_path).await?;
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
        Commands::Conventions { action } => match action {
            ConventionsAction::Init { project } => {
                let project_path = project.map(std::path::PathBuf::from);
                commands::conventions::conventions_init(project_path)?;
            }
        },
    }

    Ok(())
}
