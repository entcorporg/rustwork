#[allow(dead_code)]
pub const MIGRATION_CARGO_TOML: &str = r#"[package]
name = "migration"
version = "0.1.0"
edition = "2021"
publish = false

[lib]
name = "migration"
path = "src/lib.rs"

[[bin]]
name = "migration"
path = "src/main.rs"

[dependencies]
async-trait = "0.1"
sea-orm-migration = "1.0"
tokio = { version = "1.40", features = ["full"] }

[dependencies.sea-orm]
version = "1.0"
features = [
    "sqlx-sqlite",
    "sqlx-postgres", 
    "sqlx-mysql",
    "runtime-tokio-native-tls",
    "macros",
]
"#;
