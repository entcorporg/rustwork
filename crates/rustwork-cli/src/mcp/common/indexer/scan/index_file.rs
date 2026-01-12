use super::super::ast::CodeVisitor;
use super::super::types::SourceFile;
use anyhow::{Context, Result};
use std::path::Path;
use syn::{visit::Visit, File};
use tokio::fs;

/// Index a single Rust source file
pub async fn index_file(file_path: &Path, project_root: &Path) -> Result<SourceFile> {
    let content = fs::read_to_string(file_path)
        .await
        .context(format!("Failed to read file: {}", file_path.display()))?;

    let file_path_owned = file_path.to_path_buf();
    let project_root_owned = project_root.to_path_buf();
    let content_owned = content.clone();

    // Parse in a blocking task to avoid Send issues with syn
    let (visitor, relative_path, module_path) = tokio::task::spawn_blocking(move || {
        let syntax_tree: File = syn::parse_file(&content_owned).context(format!(
            "Failed to parse file: {}",
            file_path_owned.display()
        ))?;

        let mut visitor = CodeVisitor::new();
        visitor.visit_file(&syntax_tree);

        let relative_path = file_path_owned
            .strip_prefix(&project_root_owned)
            .unwrap_or(&file_path_owned)
            .to_string_lossy()
            .to_string()
            .replace('\\', "/"); // Normalize to POSIX format for consistency

        // Convert file path to module path (src/controllers/user.rs -> controllers::user)
        let module_path = relative_path
            .strip_prefix("src/")
            .unwrap_or(&relative_path)
            .strip_suffix(".rs")
            .unwrap_or(&relative_path)
            .replace('/', "::")
            .replace("mod.rs", "");

        Ok::<_, anyhow::Error>((visitor, relative_path, module_path))
    })
    .await??;

    let metadata = fs::metadata(file_path).await?;
    let last_modified = metadata
        .modified()?
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();

    Ok(SourceFile {
        path: file_path.to_path_buf(),
        relative_path,
        module_path,
        functions: visitor.functions,
        structs: visitor.structs,
        last_modified,
        service: None,
    })
}

/// Index a file with service context
pub async fn index_file_with_service(
    file_path: &Path,
    project_root: &Path,
    service_name: &str,
) -> Result<SourceFile> {
    let mut source_file = index_file(file_path, project_root).await?;
    source_file.service = Some(service_name.to_string());
    Ok(source_file)
}
