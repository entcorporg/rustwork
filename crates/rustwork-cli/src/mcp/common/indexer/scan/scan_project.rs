use super::super::types::{CodeIndex, ProjectInfo};
use super::{index_file, index_file_with_service};
use anyhow::Result;
use std::path::Path;
use tokio::fs;

/// Scan all Rust files in a project
pub async fn scan_project(project_root: &Path) -> Result<CodeIndex> {
    let mut index = CodeIndex::new();

    // Check if this is a microservices project (has services/ or Backend/services/ directory)
    let services_dir = if project_root.join("Backend/services").exists() {
        project_root.join("Backend/services")
    } else {
        project_root.join("services")
    };

    if services_dir.exists() && services_dir.is_dir() {
        // Microservices mode: scan each service
        let mut entries = fs::read_dir(&services_dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            let file_type = entry.file_type().await?;

            if file_type.is_dir() {
                let src_dir = path.join("src");
                if src_dir.exists() {
                    let service_name = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown")
                        .to_string();

                    println!("Scanning service: {}", service_name);

                    index.projects.push(ProjectInfo {
                        name: service_name.clone(),
                        path: path.clone(),
                        is_service: true,
                    });

                    // CRITICAL: Pass workspace root (project_root) for consistent path normalization
                    scan_directory_with_service(&src_dir, project_root, &service_name, &mut index)
                        .await?;
                }
            }
        }
    } else {
        // Fallback: scan src/ directory directly (legacy or single-service project)
        let src_dir = project_root.join("src");

        if !src_dir.exists() {
            anyhow::bail!("Source directory not found: {}", src_dir.display());
        }

        let project_name = project_root
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        index.projects.push(ProjectInfo {
            name: project_name,
            path: project_root.to_path_buf(),
            is_service: false,
        });

        scan_directory(&src_dir, project_root, &mut index).await?;
    }

    index.build_call_graphs();
    index.last_scan = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();

    Ok(index)
}

/// Scan directory with service context
fn scan_directory_with_service<'a>(
    dir: &'a Path,
    project_root: &'a Path,
    service_name: &'a str,
    index: &'a mut CodeIndex,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + Send + 'a>> {
    Box::pin(async move {
        let mut entries = fs::read_dir(dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            let file_type = entry.file_type().await?;

            if file_type.is_dir() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name != "target" && !name.starts_with('.') {
                        scan_directory_with_service(&path, project_root, service_name, index)
                            .await?;
                    }
                }
            } else if file_type.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "rs" {
                        match index_file_with_service(&path, project_root, service_name).await {
                            Ok(source_file) => {
                                index
                                    .files
                                    .insert(source_file.relative_path.clone(), source_file);
                            }
                            Err(e) => {
                                eprintln!("Warning: Failed to index {}: {}", path.display(), e);
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    })
}

/// Recursively scan a directory for Rust files
fn scan_directory<'a>(
    dir: &'a Path,
    project_root: &'a Path,
    index: &'a mut CodeIndex,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + Send + 'a>> {
    Box::pin(async move {
        let mut entries = fs::read_dir(dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            let file_type = entry.file_type().await?;

            if file_type.is_dir() {
                // Skip target and hidden directories
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name != "target" && !name.starts_with('.') {
                        scan_directory(&path, project_root, index).await?;
                    }
                }
            } else if file_type.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "rs" {
                        match index_file(&path, project_root).await {
                            Ok(source_file) => {
                                index
                                    .files
                                    .insert(source_file.relative_path.clone(), source_file);
                            }
                            Err(e) => {
                                eprintln!("Warning: Failed to index {}: {}", path.display(), e);
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    })
}
