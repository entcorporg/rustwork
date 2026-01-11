#[allow(dead_code)]
pub const DEV_TOML: &str = r#"[server]
host = "127.0.0.1"
port = 3001

[database]
connection = "sqlite"
sqlite_path = "./data/app_dev.db"

[database.pool]
max_connections = 5
min_connections = 1
"#;
