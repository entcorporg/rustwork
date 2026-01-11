#[allow(dead_code)]
pub const GITIGNORE: &str = r#"# Rust
/target
Cargo.lock

# Environment
.env

# SQLite Database
/data/*.db
/data/*.db-shm
/data/*.db-wal

# IDE
.idea/
*.swp
*.swo

# Rustwork
.rustwork/
"#;
