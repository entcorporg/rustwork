/// Response structures for get_function_usage tool
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionUsageResponse {
    /// Function name queried
    pub function: String,

    /// Functions that call this function (CONFIRMED only)
    pub callers: Vec<FunctionReference>,

    /// Functions this function calls (CONFIRMED only)
    pub calls: Vec<FunctionReference>,

    /// Routes using this function (CONFIRMED only)
    pub used_by_routes: Vec<RouteReference>,

    /// Services impacted (if microservices)
    pub services_impacted: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionReference {
    /// Full qualified name (module::function)
    pub name: String,

    /// File where defined
    pub file: String,

    /// Starting line
    pub start_line: usize,

    /// Ending line
    pub end_line: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteReference {
    pub method: String,
    pub path: String,
    pub handler: String,
    pub file: String,
    pub line: usize,
}
