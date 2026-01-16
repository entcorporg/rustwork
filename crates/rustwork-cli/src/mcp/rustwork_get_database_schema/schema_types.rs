/// Database schema representation
#[derive(Debug, Clone)]
pub struct DatabaseSchema {
    pub database_type: String,
    pub connection_info: String, // Sanitized (no passwords)
    pub tables: Vec<TableSchema>,
    pub is_complete: bool,
    pub source: String, // "introspection", "sqlx", "migrations"
}

/// Table schema
#[derive(Debug, Clone)]
pub struct TableSchema {
    pub name: String,
    pub columns: Vec<ColumnSchema>,
    pub indexes: Vec<IndexSchema>,
    pub foreign_keys: Vec<ForeignKeySchema>,
}

/// Column schema
#[derive(Debug, Clone)]
pub struct ColumnSchema {
    pub name: String,
    pub data_type: String,
    pub nullable: bool,
    pub primary_key: bool,
    pub unique: bool,
    pub default: Option<String>,
    pub max_length: Option<i32>,
}

/// Index schema
#[derive(Debug, Clone)]
pub struct IndexSchema {
    pub name: String,
    pub columns: Vec<String>,
    pub unique: bool,
}

/// Foreign key schema
#[derive(Debug, Clone)]
pub struct ForeignKeySchema {
    pub name: Option<String>,
    pub columns: Vec<String>,
    pub referenced_table: String,
    pub referenced_columns: Vec<String>,
}
