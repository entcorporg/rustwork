use super::types::LiveProjectState;
use crate::mcp::diagnostics::DiagnosticCollector;
use crate::mcp::watcher::FileWatcher;
use anyhow::Result;
use std::sync::Arc;

impl LiveProjectState {
    /// Start the file watcher
    pub async fn start_watching(&self) -> Result<()> {
        let mut watcher = FileWatcher::new(&self.project_path)?;
        watcher.watch(&self.project_path)?;

        let state = self.clone();

        // Use std::thread to avoid Send issues with syn parsing
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async move {
                while let Some(event) = watcher.next_event().await {
                    if let Err(e) = state.handle_file_change(event).await {
                        eprintln!("Error handling file change: {}", e);
                    }
                }
            });
        });

        Ok(())
    }

    /// Start the diagnostic collector
    pub async fn start_diagnostics_collector(&self) -> Result<()> {
        let collector = DiagnosticCollector::new();
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
