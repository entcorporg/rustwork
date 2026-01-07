#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check() {
        let response = crate::controllers::health::check().await;
        assert_eq!(response.0, axum::http::StatusCode::OK);
        assert!(response.1.success);
    }
}
