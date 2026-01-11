#[allow(dead_code)]
pub const MIGRATION_RS: &str = r#"use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table({{ struct_name }}::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new({{ struct_name }}::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new({{ struct_name }}::Name).string().not_null())
                    .col(
                        ColumnDef::new({{ struct_name }}::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new({{ struct_name }}::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table({{ struct_name }}::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum {{ struct_name }} {
    Table,
    Id,
    Name,
    CreatedAt,
    UpdatedAt,
}
"#;
