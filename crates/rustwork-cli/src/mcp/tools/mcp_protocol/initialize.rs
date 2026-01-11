use crate::mcp::protocol::RpcError;
use serde_json::{json, Value};

/// MCP initialize - Required by protocol
pub async fn mcp_initialize(_params: &Option<Value>) -> Result<Value, RpcError> {
    Ok(json!({
        "protocolVersion": "2024-11-05",
        "capabilities": {
            "tools": {}
        },
        "serverInfo": {
            "name": "rustwork",
            "version": "0.1.0"
        }
    }))
}
