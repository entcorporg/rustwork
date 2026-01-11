use anyhow::{Context, Result};
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

use crate::commands::utils::is_rustwork_project;

pub async fn execute(enable_mcp: bool) -> Result<()> {
    if !is_rustwork_project() {
        anyhow::bail!("Not in a Rustwork project. Run this command from a project created with 'rustwork new'");
    }

    println!("🔧 Starting development server with hot-reload...");
    println!("   Watching for changes in src/");

    // Start MCP server in background only if enabled
    if enable_mcp {
        let project_path = std::env::current_dir()?;
        let mcp_port = 4000u16;

        println!(
            "🚀 Starting MCP server on 127.0.0.1:{}... (development only)",
            mcp_port
        );
        println!("   Press Ctrl+C to stop\n");

        // Start MCP in a separate thread to avoid Send issues with syn
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async move {
                if let Err(e) = crate::mcp::run_server("127.0.0.1", mcp_port, project_path).await {
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
