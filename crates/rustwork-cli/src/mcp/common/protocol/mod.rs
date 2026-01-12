/// JSON-RPC 2.0 protocol types and helpers
mod errors;
mod responses;
mod types;

pub use types::{RpcError, RpcRequest, RpcResponse};

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_rpc_request() {
        let json = r#"{"jsonrpc":"2.0","id":1,"method":"get_manifest","params":{}}"#;
        let req: RpcRequest = serde_json::from_str(json).unwrap();

        assert_eq!(req.jsonrpc, "2.0");
        assert_eq!(req.id, Some(json!(1)));
        assert_eq!(req.method, "get_manifest");
        assert!(req.params.is_some());
    }

    #[test]
    fn test_parse_rpc_request_no_params() {
        let json = r#"{"jsonrpc":"2.0","id":"abc-123","method":"get_routes"}"#;
        let req: RpcRequest = serde_json::from_str(json).unwrap();

        assert_eq!(req.jsonrpc, "2.0");
        assert_eq!(req.id, Some(json!("abc-123")));
        assert_eq!(req.method, "get_routes");
        assert!(req.params.is_none());
    }

    #[test]
    fn test_serialize_success_response() {
        let response = RpcResponse::success(Some(json!(1)), json!({"status": "ok"}));

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"jsonrpc\":\"2.0\""));
        assert!(json.contains("\"id\":1"));
        assert!(json.contains("\"result\""));
        assert!(!json.contains("\"error\""));
    }

    #[test]
    fn test_serialize_error_response() {
        let response =
            RpcResponse::error(Some(json!(2)), RpcError::method_not_found("unknown_method"));

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"jsonrpc\":\"2.0\""));
        assert!(json.contains("\"id\":2"));
        assert!(json.contains("\"error\""));
        assert!(json.contains("-32601"));
        assert!(!json.contains("\"result\""));
    }

    #[test]
    fn test_error_codes() {
        assert_eq!(RpcError::invalid_request("test").code, -32600);
        assert_eq!(RpcError::method_not_found("test").code, -32601);
        assert_eq!(RpcError::invalid_params("test").code, -32602);
        assert_eq!(RpcError::internal_error("test").code, -32603);
    }

    #[test]
    fn test_roundtrip_request() {
        let original = RpcRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(42)),
            method: "get_manifest".to_string(),
            params: Some(json!({"key": "value"})),
        };

        let json = serde_json::to_string(&original).unwrap();
        let parsed: RpcRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.jsonrpc, original.jsonrpc);
        assert_eq!(parsed.id, original.id);
        assert_eq!(parsed.method, original.method);
        assert_eq!(parsed.params, original.params);
    }

    #[test]
    fn test_roundtrip_response() {
        let original = RpcResponse::success(Some(json!("test-id")), json!({"data": [1, 2, 3]}));

        let json = serde_json::to_string(&original).unwrap();
        let parsed: RpcResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.jsonrpc, original.jsonrpc);
        assert_eq!(parsed.id, original.id);
        assert_eq!(parsed.result, original.result);
        assert_eq!(parsed.error, original.error);
    }
}
