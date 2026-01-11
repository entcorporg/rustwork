use super::types::LiveProjectState;
use crate::mcp::watcher::FileChangeEvent;
use anyhow::Result;

impl LiveProjectState {
    /// Handle a file change event
    pub async fn handle_file_change(&self, event: FileChangeEvent) -> Result<()> {
        match event {
            FileChangeEvent::Modified(path) | FileChangeEvent::Created(path) => {
                println!("📝 File changed: {}", path.display());

                // For now, do a full rescan
                // TODO: Implement incremental updates for better performance
                self.rescan().await?;
            }
            FileChangeEvent::Deleted(path) => {
                println!("🗑️  File deleted: {}", path.display());

                // Remove from index
                let relative_path = path
                    .strip_prefix(&self.project_path)
                    .unwrap_or(&path)
                    .to_string_lossy()
                    .to_string();

                let mut code_index = self.code_index.write().await;
                code_index.files.remove(&relative_path);
                code_index.build_call_graphs();
            }
        }
        Ok(())
    }
}
