use super::collection::DiagnosticCollection;
use super::parsers::parse_cargo_message;
use anyhow::Result;
use std::path::PathBuf;
use std::process::Stdio;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::RwLock;

/// Diagnostic collector that monitors cargo output
pub struct DiagnosticCollector {
    collection: Arc<RwLock<DiagnosticCollection>>,
    workspace_path: PathBuf,
}

impl DiagnosticCollector {
    pub fn new(workspace_path: PathBuf) -> Self {
        Self {
            collection: Arc::new(RwLock::new(DiagnosticCollection::new())),
            workspace_path,
        }
    }

    pub fn get_collection(&self) -> Arc<RwLock<DiagnosticCollection>> {
        Arc::clone(&self.collection)
    }

    /// Start collecting diagnostics from cargo check output
    pub async fn start_collecting(&self) -> Result<()> {
        let collection = Arc::clone(&self.collection);
        let workspace_path = self.workspace_path.clone();

        tokio::spawn(async move {
            loop {
                // Run cargo check with JSON output in the workspace directory
                let mut child = match Command::new("cargo")
                    .args(["check", "--message-format=json", "--all-targets"])
                    .current_dir(&workspace_path)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                {
                    Ok(child) => child,
                    Err(e) => {
                        eprintln!("Failed to spawn cargo check: {}", e);
                        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                        continue;
                    }
                };

                // Process stdout
                if let Some(stdout) = child.stdout.take() {
                    let reader = BufReader::new(stdout);
                    let mut lines = reader.lines();
                    let collection_clone = Arc::clone(&collection);

                    tokio::spawn(async move {
                        while let Ok(Some(line)) = lines.next_line().await {
                            if let Err(e) = parse_cargo_message(&line, &collection_clone).await {
                                eprintln!("Failed to parse cargo message: {}", e);
                            }
                        }
                    });
                }

                // Wait for process to complete
                let _ = child.wait().await;

                // Wait before next check
                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            }
        });

        Ok(())
    }
}
