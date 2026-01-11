use crate::mcp::protocol::RpcError;
use crate::mcp::state::LiveProjectState;
use crate::mcp::tools::rustwork::{
    p1_get_file_doc, p1_get_function_usage, p1_get_route_impact, rustwork_get_conventions,
    rustwork_get_diagnostics, rustwork_get_routes,
};
use serde::Deserialize;
use serde_json::{json, Value};

/// MCP tools/call - Execute a tool
pub async fn mcp_call_tool(
    params: &Option<Value>,
    state: Option<&LiveProjectState>,
) -> Result<Value, RpcError> {
    #[derive(Deserialize)]
    struct CallParams {
        name: String,
        arguments: Option<Value>,
    }

    let call_params: CallParams = serde_json::from_value(params.clone().unwrap_or(json!({})))
        .map_err(|e| RpcError::invalid_request(format!("Invalid params: {}", e)))?;

    let result = match call_params.name.as_str() {
        "rustwork_get_routes" => rustwork_get_routes(state).await?,
        "rustwork_get_file_doc" => p1_get_file_doc(&call_params.arguments, state).await?,
        "rustwork_get_function_usage" => {
            p1_get_function_usage(&call_params.arguments, state).await?
        }
        "rustwork_get_route_impact" => p1_get_route_impact(&call_params.arguments, state).await?,
        "rustwork_get_diagnostics" => rustwork_get_diagnostics(state).await?,
        "rustwork_get_conventions" => rustwork_get_conventions().await?,
        _ => return Err(RpcError::method_not_found(&call_params.name)),
    };

    Ok(json!({
        "content": [{
            "type": "text",
            "text": serde_json::to_string_pretty(&result).unwrap()
        }]
    }))
}
