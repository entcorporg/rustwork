use crate::mcp::common::protocol::RpcError;
use crate::mcp::common::state::LiveProjectState;
use serde_json::{json, Value};
use std::path::PathBuf;

mod aggregator;
mod metrics;
mod service_types;

/// rustwork.getServicesOverview - Get architectural overview of all services
///
/// Provides a macro view of the entire workspace architecture, including:
/// - Service identification and metadata
/// - Port assignments and configuration
/// - Route counts and gRPC services
/// - Database usage
/// - Dependencies and relationships
/// - Health metrics and LOC stats
///
/// CRITICAL: Aggregates data from other MCP tools and static analysis.
/// Does NOT execute code or invent responsibilities.
pub async fn rustwork_get_services_overview(
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

    // Discover all services
    let service_paths = discover_services(workspace_path)?;

    // Filter if requested
    let services_to_analyze: Vec<PathBuf> = if let Some(filter) = service_filter {
        service_paths
            .into_iter()
            .filter(|p| {
                p.file_name()
                    .and_then(|n| n.to_str())
                    .map(|n| n == filter)
                    .unwrap_or(false)
            })
            .collect()
    } else {
        service_paths
    };

    if services_to_analyze.is_empty() {
        return Err(RpcError::invalid_params("No services found to analyze"));
    }

    // Analyze each service
    let mut services = Vec::new();
    for service_path in services_to_analyze {
        let service_overview =
            aggregator::analyze_service(&service_path, workspace_path, state).await?;
        services.push(service_overview);
    }

    // Detect dependencies between services
    let dependencies = aggregator::detect_service_dependencies(&services).await;

    // Build response
    Ok(json!({
        "services": services.iter().map(|s| json!({
            "name": s.name,
            "path": s.path,
            "port": s.port,
            "responsibility": s.responsibility,
            "status": format!("{:?}", s.status),
            "routes_count": s.routes_count,
            "grpc_services_count": s.grpc_services_count,
            "models_count": s.models_count,
            "middleware_count": s.middleware_count,
            "tests_count": s.tests_count,
            "lines_of_code": s.lines_of_code,
            "database": s.database.as_ref().map(|db| json!({
                "type": db.db_type,
                "tables": db.tables
            })),
            "depends_on": s.depends_on,
            "called_by": s.called_by
        })).collect::<Vec<_>>(),
        "architecture": {
            "total_services": services.len(),
            "total_routes": services.iter().map(|s| s.routes_count).sum::<usize>(),
            "total_models": services.iter().map(|s| s.models_count).sum::<usize>(),
            "total_loc": services.iter().map(|s| s.lines_of_code).sum::<usize>()
        },
        "dependencies": dependencies,
        "confidence": "high",
        "context": {
            "workspace": workspace_path.display().to_string(),
            "service_filter": service_filter
        }
    }))
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

    // Also check root-level crates/ directory (like rustwork itself)
    let crates_dir = workspace_root.join("crates");
    if crates_dir.exists() && crates_dir.is_dir() {
        if let Ok(entries) = std::fs::read_dir(crates_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() && path.join("Cargo.toml").exists() {
                    services.push(path);
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
