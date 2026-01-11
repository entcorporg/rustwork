#[allow(dead_code)]
pub const DEFAULT_TOML: &str = r#"[server]
host = "0.0.0.0"
port = 3000

[database]
connection = "sqlite"
sqlite_path = "./data/app.db"

[database.pool]
max_connections = 10
min_connections = 2
connect_timeout_ms = 8000

[cors]
enabled = false
allowed_origins = []
allowed_methods = ["GET", "POST", "PUT", "PATCH", "DELETE"]
allowed_headers = ["Content-Type", "Accept"]
allow_credentials = false
max_age_seconds = 3600
"#;
