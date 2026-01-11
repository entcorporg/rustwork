use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn new(success: bool, data: Option<T>, message: Option<String>) -> Self {
        Self {
            success,
            data,
            message,
            error: None,
        }
    }

    pub fn with_error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            message: None,
            error: Some(error),
        }
    }
}

/// Helper: réponse success 200
pub fn ok<T: Serialize>(data: T) -> (StatusCode, Json<ApiResponse<T>>) {
    (
        StatusCode::OK,
        Json(ApiResponse::new(true, Some(data), None)),
    )
}

/// Helper: réponse created 201
pub fn created<T: Serialize>(data: T) -> (StatusCode, Json<ApiResponse<T>>) {
    (
        StatusCode::CREATED,
        Json(ApiResponse::new(
            true,
            Some(data),
            Some("Resource created".to_string()),
        )),
    )
}

/// Helper: réponse error
pub fn error<T>(status: StatusCode, message: String) -> (StatusCode, Json<ApiResponse<T>>) {
    (status, Json(ApiResponse::with_error(message)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;

    #[test]
    fn test_api_response_new_success() {
        let response: ApiResponse<String> =
            ApiResponse::new(true, Some("data".to_string()), Some("success".to_string()));
        assert!(response.success);
        assert_eq!(response.data, Some("data".to_string()));
        assert_eq!(response.message, Some("success".to_string()));
        assert!(response.error.is_none());
    }

    #[test]
    fn test_api_response_with_error() {
        let response: ApiResponse<()> = ApiResponse::with_error("error message".to_string());
        assert!(!response.success);
        assert!(response.data.is_none());
        assert!(response.message.is_none());
        assert_eq!(response.error, Some("error message".to_string()));
    }

    #[test]
    fn test_ok_helper_returns_200() {
        let (status, json) = ok("test data");
        assert_eq!(status, StatusCode::OK);
        assert!(json.0.success);
        assert_eq!(json.0.data, Some("test data"));
        assert!(json.0.message.is_none());
    }

    #[test]
    fn test_ok_helper_with_struct() {
        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
        struct TestData {
            id: i32,
            name: String,
        }
        let data = TestData {
            id: 1,
            name: "test".to_string(),
        };
        let (status, json) = ok(data.clone());
        assert_eq!(status, StatusCode::OK);
        assert_eq!(json.0.data, Some(data));
    }

    #[test]
    fn test_created_helper_returns_201() {
        let (status, json) = created("new resource");
        assert_eq!(status, StatusCode::CREATED);
        assert!(json.0.success);
        assert_eq!(json.0.data, Some("new resource"));
        assert_eq!(json.0.message, Some("Resource created".to_string()));
    }

    #[test]
    fn test_error_helper_returns_correct_status() {
        let (status, json) = error::<()>(StatusCode::BAD_REQUEST, "bad input".to_string());
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert!(!json.0.success);
        assert_eq!(json.0.error, Some("bad input".to_string()));
    }

    #[test]
    fn test_error_helper_404() {
        let (status, json) = error::<String>(StatusCode::NOT_FOUND, "not found".to_string());
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert!(!json.0.success);
        assert!(json.0.data.is_none());
    }

    #[test]
    fn test_error_helper_500() {
        let (status, json) = error::<()>(
            StatusCode::INTERNAL_SERVER_ERROR,
            "server error".to_string(),
        );
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(json.0.error, Some("server error".to_string()));
    }

    #[test]
    fn test_api_response_serialization() {
        let response: ApiResponse<i32> = ApiResponse::new(true, Some(42), None);
        let json_str = serde_json::to_string(&response).unwrap();
        assert!(json_str.contains("\"success\":true"));
        assert!(json_str.contains("\"data\":42"));
    }

    #[test]
    fn test_api_response_deserialization() {
        let json_str = r#"{"success":true,"data":"test","message":null}"#;
        let response: ApiResponse<String> = serde_json::from_str(json_str).unwrap();
        assert!(response.success);
        assert_eq!(response.data, Some("test".to_string()));
    }

    #[test]
    fn test_api_response_skip_none_error() {
        let response: ApiResponse<i32> = ApiResponse::new(true, Some(42), None);
        let json_str = serde_json::to_string(&response).unwrap();
        // L'attribut skip_serializing_if devrait omettre "error" quand None
        assert!(!json_str.contains("\"error\""));
    }

    #[test]
    fn test_ok_with_empty_vec() {
        let empty: Vec<String> = vec![];
        let (status, json) = ok(empty.clone());
        assert_eq!(status, StatusCode::OK);
        assert_eq!(json.0.data, Some(empty));
    }

    #[test]
    fn test_ok_with_nested_data() {
        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
        struct Nested {
            items: Vec<i32>,
        }
        let data = Nested {
            items: vec![1, 2, 3],
        };
        let (status, json) = ok(data.clone());
        assert_eq!(status, StatusCode::OK);
        assert_eq!(json.0.data, Some(data));
    }

    #[test]
    fn test_error_with_empty_message() {
        let (status, json) = error::<()>(StatusCode::BAD_REQUEST, "".to_string());
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(json.0.error, Some("".to_string()));
    }
}
