use crate::mcp::common::protocol::RpcError;
use crate::mcp::common::state::LiveProjectState;
use serde_json::{json, Value};
use std::path::PathBuf;

mod model_types;
mod parser;

/// rustwork.getModels - Get all Rust models/DTOs in the project
///
/// Parses Rust source code to identify structs used as models or DTOs.
/// Includes domain entities, request/response types, and data transfer objects.
///
/// CRITICAL: Returns exact data from source code analysis. No runtime inspection,
/// no database inference, no guessing.
pub async fn rustwork_get_models(
    params: &Option<Value>,
    state: Option<&LiveProjectState>,
) -> Result<Value, RpcError> {
    let state = state.ok_or_else(|| RpcError::internal_error("State not initialized"))?;

    // Parse parameters (optional service filter)
    let service_filter = params
        .as_ref()
        .and_then(|p| p.get("service"))
        .and_then(|s| s.as_str());

    // Get workspace root
    let workspace_path = state.workspace_root.path();

    // Determine services to scan
    let services = if let Some(service) = service_filter {
        vec![resolve_service_path(workspace_path, service)?]
    } else {
        discover_services(workspace_path)?
    };

    // Parse models from all services
    let mut all_models = Vec::new();
    for service_path in &services {
        let service_name = service_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");

        let models = parser::parse_service_models(service_path, service_name).await?;
        all_models.extend(models);
    }

    // Build response
    Ok(json!({
        "models": all_models.iter().map(|m| json!({
            "name": m.name,
            "service": m.service,
            "file": m.file_path,
            "line": m.line,
            "model_type": format!("{:?}", m.model_type),
            "fields": m.fields.iter().map(|f| json!({
                "name": f.name,
                "rust_type": f.rust_type,
                "nullable": f.nullable,
                "attributes": f.attributes
            })).collect::<Vec<_>>(),
            "derives": m.derives,
            "relations": m.relations.iter().map(|r| json!({
                "relation_type": format!("{:?}", r.relation_type),
                "target_entity": r.target_entity,
                "field_name": r.field_name
            })).collect::<Vec<_>>(),
            "is_public": m.is_public
        })).collect::<Vec<_>>(),
        "count": all_models.len(),
        "services_scanned": if service_filter.is_some() { 1 } else { services.len() },
        "confidence": "high",
        "context": {
            "workspace": workspace_path.display().to_string(),
            "service_filter": service_filter
        }
    }))
}

/// Resolve service path from workspace root
fn resolve_service_path(
    workspace_root: &std::path::Path,
    service_name: &str,
) -> Result<PathBuf, RpcError> {
    let candidates = vec![
        workspace_root.join(service_name),
        workspace_root.join("services").join(service_name),
        workspace_root.to_path_buf(),
    ];

    for path in candidates {
        if path.exists() && path.join("Cargo.toml").exists() {
            return Ok(path);
        }
    }

    Err(RpcError::invalid_params(format!(
        "Service '{}' not found in workspace",
        service_name
    )))
}

/// Discover all services in workspace
fn discover_services(workspace_root: &std::path::Path) -> Result<Vec<PathBuf>, RpcError> {
    let mut services = Vec::new();

    // Check for Backend/services/ structure (new)
    let backend_services_dir = workspace_root.join("Backend/services");
    if backend_services_dir.exists() && backend_services_dir.is_dir() {
        if let Ok(entries) = std::fs::read_dir(&backend_services_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                // Skip shared library
                if path.file_name().map(|n| n == "shared").unwrap_or(false) {
                    continue;
                }
                if path.is_dir() && path.join("Cargo.toml").exists() {
                    services.push(path);
                }
            }
        }
    }

    // Check for services/ structure (legacy)
    if services.is_empty() {
        let services_dir = workspace_root.join("services");
        if services_dir.exists() && services_dir.is_dir() {
            if let Ok(entries) = std::fs::read_dir(&services_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    // Skip shared library
                    if path.file_name().map(|n| n == "shared").unwrap_or(false) {
                        continue;
                    }
                    if path.is_dir() && path.join("Cargo.toml").exists() {
                        services.push(path);
                    }
                }
            }
        }
    }

    if services.is_empty() {
        return Err(RpcError::internal_error("No services found in workspace"));
    }

    Ok(services)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discover_services_validation() {
        let temp_dir = std::env::temp_dir();
        let result = discover_services(&temp_dir);
        assert!(result.is_err());
    }
}
