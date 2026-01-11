#[allow(dead_code)]
pub const CARGO_TOML: &str = r#"[package]
name = "{{ project_name }}"
version = "0.1.0"
edition = "2021"

# This project is standalone, not part of rustwork workspace
[workspace]
members = ["migration"]

[dependencies]
# NOTE:
# This project depends on a local checkout of rustwork.
# When rustwork is published on crates.io, this will switch to:
# rustwork = "0.1"
rustwork = { path = "../rustwork/crates/rustwork" }
migration = { path = "migration" }
axum = "0.7"
tokio = { version = "1.40", features = ["full"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
anyhow = "1.0"
serde = { version = "1.0", features = ["derive"] }
sea-orm = { version = "1.0", features = ["sqlx-sqlite", "sqlx-postgres", "sqlx-mysql", "runtime-tokio-native-tls", "macros"] }
sea-orm-migration = { version = "1.0" }
"#;
