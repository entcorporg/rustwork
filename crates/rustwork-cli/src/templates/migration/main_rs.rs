#[allow(dead_code)]
pub const MIGRATION_MAIN_RS: &str = r#"use sea_orm_migration::prelude::*;

#[tokio::main]
async fn main() {
    cli::run_cli(migration::Migrator).await;
}
"#;
