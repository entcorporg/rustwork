/// Structured responses for MCP tools
///
/// CRITICAL: All responses include confidence and context
mod error;
mod file_doc;
mod function_usage;
mod route_impact;

pub use error::McpError;
pub use file_doc::{FieldDocInfo, FileDocResponse, FunctionDocInfo, ParameterInfo, StructDocInfo};
pub use function_usage::{FunctionReference, FunctionUsageResponse, RouteReference};
pub use route_impact::{HandlerResolution, RouteImpactResponse, RouteInfo};
