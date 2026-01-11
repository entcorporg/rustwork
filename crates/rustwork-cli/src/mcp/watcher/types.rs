use notify::RecommendedWatcher;
use std::path::PathBuf;
use tokio::sync::mpsc;

/// File change event
#[derive(Debug, Clone)]
pub enum FileChangeEvent {
    Modified(PathBuf),
    Created(PathBuf),
    Deleted(PathBuf),
}

/// File watcher for monitoring Rust source files
pub struct FileWatcher {
    pub(super) watcher: RecommendedWatcher,
    pub(super) receiver: mpsc::UnboundedReceiver<FileChangeEvent>,
}
