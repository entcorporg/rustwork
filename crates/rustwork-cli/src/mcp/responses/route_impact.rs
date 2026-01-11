/// Response structures for get_route_impact tool
use super::function_usage::FunctionReference;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteImpactResponse {
    /// Route information
    pub route: RouteInfo,

    /// Handler function (resolved or error)
    pub handler: HandlerResolution,

    /// Called functions (CONFIRMED only, no guessing)
    pub called_functions: Vec<FunctionReference>,

    /// Affected files (CONFIRMED only)
    pub affected_files: Vec<String>,

    /// Services impacted
    pub services_impacted: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteInfo {
    pub method: String,
    pub path: String,
    pub file: String,
    pub line: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "lowercase")]
pub enum HandlerResolution {
    /// Handler resolved to a concrete function
    Resolved {
        function: String,
        file: String,
        start_line: usize,
        end_line: usize,
    },

    /// Handler found but cannot resolve to function
    Unresolved {
        handler_name: String,
        reason: String,
    },

    /// Handler not found at all
    NotFound { reason: String },
}
