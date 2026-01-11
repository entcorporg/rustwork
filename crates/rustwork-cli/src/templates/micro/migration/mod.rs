pub const MIGRATION_RS: &str = r#"use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Implement your migration here
        manager
            .create_table(
                Table::create()
                    .table({{ table_name }}::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new({{ table_name }}::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table({{ table_name }}::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum {{ table_name }} {
    Table,
    Id,
}
"#;

pub const MIGRATION_CARGO_TOML: &str = r#"[package]
name = "migration"
version = "0.1.0"
edition = "2021"

[lib]
name = "migration"
path = "src/lib.rs"

[[bin]]
name = "migration"
path = "src/main.rs"

[dependencies]
async-trait = "0.1"
sea-orm-migration = { version = "1.0", features = ["runtime-tokio-native-tls", "sqlx-sqlite", "sqlx-postgres", "sqlx-mysql"] }
"#;

pub const MIGRATION_LIB_RS: &str = r#"pub use sea_orm_migration::prelude::*;

mod m20240101_000001_create_migrations_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240101_000001_create_migrations_table::Migration),
        ]
    }
}
"#;

pub const MIGRATION_INITIAL: &str = r#"use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Initial migration - customize as needed
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
"#;

pub const MIGRATION_MAIN_RS: &str = r#"use sea_orm_migration::prelude::*;

#[tokio::main]
async fn main() {
    cli::run_cli(migration::Migrator).await;
}
"#;
