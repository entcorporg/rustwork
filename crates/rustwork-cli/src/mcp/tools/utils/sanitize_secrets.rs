use serde_json::Value;

/// Sanitize sensitive data from JSON values
#[allow(dead_code)]
pub fn sanitize_secrets(value: &mut Value) {
    match value {
        Value::Object(map) => {
            for (key, val) in map.iter_mut() {
                let key_lower = key.to_lowercase();
                if key_lower.contains("password")
                    || key_lower.contains("secret")
                    || key_lower.contains("token")
                    || key_lower.contains("key")
                    || key_lower == "jwt_secret"
                    || key_lower == "db_password"
                {
                    *val = Value::String("***".to_string());
                } else {
                    sanitize_secrets(val);
                }
            }
        }
        Value::Array(arr) => {
            for item in arr.iter_mut() {
                sanitize_secrets(item);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_sanitize_secrets() {
        let mut value = json!({
            "db_password": "secret123",
            "jwt_secret": "my-secret",
            "api_key": "key123",
            "normal_field": "public",
            "nested": {
                "password": "hidden",
                "public": "visible"
            }
        });

        sanitize_secrets(&mut value);

        assert_eq!(value["db_password"], "***");
        assert_eq!(value["jwt_secret"], "***");
        assert_eq!(value["api_key"], "***");
        assert_eq!(value["normal_field"], "public");
        assert_eq!(value["nested"]["password"], "***");
        assert_eq!(value["nested"]["public"], "visible");
    }
}
