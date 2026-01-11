/// P1 HANDLER: get_function_usage (ZERO HEURISTICS)
///
/// Get function usage with ZERO heuristics
/// CRITICAL: Only CONFIRMED relationships, no guessing
use super::super::utils::p1_helpers::{
    determine_service_context, find_function_in_index, get_confirmed_calls,
};
use crate::mcp::confidence::{Confidence, McpResponse};
use crate::mcp::indexer::CodeIndex;
use crate::mcp::protocol::RpcError;
use crate::mcp::responses::*;
use crate::mcp::routes::RouteRegistry;
use crate::mcp::state::LiveProjectState;
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::HashSet;

/// Get function usage with ZERO heuristics
///
/// Rules:
/// - Only CONFIRMED relationships
/// - No "probably" or "maybe"
/// - If relation not proven → it doesn't appear
pub async fn p1_get_function_usage(
    params: &Option<Value>,
    state: Option<&LiveProjectState>,
) -> Result<Value, RpcError> {
    let state = state.ok_or_else(|| RpcError::internal_error("State not initialized"))?;

    #[derive(Deserialize)]
    struct Params {
        function: String,
    }

    let params: Params = serde_json::from_value(params.clone().unwrap_or(json!({})))
        .map_err(|e| RpcError::invalid_request(format!("Invalid params: {}", e)))?;

    let code_index = state.code_index.read().await;
    let routes = state.routes.read().await;

    // Step 1: Find function definition
    let function_def = find_function_in_index(&code_index, &params.function);

    if function_def.is_none() {
        // Function not found - refuse
        let error = McpError::function_not_found(&params.function);
        let context = determine_service_context(state).await;
        let response: McpResponse<McpError> = McpResponse {
            data: error,
            confidence: Confidence::None,
            context,
        };
        return Ok(serde_json::to_value(response).unwrap());
    }

    // Step 2: Get CONFIRMED callers from reverse call graph
    let callers = get_confirmed_callers(&code_index, &params.function);

    // Step 3: Get CONFIRMED calls from call graph
    let calls = get_confirmed_calls(&code_index, &params.function);

    // Step 4: Get CONFIRMED routes
    let used_by_routes = get_confirmed_routes(&routes, &params.function);

    // Step 5: Determine services impacted
    let services_impacted = determine_impacted_services(&code_index, &callers, &calls);

    let response_data = FunctionUsageResponse {
        function: params.function.clone(),
        callers,
        calls,
        used_by_routes,
        services_impacted,
    };

    let context = determine_service_context(state).await;

    // High confidence - all data from proven AST relationships
    let response = McpResponse::high_confidence(response_data, context);

    Ok(serde_json::to_value(response).unwrap())
}

fn get_confirmed_callers(index: &CodeIndex, function: &str) -> Vec<FunctionReference> {
    let callers = index.get_callers(function);
    let mut result = Vec::new();

    for caller in callers {
        if let Some((file, start, end)) = find_function_in_index(index, &caller) {
            result.push(FunctionReference {
                name: caller,
                file,
                start_line: start,
                end_line: end,
            });
        }
    }

    result
}

fn get_confirmed_routes(routes: &RouteRegistry, function: &str) -> Vec<RouteReference> {
    routes
        .get_routes_by_handler(function)
        .into_iter()
        .map(|r| RouteReference {
            method: format!("{:?}", r.method),
            path: r.path.clone(),
            handler: r.handler.clone(),
            file: r.file.clone(),
            line: r.line,
        })
        .collect()
}

fn determine_impacted_services(
    index: &CodeIndex,
    callers: &[FunctionReference],
    calls: &[FunctionReference],
) -> Vec<String> {
    let mut services = HashSet::new();

    // Check all files involved
    for caller in callers {
        if let Some(file) = index.files.get(&caller.file) {
            if let Some(service) = &file.service {
                services.insert(service.clone());
            }
        }
    }

    for call in calls {
        if let Some(file) = index.files.get(&call.file) {
            if let Some(service) = &file.service {
                services.insert(service.clone());
            }
        }
    }

    services.into_iter().collect()
}
