/// P1 Shared Helpers - ZERO HEURISTICS
///
/// Functions used across multiple P1 handlers with conservative approach
use crate::mcp::common::confidence::{ServiceContext, ServiceScope};
use crate::mcp::common::indexer::{CodeIndex, SourceFile};
use crate::mcp::common::responses::FunctionReference;
use crate::mcp::common::state::LiveProjectState;

/// Find function in code index (EXACT match only)
pub(crate) fn find_function_in_index(
    index: &CodeIndex,
    function_name: &str,
) -> Option<(String, usize, usize)> {
    for file in index.files.values() {
        for func in &file.functions {
            if func.name == function_name
                || format!("{}::{}", file.module_path, func.name) == function_name
            {
                return Some((file.relative_path.clone(), func.start_line, func.end_line));
            }
        }
    }
    None
}

/// Get confirmed function calls (depth 1 only)
pub(crate) fn get_confirmed_calls(index: &CodeIndex, function: &str) -> Vec<FunctionReference> {
    let calls = index.get_calls(function, 1); // Only direct calls (depth 1)
    let mut result = Vec::new();

    for (called, _depth) in calls {
        if let Some((file, start, end)) = find_function_in_index(index, &called) {
            result.push(FunctionReference {
                name: called,
                file,
                start_line: start,
                end_line: end,
            });
        }
    }

    result
}

/// Determine service context from project state
pub(crate) async fn determine_service_context(state: &LiveProjectState) -> ServiceContext {
    let code_index = state.code_index.read().await;

    if code_index.projects.len() > 1 {
        // Microservices
        ServiceContext::service("multiple".to_string(), ServiceScope::InterService)
    } else if let Some(project) = code_index.projects.first() {
        if project.is_service {
            ServiceContext::service(project.name.clone(), ServiceScope::Local)
        } else {
            ServiceContext::default_service()
        }
    } else {
        ServiceContext::default_service()
    }
}

/// Determine service context from source file
pub(crate) fn determine_service_context_from_file(
    file: &SourceFile,
    _state: &LiveProjectState,
) -> ServiceContext {
    if let Some(service) = &file.service {
        ServiceContext::service(service.clone(), ServiceScope::Local)
    } else {
        // Default to micro service context if no service context
        ServiceContext::default_service()
    }
}
