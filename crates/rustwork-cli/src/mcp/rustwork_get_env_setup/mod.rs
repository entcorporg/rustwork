use crate::mcp::common::protocol::RpcError;
use crate::mcp::common::state::LiveProjectState;
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

/// rustwork_get_env_setup - Analyze environment variable configuration
pub async fn rustwork_get_env_setup(state: Option<&LiveProjectState>) -> Result<Value, RpcError> {
    let state = state.ok_or_else(|| RpcError::internal_error("State not initialized"))?;

    let root_path = state.workspace_root.path();

    // Analyze environment setup
    let analysis = analyze_env_setup(root_path)?;

    Ok(analysis)
}

/// Analyze environment setup for microservices workspace
fn analyze_env_setup(root_path: &Path) -> Result<Value, RpcError> {
    let mut services = Vec::new();
    let mut all_ports: HashMap<u16, Vec<String>> = HashMap::new();

    // Find services directory (Backend/services/ or services/)
    let services_dir = if root_path.join("Backend/services").exists() {
        root_path.join("Backend/services")
    } else if root_path.join("services").exists() {
        root_path.join("services")
    } else {
        return Err(RpcError::internal_error(
            "No services directory found. Expected Backend/services/ or services/"
        ));
    };

    // Scan all services
    if let Ok(entries) = fs::read_dir(&services_dir) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                let service_name = entry.file_name().to_string_lossy().to_string();
                
                // Skip shared library
                if service_name == "shared" {
                    continue;
                }
                
                let service_analysis = analyze_service(&entry.path(), &service_name)?;

                // Collect ports
                for port in &service_analysis.ports {
                    all_ports
                        .entry(*port)
                        .or_default()
                        .push(service_name.clone());
                }

                services.push(service_analysis.to_json());
            }
        }
    }

    // Detect port conflicts
    let port_conflicts = detect_port_conflicts(&all_ports);

    // Determine overall status
    let status = determine_status(&services, &port_conflicts);

    // Generate recommendations
    let recommendations = generate_recommendations(&services, &port_conflicts);

    Ok(json!({
        "status": status,
        "confidence": "high",
        "architecture": "microservices",
        "services": services,
        "port_conflicts": port_conflicts,
        "recommendations": recommendations
    }))
}

/// Analyze a single service
fn analyze_service(service_path: &Path, service_name: &str) -> Result<ServiceAnalysis, RpcError> {
    let env_example_path = service_path.join(".env.example");
    let env_path = service_path.join(".env");
    let config_dir = service_path.join("config");

    let env_example_exists = env_example_path.exists();
    let env_exists = env_path.exists();

    // Parse .env.example to get expected variables
    let expected_vars = if env_example_exists {
        parse_env_file(&env_example_path)?
    } else {
        HashMap::new()
    };

    // Parse .env to get actual variables
    let actual_vars = if env_exists {
        parse_env_file(&env_path)?
    } else {
        HashMap::new()
    };

    // Parse config/*.toml files
    let toml_configs = parse_toml_configs(&config_dir)?;

    // Classify variables
    let mut required = Vec::new();
    let mut optional = Vec::new();
    let mut missing = Vec::new();
    let mut overridden = Vec::new();
    let mut ports = Vec::new();
    let mut config_files = Vec::new();

    for (key, value) in &expected_vars {
        // Check if it's a port variable
        if is_port_variable(key) {
            if let Some(port) = parse_port_value(value) {
                ports.push(port);
            }
        }

        // Determine if variable is present
        if actual_vars.contains_key(key) {
            required.push(key.clone());
        } else if std::env::var(key).is_ok() {
            overridden.push(key.clone());
        } else {
            // Heuristic: variables with default values are optional
            if !value.is_empty() && value != "changeme" && value != "secret" {
                optional.push(key.clone());
            } else {
                missing.push(key.clone());
            }
        }
    }

    // Also check actual .env for port variables not in .env.example
    for (key, value) in &actual_vars {
        if is_port_variable(key) {
            if let Some(port) = parse_port_value(value) {
                if !ports.contains(&port) {
                    ports.push(port);
                }
            }
        }
    }

    // Extract ports and info from TOML configs
    for toml_config in &toml_configs {
        config_files.push(toml_config.to_json());

        // Add ports from TOML to the list
        for port in &toml_config.ports {
            if !ports.contains(port) {
                ports.push(*port);
            }
        }
    }

    Ok(ServiceAnalysis {
        name: service_name.to_string(),
        path: service_path.to_string_lossy().to_string(),
        env_example_exists,
        env_exists,
        required,
        optional,
        missing,
        overridden,
        ports,
        config_files,
    })
}

/// Parse .env file and return key-value pairs (only expose non-sensitive data)
fn parse_env_file(path: &Path) -> Result<HashMap<String, String>, RpcError> {
    let content = fs::read_to_string(path)
        .map_err(|e| RpcError::internal_error(format!("Failed to read file: {}", e)))?;

    let mut vars = HashMap::new();

    for line in content.lines() {
        let line = line.trim();

        // Skip comments and empty lines
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Parse KEY=VALUE
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim().to_string();
            let value = value
                .trim()
                .trim_matches('"')
                .trim_matches('\'')
                .to_string();
            vars.insert(key, value);
        }
    }

    Ok(vars)
}

/// Check if a variable name indicates a port
fn is_port_variable(key: &str) -> bool {
    let key_upper = key.to_uppercase();
    key_upper.contains("PORT")
        || key_upper.ends_with("_PORT")
        || key_upper == "APP_PORT"
        || key_upper == "SERVICE_PORT"
}

/// Parse port value (only if numeric and non-sensitive)
fn parse_port_value(value: &str) -> Option<u16> {
    value.parse::<u16>().ok()
}

/// Parse all TOML config files in config/ directory
fn parse_toml_configs(config_dir: &Path) -> Result<Vec<TomlConfig>, RpcError> {
    let mut configs = Vec::new();

    if !config_dir.exists() || !config_dir.is_dir() {
        return Ok(configs);
    }

    let entries = fs::read_dir(config_dir)
        .map_err(|e| RpcError::internal_error(format!("Failed to read config dir: {}", e)))?;

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("toml") {
            if let Ok(config) = parse_toml_file(&path) {
                configs.push(config);
            }
        }
    }

    Ok(configs)
}

/// Parse a single TOML file and extract relevant config
fn parse_toml_file(path: &Path) -> Result<TomlConfig, RpcError> {
    let content = fs::read_to_string(path)
        .map_err(|e| RpcError::internal_error(format!("Failed to read TOML: {}", e)))?;

    let parsed: toml::Value = content
        .parse()
        .map_err(|e| RpcError::internal_error(format!("Failed to parse TOML: {}", e)))?;

    let mut ports = Vec::new();
    let mut settings = HashMap::new();

    // Recursively extract ports and settings
    extract_toml_data(&parsed, &mut ports, &mut settings, "");

    Ok(TomlConfig {
        file_name: path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string(),
        path: path.to_string_lossy().to_string(),
        ports,
        settings,
    })
}

/// Recursively extract ports and relevant settings from TOML
fn extract_toml_data(
    value: &toml::Value,
    ports: &mut Vec<u16>,
    settings: &mut HashMap<String, String>,
    prefix: &str,
) {
    match value {
        toml::Value::Table(table) => {
            for (key, val) in table {
                let full_key = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{}.{}", prefix, key)
                };

                // Check if this is a port field
                if is_port_variable(key) {
                    if let Some(port) = val.as_integer().and_then(|p| u16::try_from(p).ok()) {
                        if !ports.contains(&port) {
                            ports.push(port);
                        }
                        settings.insert(full_key.clone(), port.to_string());
                    }
                } else {
                    // Store non-sensitive settings
                    match val {
                        toml::Value::String(s) => {
                            // Only store if not a secret-like value
                            if !is_sensitive_value(s) {
                                settings.insert(full_key.clone(), s.clone());
                            }
                        }
                        toml::Value::Integer(i) => {
                            settings.insert(full_key.clone(), i.to_string());
                        }
                        toml::Value::Boolean(b) => {
                            settings.insert(full_key.clone(), b.to_string());
                        }
                        toml::Value::Table(_) | toml::Value::Array(_) => {
                            // Recurse for nested structures
                            extract_toml_data(val, ports, settings, &full_key);
                        }
                        _ => {}
                    }
                }
            }
        }
        toml::Value::Array(arr) => {
            for (i, item) in arr.iter().enumerate() {
                let full_key = format!("{}[{}]", prefix, i);
                extract_toml_data(item, ports, settings, &full_key);
            }
        }
        _ => {}
    }
}

/// Check if a value looks sensitive (should not be exposed)
fn is_sensitive_value(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("secret")
        || lower.contains("password")
        || lower.contains("token")
        || lower.contains("key")
        || lower.contains("credential")
        || value.len() > 100 // Long strings are likely secrets
}

/// Detect port conflicts
fn detect_port_conflicts(all_ports: &HashMap<u16, Vec<String>>) -> Vec<Value> {
    let mut conflicts = Vec::new();
    let used_ports: HashSet<u16> = all_ports.keys().copied().collect();

    for (port, services) in all_ports {
        if services.len() > 1 {
            // Conflict detected - suggest alternative port
            let suggested_port = find_available_port(*port, &used_ports);

            conflicts.push(json!({
                "port": port,
                "services": services,
                "suggested_port": suggested_port
            }));
        }
    }

    conflicts
}

/// Find an available port starting from the given port
fn find_available_port(start_port: u16, used_ports: &HashSet<u16>) -> u16 {
    let mut port = start_port;
    while used_ports.contains(&port) && port < 65535 {
        port += 1;
    }
    port
}

/// Determine overall status
fn determine_status(services: &[Value], port_conflicts: &[Value]) -> &'static str {
    if !port_conflicts.is_empty() {
        return "conflict_detected";
    }

    for service in services {
        if let Some(missing) = service.get("missing").and_then(|m| m.as_array()) {
            if !missing.is_empty() {
                return "action_required";
            }
        }
        if let Some(env_exists) = service.get("env").and_then(|e| e.as_bool()) {
            if !env_exists {
                return "action_required";
            }
        }
    }

    "ok"
}

/// Generate recommendations
fn generate_recommendations(services: &[Value], port_conflicts: &[Value]) -> Vec<Value> {
    let mut recommendations = Vec::new();

    // Check for missing .env files
    let mut services_without_env = Vec::new();
    for service in services {
        if let Some(env_exists) = service.get("env").and_then(|e| e.as_bool()) {
            if !env_exists {
                if let Some(name) = service.get("name").and_then(|n| n.as_str()) {
                    services_without_env.push(name.to_string());
                }
            }
        }
    }

    if !services_without_env.is_empty() {
        recommendations.push(json!({
            "action": "copy_env_example",
            "severity": "error",
            "message": format!(
                "Copy .env.example to .env for: {}",
                services_without_env.join(", ")
            ),
            "services": services_without_env
        }));
    }

    // Check for missing variables
    for service in services {
        if let Some(missing) = service.get("missing").and_then(|m| m.as_array()) {
            if !missing.is_empty() {
                if let Some(name) = service.get("name").and_then(|n| n.as_str()) {
                    recommendations.push(json!({
                        "action": "set_missing_variables",
                        "severity": "error",
                        "message": format!(
                            "Set missing variables in {}: {}",
                            name,
                            missing.iter()
                                .filter_map(|v| v.as_str())
                                .collect::<Vec<_>>()
                                .join(", ")
                        ),
                        "service": name,
                        "variables": missing
                    }));
                }
            }
        }
    }

    // Check for port conflicts
    if !port_conflicts.is_empty() {
        recommendations.push(json!({
            "action": "resolve_port_conflicts",
            "severity": "error",
            "message": "Assign unique ports to each service to avoid runtime collision",
            "conflicts": port_conflicts
        }));
    }

    // All good
    if recommendations.is_empty() {
        recommendations.push(json!({
            "action": "none",
            "severity": "info",
            "message": "Environment configuration is correct. No action required."
        }));
    }

    recommendations
}

/// Service analysis result
struct ServiceAnalysis {
    name: String,
    path: String,
    env_example_exists: bool,
    env_exists: bool,
    required: Vec<String>,
    optional: Vec<String>,
    missing: Vec<String>,
    overridden: Vec<String>,
    ports: Vec<u16>,
    config_files: Vec<Value>,
}

impl ServiceAnalysis {
    fn to_json(&self) -> Value {
        json!({
            "name": self.name,
            "path": self.path,
            "env_example": if self.env_example_exists { "present" } else { "absent" },
            "env": if self.env_exists { "present" } else { "absent" },
            "required": self.required,
            "optional": self.optional,
            "missing": self.missing,
            "overridden": self.overridden,
            "ports": self.ports,
            "config_files": self.config_files
        })
    }
}

/// TOML config file analysis
struct TomlConfig {
    file_name: String,
    path: String,
    ports: Vec<u16>,
    settings: HashMap<String, String>,
}

impl TomlConfig {
    fn to_json(&self) -> Value {
        json!({
            "file": self.file_name,
            "path": self.path,
            "ports": self.ports,
            "settings": self.settings
        })
    }
}
