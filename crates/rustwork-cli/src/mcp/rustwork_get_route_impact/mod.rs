use crate::mcp::common::confidence::{Confidence, McpResponse};
use crate::mcp::common::indexer::CodeIndex;
use crate::mcp::common::protocol::RpcError;
use crate::mcp::common::responses::*;
use crate::mcp::common::state::LiveProjectState;
/// P1 HANDLER: get_route_impact (CONSERVATIVE)
///
/// Get route impact with CONSERVATIVE approach
/// CRITICAL: Route → Handler → Function → Service chain, stops if any step fails
use crate::mcp::common::utils::p1_helpers::{
    determine_service_context, find_function_in_index, get_confirmed_calls,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::HashSet;

/// Get route impact with CONSERVATIVE approach
///
/// Rules:
/// - Route → Handler → Function → Service
/// - If any step fails, chain stops and explains why
pub async fn p1_get_route_impact(
    params: &Option<Value>,
    state: Option<&LiveProjectState>,
) -> Result<Value, RpcError> {
    let state = state.ok_or_else(|| RpcError::internal_error("State not initialized"))?;

    #[derive(Deserialize)]
    struct Params {
        method: String,
        path: String,
    }

    let params: Params = serde_json::from_value(params.clone().unwrap_or(json!({})))
        .map_err(|e| RpcError::invalid_request(format!("Invalid params: {}", e)))?;

    let routes = state.routes.read().await;
    let code_index = state.code_index.read().await;

    // Step 1: Parse method
    let method = match params.method.to_uppercase().as_str() {
        "GET" => crate::mcp::common::routes::HttpMethod::GET,
        "POST" => crate::mcp::common::routes::HttpMethod::POST,
        "PUT" => crate::mcp::common::routes::HttpMethod::PUT,
        "PATCH" => crate::mcp::common::routes::HttpMethod::PATCH,
        "DELETE" => crate::mcp::common::routes::HttpMethod::DELETE,
        _ => {
            let error = McpError {
                code: "INVALID_HTTP_METHOD".to_string(),
                message: format!("Invalid HTTP method: {}", params.method),
                cause: Some("Method must be GET, POST, PUT, PATCH, or DELETE".to_string()),
                suggestion: Some("Use a valid HTTP method".to_string()),
            };
            let context = determine_service_context(state).await;
            let response: McpResponse<McpError> = McpResponse {
                data: error,
                confidence: Confidence::None,
                context,
            };
            return Ok(serde_json::to_value(response).unwrap());
        }
    };

    // Step 2: Find route
    let route = match routes.find_route(&method, &params.path) {
        Some(r) => r,
        None => {
            let error = McpError::route_not_found(&params.method, &params.path);
            let context = determine_service_context(state).await;
            let response: McpResponse<McpError> = McpResponse {
                data: error,
                confidence: Confidence::None,
                context,
            };
            return Ok(serde_json::to_value(response).unwrap());
        }
    };

    // Step 3: Resolve handler to concrete function
    let handler_name = route.handler_function.as_ref().unwrap_or(&route.handler);
    let handler_resolution = resolve_handler(&code_index, handler_name);

    // Step 4: Get called functions (only if handler resolved)
    let called_functions = match &handler_resolution {
        HandlerResolution::Resolved { function, .. } => get_confirmed_calls(&code_index, function),
        _ => Vec::new(), // Cannot determine calls if handler unresolved
    };

    // Step 5: Determine affected files (CONFIRMED only)
    let mut affected_files = vec![route.file.clone()];
    for func in &called_functions {
        if !affected_files.contains(&func.file) {
            affected_files.push(func.file.clone());
        }
    }

    // Step 6: Determine impacted services
    let services_impacted = determine_impacted_services_from_files(&code_index, &affected_files);

    let response_data = RouteImpactResponse {
        route: RouteInfo {
            method: format!("{:?}", route.method),
            path: route.path.clone(),
            file: route.file.clone(),
            line: route.line,
        },
        handler: handler_resolution,
        called_functions,
        affected_files,
        services_impacted,
    };

    let context = determine_service_context(state).await;

    // Confidence depends on handler resolution
    let confidence = match &response_data.handler {
        HandlerResolution::Resolved { .. } => Confidence::High,
        HandlerResolution::Unresolved { .. } => Confidence::Partial,
        HandlerResolution::NotFound { .. } => Confidence::None,
    };

    let response = McpResponse {
        data: response_data,
        confidence,
        context,
    };

    Ok(serde_json::to_value(response).unwrap())
}

fn resolve_handler(index: &CodeIndex, handler_name: &str) -> HandlerResolution {
    match find_function_in_index(index, handler_name) {
        Some((file, start, end)) => HandlerResolution::Resolved {
            function: handler_name.to_string(),
            file,
            start_line: start,
            end_line: end,
        },
        None => {
            if handler_name.is_empty() {
                HandlerResolution::NotFound {
                    reason: "No handler function specified for this route".to_string(),
                }
            } else {
                HandlerResolution::Unresolved {
                    handler_name: handler_name.to_string(),
                    reason: format!("Handler '{}' found in route but cannot be resolved to a concrete function in the indexed codebase", handler_name),
                }
            }
        }
    }
}

fn determine_impacted_services_from_files(index: &CodeIndex, files: &[String]) -> Vec<String> {
    let mut services = HashSet::new();

    for file_path in files {
        if let Some(file) = index.files.get(file_path) {
            if let Some(service) = &file.service {
                services.insert(service.clone());
            }
        }
    }

    services.into_iter().collect()
}
