use anyhow::{Context, Result};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::commands::utils::{detect_rustwork_services, RustworkService};
use crate::mcp::common::workspace_root::WorkspaceRoot;

/// Lance un service avec cargo-watch et préfixe les logs
fn start_service_with_watch(service: &RustworkService) -> Result<Child> {
    let mut child = Command::new("cargo")
        .args(["watch", "-x", "run", "-w", "src", "-w", "config"])
        .current_dir(&service.path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context(format!("Failed to start service: {}", service.name))?;

    // Streamer stdout avec préfixe
    if let Some(stdout) = child.stdout.take() {
        let service_name = service.name.clone();
        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines().map_while(Result::ok) {
                println!("[{}] {}", service_name, line);
            }
        });
    }

    // Streamer stderr avec préfixe
    if let Some(stderr) = child.stderr.take() {
        let service_name = service.name.clone();
        thread::spawn(move || {
            let reader = BufReader::new(stderr);
            for line in reader.lines().map_while(Result::ok) {
                eprintln!("[{}] {}", service_name, line);
            }
        });
    }

    Ok(child)
}

/// Lance un seul service (comportement classique)
async fn run_single_service(
    workspace_root: &WorkspaceRoot,
    service: &RustworkService,
    enable_mcp: bool,
) -> Result<()> {
    println!("🔧 Starting development server with hot-reload...");
    println!("📂 Workspace root: {}", workspace_root.path().display());
    println!("   Watching for changes in src/");

    // Start MCP server in background only if enabled
    if enable_mcp {
        let workspace_path = workspace_root.path().to_path_buf();
        let mcp_port = 4000u16;

        println!(
            "🚀 Starting MCP server on 127.0.0.1:{}... (development only)",
            mcp_port
        );
        println!("   Press Ctrl+C to stop\n");

        // Start MCP in a separate thread to avoid Send issues with syn
        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async move {
                if let Err(e) = crate::mcp::run_server("127.0.0.1", mcp_port, workspace_path).await
                {
                    eprintln!("⚠️  MCP server error: {}", e);
                }
            });
        });
    } else {
        println!("ℹ️  MCP server disabled. Use --mcp to enable it.\n");
    }

    // Check if cargo-watch is installed
    let has_cargo_watch = Command::new("cargo")
        .args(["watch", "--version"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    if !has_cargo_watch {
        println!("⚠️  cargo-watch not found. Installing...");
        println!("   Run: cargo install cargo-watch");
        println!("\nAlternatively, you can run manually:");
        println!("   cargo run");
        anyhow::bail!("cargo-watch is required for dev mode");
    }

    // Run cargo watch
    let mut child = Command::new("cargo")
        .args(["watch", "-x", "run", "-w", "src", "-w", "config"])
        .current_dir(&service.path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("Failed to start cargo watch")?;

    // Stream output
    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines().map_while(Result::ok) {
            println!("{}", line);
        }
    }

    let status = child.wait()?;

    if !status.success() {
        anyhow::bail!("Dev server exited with error");
    }

    Ok(())
}

/// Lance plusieurs services en parallèle
async fn run_multiple_services(
    workspace_root: &WorkspaceRoot,
    services: Vec<RustworkService>,
    enable_mcp: bool,
) -> Result<()> {
    println!("🔧 Starting Rustwork microservices workspace...");
    println!("📂 Workspace root: {}", workspace_root.path().display());
    println!("📦 Services directory: {}", workspace_root.services_dir().display());
    println!("🔍 Detected {} Rustwork service(s):", services.len());
    for service in &services {
        let relative_path = service
            .path
            .strip_prefix(workspace_root.path())
            .unwrap_or(&service.path);
        println!("   - {} ({})", service.name, relative_path.display());
    }
    println!();

    // Start MCP server in background only if enabled
    if enable_mcp {
        let mcp_port = 4000u16;

        println!(
            "🚀 Starting MCP server on 127.0.0.1:{}... (development only)",
            mcp_port
        );
        println!(
            "   MCP observing workspace: {}",
            workspace_root.path().display()
        );
        println!("   Press Ctrl+C to stop\n");

        // Start MCP in a separate thread to avoid Send issues with syn
        let workspace_path = workspace_root.path().to_path_buf();
        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async move {
                if let Err(e) = crate::mcp::run_server("127.0.0.1", mcp_port, workspace_path).await
                {
                    eprintln!("⚠️  MCP server error: {}", e);
                }
            });
        });
    } else {
        println!("ℹ️  MCP server disabled. Use --mcp to enable it.\n");
    }

    // Check if cargo-watch is installed
    let has_cargo_watch = Command::new("cargo")
        .args(["watch", "--version"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    if !has_cargo_watch {
        println!("⚠️  cargo-watch not found.");
        println!("   Run: cargo install cargo-watch");
        anyhow::bail!("cargo-watch is required for dev mode");
    }

    // Lancer tous les services
    let children = Arc::new(Mutex::new(Vec::new()));

    for service in &services {
        println!("▶ Starting {}...", service.name);
        match start_service_with_watch(service) {
            Ok(child) => {
                children.lock().unwrap().push(child);
            }
            Err(e) => {
                eprintln!("⚠️  Failed to start {}: {}", service.name, e);
                eprintln!("   Continuing with other services...");
            }
        }
    }

    println!("\n✅ All services started. Press Ctrl+C to stop all services.\n");

    // Attendre que tous les processus se terminent
    let children_clone = Arc::clone(&children);
    let handle = thread::spawn(move || {
        loop {
            let mut children_lock = children_clone.lock().unwrap();
            children_lock.retain_mut(|child| {
                match child.try_wait() {
                    Ok(Some(_status)) => false, // Le processus est terminé
                    Ok(None) => true,           // Toujours en cours
                    Err(_) => false,            // Erreur, retirer le processus
                }
            });

            if children_lock.is_empty() {
                break;
            }
            drop(children_lock);
            thread::sleep(std::time::Duration::from_millis(100));
        }
    });

    handle.join().unwrap();

    Ok(())
}

pub async fn execute(enable_mcp: bool, explicit_path: Option<&Path>) -> Result<()> {
    let current_dir = std::env::current_dir()?;

    // Step 1: Detect workspace root using the new robust detection logic
    let workspace_root = if let Some(explicit) = explicit_path {
        // Use explicit path if provided
        WorkspaceRoot::detect_with_explicit(&current_dir, Some(explicit))?
    } else {
        // Auto-detect from current directory
        WorkspaceRoot::detect(&current_dir)?
    };

    // Step 2: Detect all Rustwork services in the workspace
    let services = detect_rustwork_services(workspace_root.path())?;

    // Step 3: Determine execution mode based on number of services
    match services.len() {
        0 => {
            // No services found - fail explicitly
            anyhow::bail!(
                "No Rustwork services found in workspace: {}\n\
                 \n\
                 Expected structure:\n\
                 ./\n\
                 └── Backend/\n\
                     └── services/\n\
                         ├── <service1>/\n\
                         └── <service2>/\n\
                 \n\
                 A valid Rustwork service requires:\n\
                 - .rustwork/manifest.json\n\
                 - Cargo.toml\n\
                 - src/main.rs\n\
                 \n\
                 Create a new workspace with: rustwork new auth,user,session",
                workspace_root.path().display()
            );
        }
        1 => {
            // Single service mode
            run_single_service(&workspace_root, &services[0], enable_mcp).await
        }
        _ => {
            // Multi-service mode (default for microservices)
            run_multiple_services(&workspace_root, services, enable_mcp).await
        }
    }
}
