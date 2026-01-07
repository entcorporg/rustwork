use anyhow::{Context, Result};
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};

use crate::commands::utils::is_rustwork_project;

pub async fn execute() -> Result<()> {
    if !is_rustwork_project() {
        anyhow::bail!("Not in a Rustwork project. Run this command from a project created with 'rustwork new'");
    }

    println!("🔧 Starting development server with hot-reload...");
    println!("   Watching for changes in src/");
    println!("   Press Ctrl+C to stop\n");

    // Check if cargo-watch is installed
    let has_cargo_watch = Command::new("cargo")
        .args(&["watch", "--version"])
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
        .args(&[
            "watch",
            "-x", "run",
            "-w", "src",
            "-w", "config",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("Failed to start cargo watch")?;

    // Stream output
    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(line) = line {
                println!("{}", line);
            }
        }
    }

    let status = child.wait()?;
    
    if !status.success() {
        anyhow::bail!("Dev server exited with error");
    }

    Ok(())
}
