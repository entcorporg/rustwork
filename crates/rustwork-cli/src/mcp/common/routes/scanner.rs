use super::types::{RouteInfo, RouteRegistry};
use super::visitor::RouteVisitor;
use anyhow::{Context, Result};
use std::path::Path;
use syn::{visit::Visit, File};
use tokio::fs;

/// Analyze a single file for routes
pub async fn analyze_routes_in_file(
    file_path: &Path,
    project_root: &Path,
) -> Result<Vec<RouteInfo>> {
    let content = fs::read_to_string(file_path)
        .await
        .context(format!("Failed to read file: {}", file_path.display()))?;

    let file_path_owned = file_path.to_path_buf();
    let project_root_owned = project_root.to_path_buf();
    let content_owned = content.clone();

    // Parse in a blocking task to avoid Send issues with syn
    let routes = tokio::task::spawn_blocking(move || {
        let syntax_tree: File = syn::parse_file(&content_owned).context(format!(
            "Failed to parse file: {}",
            file_path_owned.display()
        ))?;

        let relative_path = file_path_owned
            .strip_prefix(&project_root_owned)
            .unwrap_or(&file_path_owned)
            .to_string_lossy()
            .to_string();

        let mut visitor = RouteVisitor::new(relative_path);
        visitor.visit_file(&syntax_tree);

        Ok::<_, anyhow::Error>(visitor.routes)
    })
    .await??;

    Ok(routes)
}

/// Scan project for all route definitions
pub async fn scan_routes(project_root: &Path) -> Result<RouteRegistry> {
    let mut registry = RouteRegistry::new();

    // Check for micro-services layout (services/ or Backend/services/ directory)
    let services_dir = if project_root.join("Backend/services").exists() {
        project_root.join("Backend/services")
    } else {
        project_root.join("services")
    };
    if services_dir.exists() && services_dir.is_dir() {
        // Scan each service
        let mut entries = fs::read_dir(&services_dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let service_path = entry.path();
            if service_path.is_dir() {
                let src_dir = service_path.join("src");
                if src_dir.exists() {
                    // Scan routes in this service - use project_root for relative paths
                    scan_routes_in_directory(&src_dir, project_root, &mut registry).await?;
                }
            }
        }
        return Ok(registry);
    }

    // Fallback: scan src/ directly if no services directory
    let src_dir = project_root.join("src");
    if !src_dir.exists() {
        anyhow::bail!("Source directory not found: {}", src_dir.display());
    }

    scan_routes_in_directory(&src_dir, project_root, &mut registry).await?;

    Ok(registry)
}

fn scan_routes_in_directory<'a>(
    dir: &'a Path,
    project_root: &'a Path,
    registry: &'a mut RouteRegistry,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + Send + 'a>> {
    Box::pin(async move {
        let mut entries = fs::read_dir(dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            let file_type = entry.file_type().await?;

            if file_type.is_dir() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name != "target" && !name.starts_with('.') {
                        scan_routes_in_directory(&path, project_root, registry).await?;
                    }
                }
            } else if file_type.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "rs" {
                        match analyze_routes_in_file(&path, project_root).await {
                            Ok(routes) => {
                                for route in routes {
                                    registry.add_route(route);
                                }
                            }
                            Err(e) => {
                                eprintln!(
                                    "Warning: Failed to analyze routes in {}: {}",
                                    path.display(),
                                    e
                                );
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    })
}
