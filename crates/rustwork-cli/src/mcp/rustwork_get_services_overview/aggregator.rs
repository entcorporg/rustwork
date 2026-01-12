use super::service_types::*;
use crate::mcp::common::protocol::RpcError;
use crate::mcp::common::state::LiveProjectState;
use std::path::Path;

/// Analyze a single service
pub async fn analyze_service(
    service_path: &Path,
    workspace_root: &Path,
    state: &LiveProjectState,
) -> Result<ServiceOverview, RpcError> {
    let service_name = service_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    // Read basic service metadata
    let responsibility = read_service_responsibility(service_path).await;
    let port = read_service_port(service_path).await;

    // Count routes from MCP state
    let routes_count = count_routes_for_service(&service_name, state).await;

    // Count gRPC services (look for .proto or .rwk files)
    let grpc_services_count = count_grpc_services(service_path).await;

    // Count models (scan src/models or src/entities)
    let models_count = count_models(service_path).await;

    // Count middleware (scan src/middleware)
    let middleware_count = count_middleware(service_path).await;

    // Count tests
    let tests_count = count_tests(service_path).await;

    // Calculate lines of code
    let lines_of_code = calculate_loc(service_path).await;

    // Get database info
    let database = get_database_info(service_path).await;

    // Determine status (simplified - check if port is in use or from state)
    let status = ServiceStatus::Unknown;

    // Normalize path to workspace-relative
    let relative_path = service_path
        .strip_prefix(workspace_root)
        .unwrap_or(service_path);

    Ok(ServiceOverview {
        name: service_name,
        path: relative_path.display().to_string(),
        port,
        responsibility,
        status,
        routes_count,
        grpc_services_count,
        models_count,
        middleware_count,
        tests_count,
        lines_of_code,
        database,
        depends_on: Vec::new(), // Filled later by detect_service_dependencies
        called_by: Vec::new(),  // Filled later by detect_service_dependencies
    })
}

/// Read service responsibility from README.md
async fn read_service_responsibility(service_path: &Path) -> Option<String> {
    let readme_path = service_path.join("README.md");
    if !readme_path.exists() {
        return None;
    }

    if let Ok(content) = tokio::fs::read_to_string(&readme_path).await {
        // Look for common patterns
        for line in content.lines() {
            let lower = line.to_lowercase();
            if lower.contains("responsibility") || lower.contains("purpose") {
                // Extract the next line or same line after colon
                if let Some(idx) = line.find(':') {
                    let resp = line[idx + 1..].trim();
                    if !resp.is_empty() {
                        return Some(resp.to_string());
                    }
                }
            }
        }

        // Fallback: return first non-empty paragraph
        for line in content.lines().skip(1) {
            // Skip title
            let trimmed = line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with('#') {
                return Some(trimmed.to_string());
            }
        }
    }

    None
}

/// Read service port from config or .env
async fn read_service_port(service_path: &Path) -> Option<u16> {
    // Try .env files
    let env_files = vec![".env", ".env.local", ".env.development"];

    for env_file in env_files {
        let env_path = service_path.join(env_file);
        if !env_path.exists() {
            continue;
        }

        if let Ok(content) = tokio::fs::read_to_string(&env_path).await {
            for line in content.lines() {
                if line.starts_with("PORT=") || line.starts_with("HTTP_PORT=") {
                    if let Some(port_str) = line.split('=').nth(1) {
                        if let Ok(port) = port_str.trim().parse::<u16>() {
                            return Some(port);
                        }
                    }
                }
            }
        }
    }

    // Try config files
    let config_path = service_path.join("config/default.toml");
    if config_path.exists() {
        if let Ok(content) = tokio::fs::read_to_string(&config_path).await {
            for line in content.lines() {
                if line.contains("port") {
                    if let Some(port_str) = line.split('=').nth(1) {
                        let cleaned = port_str.trim().trim_matches('"');
                        if let Ok(port) = cleaned.parse::<u16>() {
                            return Some(port);
                        }
                    }
                }
            }
        }
    }

    None
}

/// Count routes for a service from MCP state
async fn count_routes_for_service(_service_name: &str, state: &LiveProjectState) -> usize {
    let routes_registry = state.routes.read().await;

    // In microservices, each service tracks its own routes
    // For now, return total routes (would need service-specific filtering)
    routes_registry.routes.len()
}

/// Count gRPC services
async fn count_grpc_services(service_path: &Path) -> usize {
    let mut count = 0;

    // Look for .proto files
    let proto_dir = service_path.join("proto");
    if proto_dir.exists() {
        if let Ok(entries) = std::fs::read_dir(proto_dir) {
            count += entries
                .filter_map(|e| {
                    let entry = e.ok()?;
                    let path = entry.path();
                    let ext = path.extension()?.to_str()?;
                    if ext == "proto" {
                        Some(())
                    } else {
                        None
                    }
                })
                .count();
        }
    }

    // Look for .rwk files (Rustwork gRPC DSL)
    let grpc_dir = service_path.join("grpc");
    if grpc_dir.exists() {
        if let Ok(entries) = std::fs::read_dir(grpc_dir) {
            count += entries
                .filter_map(|e| {
                    let entry = e.ok()?;
                    let path = entry.path();
                    let ext = path.extension()?.to_str()?;
                    if ext == "rwk" {
                        Some(())
                    } else {
                        None
                    }
                })
                .count();
        }
    }

    count
}

/// Count models
async fn count_models(service_path: &Path) -> usize {
    let mut count = 0;

    let models_dir = service_path.join("src/models");
    if models_dir.exists() {
        count += count_rust_files(&models_dir).await;
    }

    let entities_dir = service_path.join("src/entities");
    if entities_dir.exists() {
        count += count_rust_files(&entities_dir).await;
    }

    count
}

/// Count middleware
async fn count_middleware(service_path: &Path) -> usize {
    let middleware_dir = service_path.join("src/middleware");
    if !middleware_dir.exists() {
        return 0;
    }

    count_rust_files(&middleware_dir).await
}

/// Count tests
async fn count_tests(service_path: &Path) -> usize {
    let mut count = 0;

    // Unit tests in tests/ directory
    let tests_dir = service_path.join("tests");
    if tests_dir.exists() {
        count += count_rust_files(&tests_dir).await;
    }

    // Integration tests (could be in src/tests or inline)
    let src_tests_dir = service_path.join("src/tests");
    if src_tests_dir.exists() {
        count += count_rust_files(&src_tests_dir).await;
    }

    count
}

/// Count Rust files recursively
fn count_rust_files_sync(dir: &Path) -> usize {
    let mut count = 0;

    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("rs") {
                count += 1;
            } else if path.is_dir() {
                count += count_rust_files_sync(&path);
            }
        }
    }

    count
}

/// Count Rust files recursively (async wrapper)
async fn count_rust_files(dir: &Path) -> usize {
    let dir = dir.to_path_buf();
    tokio::task::spawn_blocking(move || count_rust_files_sync(&dir))
        .await
        .unwrap_or(0)
}

/// Calculate lines of code
async fn calculate_loc(service_path: &Path) -> usize {
    let src_dir = service_path.join("src");
    if !src_dir.exists() {
        return 0;
    }

    count_lines_recursive(&src_dir).await
}

/// Count lines recursively (synchronous)
fn count_lines_recursive_sync(dir: &Path) -> usize {
    let mut total = 0;

    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("rs") {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    total += content.lines().count();
                }
            } else if path.is_dir() {
                total += count_lines_recursive_sync(&path);
            }
        }
    }

    total
}

/// Count lines recursively (async wrapper)
async fn count_lines_recursive(dir: &Path) -> usize {
    let dir = dir.to_path_buf();
    tokio::task::spawn_blocking(move || count_lines_recursive_sync(&dir))
        .await
        .unwrap_or(0)
}

/// Get database info for a service
async fn get_database_info(service_path: &Path) -> Option<DatabaseInfo> {
    // Try to detect database from .env
    let env_files = vec![".env", ".env.local"];

    for env_file in env_files {
        let env_path = service_path.join(env_file);
        if !env_path.exists() {
            continue;
        }

        if let Ok(content) = tokio::fs::read_to_string(&env_path).await {
            for line in content.lines() {
                if line.starts_with("DATABASE_URL=") {
                    let url = line.split('=').nth(1).unwrap_or("").trim();
                    let url = url.trim_matches('"');

                    let db_type = if url.starts_with("sqlite:") {
                        "sqlite"
                    } else if url.starts_with("postgres") {
                        "postgres"
                    } else if url.starts_with("mysql") {
                        "mysql"
                    } else {
                        "unknown"
                    };

                    // Get table names (would need to call get_database_schema tool, but avoid circular dependency)
                    let tables = Vec::new();

                    return Some(DatabaseInfo {
                        db_type: db_type.to_string(),
                        tables,
                    });
                }
            }
        }
    }

    None
}

/// Detect dependencies between services
pub async fn detect_service_dependencies(_services: &[ServiceOverview]) -> Vec<ServiceDependency> {
    // For now, return empty - full implementation would:
    // 1. Parse HTTP client calls between services
    // 2. Parse gRPC client definitions
    // 3. Check shared database usage
    // This requires deeper code analysis and is marked as TODO for P1

    Vec::new()
}
