use crate::mcp::common::protocol::RpcError;
use crate::mcp::common::state::LiveProjectState;
use crate::mcp::{
    rustwork_get_conventions, rustwork_get_database_schema, rustwork_get_diagnostics,
    rustwork_get_env_setup, rustwork_get_file_doc, rustwork_get_function_usage,
    rustwork_get_models, rustwork_get_route_impact, rustwork_get_routes,
    rustwork_get_services_overview, rustwork_grpc_detect_drift, rustwork_grpc_diff_versions,
    rustwork_grpc_get_call_graph, rustwork_grpc_get_service_status, rustwork_grpc_list_definitions,
    rustwork_grpc_test_connectivity, rustwork_grpc_validate_workspace,
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
        "rustwork_get_routes" => rustwork_get_routes::rustwork_get_routes(state).await?,
        "rustwork_get_file_doc" => {
            rustwork_get_file_doc::p1_get_file_doc(&call_params.arguments, state).await?
        }
        "rustwork_get_function_usage" => {
            rustwork_get_function_usage::p1_get_function_usage(&call_params.arguments, state)
                .await?
        }
        "rustwork_get_route_impact" => {
            rustwork_get_route_impact::p1_get_route_impact(&call_params.arguments, state).await?
        }
        "rustwork_get_diagnostics" => {
            rustwork_get_diagnostics::rustwork_get_diagnostics(state).await?
        }
        "rustwork_get_conventions" => {
            rustwork_get_conventions::rustwork_get_conventions(&call_params.arguments, state)
                .await?
        }
        "rustwork_get_env_setup" => rustwork_get_env_setup::rustwork_get_env_setup(state).await?,
        "rustwork_grpc_list_definitions" => {
            rustwork_grpc_list_definitions::rustwork_grpc_list_definitions(state).await?
        }
        "rustwork_grpc_get_service_status" => {
            rustwork_grpc_get_service_status::rustwork_grpc_get_service_status(
                &call_params.arguments,
                state,
            )
            .await?
        }
        "rustwork_grpc_test_connectivity" => {
            rustwork_grpc_test_connectivity::rustwork_grpc_test_connectivity(
                &call_params.arguments,
                state,
            )
            .await?
        }
        "rustwork_grpc_validate_workspace" => {
            rustwork_grpc_validate_workspace::rustwork_grpc_validate_workspace(state).await?
        }
        "rustwork_grpc_get_call_graph" => {
            rustwork_grpc_get_call_graph::rustwork_grpc_get_call_graph(state).await?
        }
        "rustwork_grpc_detect_drift" => {
            rustwork_grpc_detect_drift::rustwork_grpc_detect_drift(state).await?
        }
        "rustwork_grpc_diff_versions" => {
            let params: rustwork_grpc_diff_versions::types::DiffVersionsParams =
                serde_json::from_value(call_params.arguments.unwrap_or(json!({})))
                    .map_err(|e| RpcError::invalid_params(format!("Invalid parameters: {}", e)))?;
            rustwork_grpc_diff_versions::rustwork_grpc_diff_versions(state, params).await?
        }
        "rustwork_get_database_schema" => {
            rustwork_get_database_schema::rustwork_get_database_schema(
                &call_params.arguments,
                state,
            )
            .await?
        }
        "rustwork_get_models" => {
            rustwork_get_models::rustwork_get_models(&call_params.arguments, state).await?
        }
        "rustwork_get_services_overview" => {
            rustwork_get_services_overview::rustwork_get_services_overview(
                &call_params.arguments,
                state,
            )
            .await?
        }
        _ => return Err(RpcError::method_not_found(&call_params.name)),
    };

    Ok(json!({
        "content": [{
            "type": "text",
            "text": serde_json::to_string_pretty(&result).unwrap()
        }]
    }))
}
