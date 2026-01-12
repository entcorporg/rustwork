use super::types::{FileChangeEvent, FileWatcher};
use anyhow::Result;
use notify::{Event, RecursiveMode, Watcher as NotifyWatcher};
use std::path::Path;

impl FileWatcher {
    /// Create a new file watcher for the given project directory
    pub fn new(_project_path: &Path) -> Result<Self> {
        let (tx, rx) = std::sync::mpsc::channel();

        let watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
            match res {
                Ok(event) => {
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

                            // Use non-blocking send to avoid panics if receiver is dropped
                            if tx.send(file_event).is_err() {
                                // Receiver dropped, watcher should stop
                                eprintln!(
                                    "⚠️  File watcher: receiver dropped, stopping event processing"
                                );
                                return;
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("⚠️  File watcher error: {}", e);
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

    /// Get the next file change event (blocking)
    pub fn next_event(&mut self) -> Option<FileChangeEvent> {
        self.receiver.recv().ok()
    }
}
