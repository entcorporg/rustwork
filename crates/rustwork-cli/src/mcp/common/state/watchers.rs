use super::types::LiveProjectState;
use crate::mcp::common::diagnostics::DiagnosticCollector;
use crate::mcp::common::watcher::FileWatcher;
use anyhow::Result;
use std::sync::Arc;

impl LiveProjectState {
    /// Start the file watcher
    pub async fn start_watching(&self) -> Result<()> {
        let mut watcher = FileWatcher::new(&self.project_path)?;
        watcher.watch(&self.project_path)?;

        let state = self.clone();

        // Spawn blocking task for the watcher loop
        tokio::task::spawn_blocking(move || {
            // Create a dedicated runtime for async operations in this thread
            let rt = match tokio::runtime::Runtime::new() {
                Ok(rt) => rt,
                Err(e) => {
                    eprintln!("❌ Failed to create watcher runtime: {}", e);
                    return;
                }
            };

            // next_event is blocking, so we call it directly in this blocking task
            while let Some(event) = watcher.next_event() {
                // Handle the event asynchronously using the runtime
                if let Err(e) = rt.block_on(state.handle_file_change(event)) {
                    eprintln!("⚠️  Error handling file change: {}", e);
                }
            }

            eprintln!("📤 File watcher: channel closed, stopping");
        });

        Ok(())
    }

    /// Start the diagnostic collector
    pub async fn start_diagnostics_collector(&self) -> Result<()> {
        // Use cargo_workspace_dir() instead of path() to execute cargo check
        // in the correct directory (Backend/ for microservices)
        let collector = DiagnosticCollector::new(self.workspace_root.cargo_workspace_dir());
        let collection = collector.get_collection();

        // Share the collection with our state
        {
            let mut diagnostics = self.diagnostics.write().await;
            *diagnostics = collection.read().await.clone();
        }

        // Sync periodically
        let state_diagnostics = Arc::clone(&self.diagnostics);
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                let current = collection.read().await.clone();
                let mut state_diags = state_diagnostics.write().await;
                *state_diags = current;
            }
        });

        collector.start_collecting().await?;
        Ok(())
    }
}
