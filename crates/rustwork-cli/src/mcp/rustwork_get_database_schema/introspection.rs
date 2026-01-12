use super::schema_types::*;
use crate::mcp::common::protocol::RpcError;
use sea_orm::{ConnectionTrait, ConnectOptions, Database, DatabaseBackend, DatabaseConnection, Statement};
use std::path::Path;
use std::time::Duration;

/// Database configuration
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub db_type: DatabaseType,
    pub connection_string: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DatabaseType {
    Sqlite,
    Postgres,
    Mysql,
}

/// Detect database configuration from service files
pub async fn detect_database_config(service_path: &Path) -> Result<DatabaseConfig, RpcError> {
    // Priority 1: Read from .env or config
    if let Ok(config) = read_env_database_config(service_path).await {
        return Ok(config);
    }

    // Priority 2: Look for SQLite database files
    if let Ok(config) = detect_sqlite_file(service_path).await {
        return Ok(config);
    }

    Err(RpcError::internal_error(format!(
        "Could not detect database configuration in service '{}'",
        service_path.display()
    )))
}

/// Read database config from .env files
async fn read_env_database_config(service_path: &Path) -> Result<DatabaseConfig, RpcError> {
    let env_files = vec![".env", ".env.local", ".env.development"];

    for env_file in env_files {
        let env_path = service_path.join(env_file);
        if !env_path.exists() {
            continue;
        }

        let content = tokio::fs::read_to_string(&env_path)
            .await
            .map_err(|e| RpcError::internal_error(format!("Failed to read {}: {}", env_file, e)))?;

        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("DATABASE_URL=") || line.starts_with("DB_URL=") {
                let url = line.split('=').nth(1).unwrap_or("").trim();
                let url = url.trim_matches('"').trim_matches('\'');

                return parse_database_url(url);
            }
        }
    }

    Err(RpcError::internal_error(
        "No DATABASE_URL found in .env files",
    ))
}

/// Parse database URL to determine type
fn parse_database_url(url: &str) -> Result<DatabaseConfig, RpcError> {
    let db_type = if url.starts_with("sqlite:") || url.ends_with(".db") || url.ends_with(".sqlite")
    {
        DatabaseType::Sqlite
    } else if url.starts_with("postgres:") || url.starts_with("postgresql:") {
        DatabaseType::Postgres
    } else if url.starts_with("mysql:") {
        DatabaseType::Mysql
    } else {
        return Err(RpcError::internal_error(format!(
            "Unsupported database URL format: {}",
            url
        )));
    };

    Ok(DatabaseConfig {
        db_type,
        connection_string: url.to_string(),
    })
}

/// Detect SQLite database file in service directory
async fn detect_sqlite_file(service_path: &Path) -> Result<DatabaseConfig, RpcError> {
    let common_names = vec!["db.sqlite", "database.sqlite", "app.db", "data.db"];

    for name in common_names {
        let db_path = service_path.join(name);
        if db_path.exists() {
            return Ok(DatabaseConfig {
                db_type: DatabaseType::Sqlite,
                connection_string: format!("sqlite://{}?mode=ro", db_path.display()),
            });
        }
    }

    Err(RpcError::internal_error("No SQLite database file found"))
}

/// Introspect actual database structure
pub async fn introspect_database(config: &DatabaseConfig) -> Result<DatabaseSchema, RpcError> {
    match config.db_type {
        DatabaseType::Sqlite => introspect_sqlite(config).await,
        DatabaseType::Postgres => introspect_postgres(config).await,
        DatabaseType::Mysql => introspect_mysql(config).await,
    }
}

/// Introspect SQLite database
async fn introspect_sqlite(config: &DatabaseConfig) -> Result<DatabaseSchema, RpcError> {
    let mut opts = ConnectOptions::new(&config.connection_string);
    opts.max_connections(1)
        .connect_timeout(Duration::from_secs(5))
        .sqlx_logging(false);

    let db = Database::connect(opts)
        .await
        .map_err(|e| RpcError::internal_error(format!("Failed to connect to SQLite: {}", e)))?;

    let tables = get_sqlite_tables(&db).await?;

    let sanitized_url = sanitize_connection_string(&config.connection_string);

    Ok(DatabaseSchema {
        database_type: "sqlite".to_string(),
        connection_info: sanitized_url,
        tables,
        is_complete: true,
        source: "introspection".to_string(),
    })
}

/// Get all tables from SQLite
async fn get_sqlite_tables(db: &DatabaseConnection) -> Result<Vec<TableSchema>, RpcError> {
    // Query sqlite_master for all tables
    let query = Statement::from_string(
        DatabaseBackend::Sqlite,
        "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' ORDER BY name"
            .to_string(),
    );

    let result = db
        .query_all(query)
        .await
        .map_err(|e| RpcError::internal_error(format!("Failed to query tables: {}", e)))?;

    let mut tables = Vec::new();

    for row in result {
        let table_name: String = row.try_get("", "name").map_err(|e| {
            RpcError::internal_error(format!("Failed to get table name: {}", e))
        })?;

        let columns = get_sqlite_columns(db, &table_name).await?;
        let indexes = get_sqlite_indexes(db, &table_name).await?;
        let foreign_keys = get_sqlite_foreign_keys(db, &table_name).await?;

        tables.push(TableSchema {
            name: table_name,
            columns,
            indexes,
            foreign_keys,
        });
    }

    Ok(tables)
}

/// Get columns for a SQLite table
async fn get_sqlite_columns(
    db: &DatabaseConnection,
    table_name: &str,
) -> Result<Vec<ColumnSchema>, RpcError> {
    let query = Statement::from_string(
        DatabaseBackend::Sqlite,
        format!("PRAGMA table_info('{}')", table_name),
    );

    let result = db
        .query_all(query)
        .await
        .map_err(|e| RpcError::internal_error(format!("Failed to query columns: {}", e)))?;

    let mut columns = Vec::new();

    for row in result {
        let name: String = row
            .try_get("", "name")
            .map_err(|e| RpcError::internal_error(format!("Failed to get column name: {}", e)))?;

        let data_type: String = row.try_get("", "type").map_err(|e| {
            RpcError::internal_error(format!("Failed to get column type: {}", e))
        })?;

        let not_null: i32 = row.try_get("", "notnull").unwrap_or(0);
        let pk: i32 = row.try_get("", "pk").unwrap_or(0);
        let default_value: Option<String> = row.try_get("", "dflt_value").ok();

        columns.push(ColumnSchema {
            name,
            data_type: data_type.to_uppercase(),
            nullable: not_null == 0,
            primary_key: pk > 0,
            unique: false, // Will be determined from indexes
            default: default_value,
            max_length: None, // SQLite doesn't enforce length
        });
    }

    Ok(columns)
}

/// Get indexes for a SQLite table
async fn get_sqlite_indexes(
    db: &DatabaseConnection,
    table_name: &str,
) -> Result<Vec<IndexSchema>, RpcError> {
    let query = Statement::from_string(
        DatabaseBackend::Sqlite,
        format!("PRAGMA index_list('{}')", table_name),
    );

    let result = db.query_all(query).await.map_err(|e| {
        RpcError::internal_error(format!("Failed to query indexes: {}", e))
    })?;

    let mut indexes = Vec::new();

    for row in result {
        let index_name: String = row.try_get("", "name").map_err(|e| {
            RpcError::internal_error(format!("Failed to get index name: {}", e))
        })?;

        let unique: i32 = row.try_get("", "unique").unwrap_or(0);

        // Get columns in this index
        let columns_query = Statement::from_string(
            DatabaseBackend::Sqlite,
            format!("PRAGMA index_info('{}')", index_name),
        );

        let columns_result = db.query_all(columns_query).await.map_err(|e| {
            RpcError::internal_error(format!("Failed to query index columns: {}", e))
        })?;

        let columns: Vec<String> = columns_result
            .iter()
            .filter_map(|col_row| col_row.try_get::<String>("", "name").ok())
            .collect();

        indexes.push(IndexSchema {
            name: index_name,
            columns,
            unique: unique > 0,
        });
    }

    Ok(indexes)
}

/// Get foreign keys for a SQLite table
async fn get_sqlite_foreign_keys(
    db: &DatabaseConnection,
    table_name: &str,
) -> Result<Vec<ForeignKeySchema>, RpcError> {
    let query = Statement::from_string(
        DatabaseBackend::Sqlite,
        format!("PRAGMA foreign_key_list('{}')", table_name),
    );

    let result = db.query_all(query).await.map_err(|e| {
        RpcError::internal_error(format!("Failed to query foreign keys: {}", e))
    })?;

    let mut foreign_keys = Vec::new();

    for row in result {
        let referenced_table: String = row.try_get("", "table").map_err(|e| {
            RpcError::internal_error(format!("Failed to get referenced table: {}", e))
        })?;

        let from_col: String = row.try_get("", "from").map_err(|e| {
            RpcError::internal_error(format!("Failed to get from column: {}", e))
        })?;

        let to_col: String = row.try_get("", "to").map_err(|e| {
            RpcError::internal_error(format!("Failed to get to column: {}", e))
        })?;

        foreign_keys.push(ForeignKeySchema {
            name: None,
            columns: vec![from_col],
            referenced_table,
            referenced_columns: vec![to_col],
        });
    }

    Ok(foreign_keys)
}

/// Introspect PostgreSQL database
async fn introspect_postgres(_config: &DatabaseConfig) -> Result<DatabaseSchema, RpcError> {
    // TODO: Implement PostgreSQL introspection
    Err(RpcError::internal_error(
        "PostgreSQL introspection not yet implemented",
    ))
}

/// Introspect MySQL database
async fn introspect_mysql(_config: &DatabaseConfig) -> Result<DatabaseSchema, RpcError> {
    // TODO: Implement MySQL introspection
    Err(RpcError::internal_error(
        "MySQL introspection not yet implemented",
    ))
}

/// Sanitize connection string (remove passwords)
fn sanitize_connection_string(url: &str) -> String {
    if let Some(idx) = url.find("://") {
        if let Some(at_idx) = url[idx + 3..].find('@') {
            let protocol = &url[..idx + 3];
            let rest = &url[idx + 3 + at_idx..];
            return format!("{}***{}", protocol, rest);
        }
    }
    url.to_string()
}
