use super::types::{FileChangeEvent, FileWatcher};
use anyhow::Result;
use notify::{Event, RecursiveMode, Watcher as NotifyWatcher};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::mpsc;

impl FileWatcher {
    /// Create a new file watcher for the given project directory
    pub fn new(_project_path: &Path) -> Result<Self> {
        let (tx, rx) = mpsc::unbounded_channel();
        let tx = Arc::new(tx);

        let watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
            match res {
                Ok(event) => {
                    let tx = Arc::clone(&tx);
                    tokio::spawn(async move {
                        for path in event.paths {
                            // Only watch .rs files
                            if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                                let file_event = match event.kind {
                                    notify::EventKind::Create(_) => {
                                        FileChangeEvent::Created(path.clone())
                                    }
                                    notify::EventKind::Modify(_) => {
                                        FileChangeEvent::Modified(path.clone())
                                    }
                                    notify::EventKind::Remove(_) => {
                                        FileChangeEvent::Deleted(path.clone())
                                    }
                                    _ => continue,
                                };

                                let _ = tx.send(file_event);
                            }
                        }
                    });
                }
                Err(e) => {
                    eprintln!("File watcher error: {}", e);
                }
            }
        })?;

        Ok(Self {
            watcher,
            receiver: rx,
        })
    }

    /// Start watching the src directory
    pub fn watch(&mut self, project_path: &Path) -> Result<()> {
        let src_path = project_path.join("src");
        if src_path.exists() {
            self.watcher.watch(&src_path, RecursiveMode::Recursive)?;
            eprintln!("👁️  Watching for changes in {}", src_path.display());
        }
        Ok(())
    }

    /// Get the next file change event
    pub async fn next_event(&mut self) -> Option<FileChangeEvent> {
        self.receiver.recv().await
    }
}
