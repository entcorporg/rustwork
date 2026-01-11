use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

pub async fn migrate(steps: Option<u32>) -> Result<()> {
    println!("🔄 Running database migrations...");

    // Vérifier qu'on est dans un projet Rustwork
    if !Path::new(".rustwork").exists() {
        anyhow::bail!(
            "Not in a Rustwork project directory. Run this command from the project root."
        );
    }

    // Vérifier que le crate migration existe
    if !Path::new("migration").exists() {
        anyhow::bail!(
            "Migration crate not found. This project may have been created with an older version of Rustwork.\n\
            Please create a 'migration' directory with the proper structure (see documentation)."
        );
    }

    // Charger .env
    dotenvy::dotenv().ok();

    // Charger la config pour obtenir l'URL de la DB
    let config = rustwork::AppConfig::load().context("Failed to load configuration")?;

    let url = config
        .database
        .resolved_url()
        .context("Failed to resolve database URL")?;

    let sanitized = config
        .database
        .sanitized_url()
        .unwrap_or_else(|_| "[error]".to_string());
    println!("📊 Target database: {}", sanitized);

    // Préparer la commande pour exécuter les migrations via cargo
    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("--manifest-path")
        .arg("migration/Cargo.toml")
        .arg("--")
        .arg("up");

    if let Some(n) = steps {
        cmd.arg("-n").arg(n.to_string());
    }

    // Passer l'URL de la DB via variable d'environnement
    cmd.env("DATABASE_URL", &url);

    println!("📝 Executing migrations...");

    // Exécuter la commande
    let status = cmd
        .status()
        .context("Failed to execute migration command")?;

    if !status.success() {
        anyhow::bail!("Migration failed with status: {}", status);
    }

    println!("✨ All migrations completed successfully!");

    Ok(())
}

pub async fn rollback(steps: Option<u32>) -> Result<()> {
    println!("⏮️  Rolling back migrations...");

    if !Path::new(".rustwork").exists() {
        anyhow::bail!("Not in a Rustwork project directory");
    }

    if !Path::new("migration").exists() {
        anyhow::bail!("Migration crate not found");
    }

    dotenvy::dotenv().ok();
    let config = rustwork::AppConfig::load()?;

    let url = config
        .database
        .resolved_url()
        .context("Failed to resolve database URL")?;

    let sanitized = config
        .database
        .sanitized_url()
        .unwrap_or_else(|_| "[error]".to_string());
    println!("📊 Target database: {}", sanitized);

    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("--manifest-path")
        .arg("migration/Cargo.toml")
        .arg("--")
        .arg("down");

    if let Some(n) = steps {
        cmd.arg("-n").arg(n.to_string());
    }

    cmd.env("DATABASE_URL", &url);

    println!("📝 Rolling back migrations...");

    let status = cmd.status().context("Failed to execute rollback command")?;

    if !status.success() {
        anyhow::bail!("Rollback failed with status: {}", status);
    }

    println!("✨ Rollback completed successfully!");

    Ok(())
}

pub async fn status() -> Result<()> {
    println!("📋 Migration status...");

    if !Path::new(".rustwork").exists() {
        anyhow::bail!("Not in a Rustwork project directory");
    }

    if !Path::new("migration").exists() {
        anyhow::bail!("Migration crate not found");
    }

    dotenvy::dotenv().ok();
    let config = rustwork::AppConfig::load()?;

    let url = config
        .database
        .resolved_url()
        .context("Failed to resolve database URL")?;

    let sanitized = config
        .database
        .sanitized_url()
        .unwrap_or_else(|_| "[error]".to_string());
    println!("📊 Database: {}", sanitized);

    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("--manifest-path")
        .arg("migration/Cargo.toml")
        .arg("--")
        .arg("status");

    cmd.env("DATABASE_URL", &url);

    let status = cmd.status().context("Failed to execute status command")?;

    if !status.success() {
        anyhow::bail!("Status command failed with status: {}", status);
    }

    Ok(())
}
