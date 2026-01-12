use crate::mcp::common::protocol::RpcError;
use crate::mcp::common::state::LiveProjectState;
use serde_json::{json, Value};
use std::path::PathBuf;

mod introspection;
mod schema_types;

/// rustwork.getDatabaseSchema - Get database schema for a service
///
/// Introspects the actual database structure (tables, columns, indexes, foreign keys)
/// from the real database. Supports SQLite (primary), PostgreSQL, MySQL (extensible).
///
/// CRITICAL: Returns exact data or fails explicitly. No guessing, no silent fallbacks.
pub async fn rustwork_get_database_schema(
    params: &Option<Value>,
    state: Option<&LiveProjectState>,
) -> Result<Value, RpcError> {
    let state = state.ok_or_else(|| RpcError::internal_error("State not initialized"))?;

    // Parse parameters
    let service_name = params
        .as_ref()
        .and_then(|p| p.get("service"))
        .and_then(|s| s.as_str())
        .ok_or_else(|| RpcError::invalid_params("Missing 'service' parameter"))?;

    // Resolve service path
    let workspace_path = state.workspace_root.path();

    let service_path = resolve_service_path(workspace_path, service_name)?;

    // Determine database type and path from config/env
    let db_config = introspection::detect_database_config(&service_path).await?;

    // Introspect the actual database--
    let schema = introspection::introspect_database(&db_config).await?;

    // Build response
    Ok(json!({
        "service": service_name,
        "database_type": schema.database_type,
        "connection_info": schema.connection_info,
        "tables": schema.tables.iter().map(|t| json!({
            "name": t.name,
            "columns": t.columns.iter().map(|c| json!({
                "name": c.name,
                "type": c.data_type,
                "nullable": c.nullable,
                "primary_key": c.primary_key,
                "unique": c.unique,
                "default": c.default,
                "max_length": c.max_length
            })).collect::<Vec<_>>(),
            "indexes": t.indexes.iter().map(|i| json!({
                "name": i.name,
                "columns": i.columns,
                "unique": i.unique
            })).collect::<Vec<_>>(),
            "foreign_keys": t.foreign_keys.iter().map(|fk| json!({
                "name": fk.name,
                "columns": fk.columns,
                "referenced_table": fk.referenced_table,
                "referenced_columns": fk.referenced_columns
            })).collect::<Vec<_>>()
        })).collect::<Vec<_>>(),
        "confidence": if schema.is_complete { "high" } else { "medium" },
        "source": schema.source,
        "context": {
            "service": service_name,
            "workspace": workspace_path.display().to_string()
        }
    }))
}

/// Resolve service path from workspace root
fn resolve_service_path(
    workspace_root: &std::path::Path,
    service_name: &str,
) -> Result<PathBuf, RpcError> {
    // Try common locations (microservices)
    let candidates = vec![
        // New structure
        workspace_root.join("Backend/services").join(service_name),
        // Legacy structure
        workspace_root.join("services").join(service_name),
        // Direct child
        workspace_root.join(service_name),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_service_path_validation() {
        // Test that resolve_service_path handles non-existent services correctly
        let temp_dir = std::env::temp_dir();
        let result = resolve_service_path(&temp_dir, "nonexistent_service");
        assert!(result.is_err());
    }
}
