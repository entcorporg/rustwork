use super::types::LiveProjectState;
use crate::mcp::common::indexer::IndexState;
use crate::mcp::common::watcher::FileChangeEvent;
use anyhow::Result;

impl LiveProjectState {
    /// Handle a file change event
    pub async fn handle_file_change(&self, event: FileChangeEvent) -> Result<()> {
        match event {
            FileChangeEvent::Modified(path) | FileChangeEvent::Created(path) => {
                println!("📝 File changed: {}", path.display());

                // P0: Invalider l'index avant rescan
                {
                    let mut index_state = self.index_state.write().await;
                    *index_state = IndexState::Invalidated;
                }

                // For now, do a full rescan
                // TODO: Implement incremental updates for better performance
                self.rescan().await?;
            }
            FileChangeEvent::Deleted(path) => {
                println!("🗑️  File deleted: {}", path.display());

                // P0: Invalider l'index
                {
                    let mut index_state = self.index_state.write().await;
                    *index_state = IndexState::Invalidated;
                }

                // Remove from index
                let relative_path = path
                    .strip_prefix(&self.project_path)
                    .unwrap_or(&path)
                    .to_string_lossy()
                    .to_string();

                let mut code_index = self.code_index.write().await;
                code_index.files.remove(&relative_path);
                code_index.build_call_graphs();

                // P0: Remettre l'index à READY après suppression
                {
                    let mut index_state = self.index_state.write().await;
                    *index_state = IndexState::Ready;
                }
            }
        }
        Ok(())
    }
}
