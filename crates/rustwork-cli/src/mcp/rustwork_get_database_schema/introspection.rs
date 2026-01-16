use super::schema_types::*;
use crate::mcp::common::protocol::RpcError;
use sqlx::{Row, SqlitePool};
use std::path::Path;
use std::str::FromStr;
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
    let env_files = vec![".env", ".env.local", ".env.development", ".env.example"];

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
            // Supporte les lignes commentées dans .env.example
            let line = line.strip_prefix('#').unwrap_or(line).trim();

            if line.starts_with("DATABASE_URL=") || line.starts_with("DB_URL=") {
                // Utiliser splitn pour ne split qu'une seule fois sur le premier '='
                let url = line.splitn(2, '=').nth(1).unwrap_or("").trim();
                let url = url.trim_matches('"').trim_matches('\'');

                if !url.is_empty() {
                    return parse_database_url(url, service_path);
                }
            }
        }
    }

    Err(RpcError::internal_error(
        "No DATABASE_URL found in .env files",
    ))
}

/// Parse database URL to determine type and resolve relative paths
fn parse_database_url(url: &str, service_path: &Path) -> Result<DatabaseConfig, RpcError> {
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

    // Pour SQLite, résoudre les chemins relatifs
    let connection_string = if db_type == DatabaseType::Sqlite {
        resolve_sqlite_path(url, service_path)?
    } else {
        url.to_string()
    };

    Ok(DatabaseConfig {
        db_type,
        connection_string,
    })
}

/// Résout un chemin SQLite relatif par rapport au service
fn resolve_sqlite_path(url: &str, service_path: &Path) -> Result<String, RpcError> {
    // Extraire le chemin de l'URL SQLite
    // Format: sqlite://path/to/db.db?options ou sqlite:path/to/db.db

    let path_part = if let Some(stripped) = url.strip_prefix("sqlite://") {
        stripped
    } else if let Some(stripped) = url.strip_prefix("sqlite:") {
        stripped
    } else {
        return Ok(url.to_string());
    };

    // Séparer le chemin des paramètres de requête
    let (path, query) = if let Some(idx) = path_part.find('?') {
        (&path_part[..idx], &path_part[idx..])
    } else {
        (path_part, "")
    };

    // Si le chemin est déjà absolu, le garder tel quel
    let path_buf = std::path::PathBuf::from(path);
    if path_buf.is_absolute() {
        return Ok(url.to_string());
    }

    // Résoudre le chemin relatif par rapport au service
    let absolute_path = service_path.join(path);
    let absolute_path = absolute_path.canonicalize().unwrap_or(absolute_path); // Si canonicalize échoue, utiliser le chemin non canonicalisé

    Ok(format!("sqlite://{}{}", absolute_path.display(), query))
}

/// Detect SQLite database file in service directory
async fn detect_sqlite_file(service_path: &Path) -> Result<DatabaseConfig, RpcError> {
    // Cherche dans les dossiers courants pour les bases de données
    let search_dirs = vec![
        service_path.to_path_buf(),
        service_path.join("data"),
        service_path.join("database"),
        service_path.join("db"),
    ];

    let common_names = vec![
        "dev.db",
        "db.sqlite",
        "database.sqlite",
        "app.db",
        "data.db",
    ];

    for dir in &search_dirs {
        if !dir.exists() {
            continue;
        }

        // Cherche les noms de fichiers courants
        for name in &common_names {
            let db_path = dir.join(name);
            if db_path.exists() {
                return Ok(DatabaseConfig {
                    db_type: DatabaseType::Sqlite,
                    connection_string: format!("sqlite://{}?mode=ro", db_path.display()),
                });
            }
        }

        // Cherche tous les fichiers .db et .sqlite dans le dossier
        if let Ok(entries) = tokio::fs::read_dir(dir).await {
            let mut entries = entries;
            while let Ok(Some(entry)) = entries.next_entry().await {
                if let Ok(file_type) = entry.file_type().await {
                    if file_type.is_file() {
                        if let Some(file_name) = entry.file_name().to_str() {
                            if file_name.ends_with(".db") || file_name.ends_with(".sqlite") {
                                let db_path = entry.path();
                                return Ok(DatabaseConfig {
                                    db_type: DatabaseType::Sqlite,
                                    connection_string: format!(
                                        "sqlite://{}?mode=ro",
                                        db_path.display()
                                    ),
                                });
                            }
                        }
                    }
                }
            }
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
    let opts = sqlx::sqlite::SqliteConnectOptions::from_str(&config.connection_string)
        .map_err(|e| RpcError::internal_error(format!("Invalid SQLite URL: {}", e)))?;

    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_secs(5))
        .connect_with(opts)
        .await
        .map_err(|e| RpcError::internal_error(format!("Failed to connect to SQLite: {}", e)))?;

    let tables = get_sqlite_tables(&pool).await?;

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
async fn get_sqlite_tables(pool: &SqlitePool) -> Result<Vec<TableSchema>, RpcError> {
    let rows = sqlx::query("SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' ORDER BY name")
        .fetch_all(pool)
        .await
        .map_err(|e| RpcError::internal_error(format!("Failed to query tables: {}", e)))?;

    let mut tables = Vec::new();

    for row in rows {
        let table_name: String = row
            .try_get("name")
            .map_err(|e| RpcError::internal_error(format!("Failed to get table name: {}", e)))?;

        let columns = get_sqlite_columns(pool, &table_name).await?;
        let indexes = get_sqlite_indexes(pool, &table_name).await?;
        let foreign_keys = get_sqlite_foreign_keys(pool, &table_name).await?;

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
    pool: &SqlitePool,
    table_name: &str,
) -> Result<Vec<ColumnSchema>, RpcError> {
    let query = format!("PRAGMA table_info('{}')", table_name);
    let rows = sqlx::query(&query)
        .fetch_all(pool)
        .await
        .map_err(|e| RpcError::internal_error(format!("Failed to query columns: {}", e)))?;

    let mut columns = Vec::new();

    for row in rows {
        let name: String = row
            .try_get("name")
            .map_err(|e| RpcError::internal_error(format!("Failed to get column name: {}", e)))?;

        let data_type: String = row
            .try_get("type")
            .map_err(|e| RpcError::internal_error(format!("Failed to get column type: {}", e)))?;

        let not_null: i32 = row.try_get("notnull").unwrap_or(0);
        let pk: i32 = row.try_get("pk").unwrap_or(0);
        let default_value: Option<String> = row.try_get("dflt_value").ok();

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
    pool: &SqlitePool,
    table_name: &str,
) -> Result<Vec<IndexSchema>, RpcError> {
    let query = format!("PRAGMA index_list('{}')", table_name);
    let rows = sqlx::query(&query)
        .fetch_all(pool)
        .await
        .map_err(|e| RpcError::internal_error(format!("Failed to query indexes: {}", e)))?;

    let mut indexes = Vec::new();

    for row in rows {
        let index_name: String = row
            .try_get("name")
            .map_err(|e| RpcError::internal_error(format!("Failed to get index name: {}", e)))?;

        let unique: i32 = row.try_get("unique").unwrap_or(0);

        // Get columns in this index
        let columns_query = format!("PRAGMA index_info('{}')", index_name);
        let columns_rows = sqlx::query(&columns_query)
            .fetch_all(pool)
            .await
            .map_err(|e| {
                RpcError::internal_error(format!("Failed to query index columns: {}", e))
            })?;

        let columns: Vec<String> = columns_rows
            .iter()
            .filter_map(|col_row| col_row.try_get::<String, _>("name").ok())
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
    pool: &SqlitePool,
    table_name: &str,
) -> Result<Vec<ForeignKeySchema>, RpcError> {
    let query = format!("PRAGMA foreign_key_list('{}')", table_name);
    let rows = sqlx::query(&query)
        .fetch_all(pool)
        .await
        .map_err(|e| RpcError::internal_error(format!("Failed to query foreign keys: {}", e)))?;

    let mut foreign_keys = Vec::new();

    for row in rows {
        let referenced_table: String = row.try_get("table").map_err(|e| {
            RpcError::internal_error(format!("Failed to get referenced table: {}", e))
        })?;

        let from_col: String = row
            .try_get("from")
            .map_err(|e| RpcError::internal_error(format!("Failed to get from column: {}", e)))?;

        let to_col: String = row
            .try_get("to")
            .map_err(|e| RpcError::internal_error(format!("Failed to get to column: {}", e)))?;

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_detect_sqlite_file_in_data_folder() {
        let temp_dir = TempDir::new().unwrap();
        let service_path = temp_dir.path();
        let data_dir = service_path.join("data");
        fs::create_dir(&data_dir).unwrap();

        // Crée un fichier dev.db
        let db_path = data_dir.join("dev.db");
        fs::write(&db_path, b"").unwrap();

        let result = detect_sqlite_file(service_path).await;
        assert!(result.is_ok());

        let config = result.unwrap();
        assert_eq!(config.db_type, DatabaseType::Sqlite);
        assert!(config.connection_string.contains("dev.db"));
    }

    #[tokio::test]
    async fn test_read_env_example_with_commented_database_url() {
        let temp_dir = TempDir::new().unwrap();
        let service_path = temp_dir.path();

        // Crée un .env.example avec DATABASE_URL commenté
        let env_example = service_path.join(".env.example");
        fs::write(
            &env_example,
            "# DATABASE_URL=sqlite://data/db.sqlite?mode=rwc\n",
        )
        .unwrap();

        let result = read_env_database_config(service_path).await;
        assert!(result.is_ok());

        let config = result.unwrap();
        assert_eq!(config.db_type, DatabaseType::Sqlite);
        assert!(config.connection_string.contains("sqlite://"));
    }

    #[tokio::test]
    async fn test_parse_database_url_sqlite() {
        let temp_dir = TempDir::new().unwrap();
        let service_path = temp_dir.path();
        
        let url = "sqlite://data/db.sqlite?mode=rwc";
        let result = parse_database_url(url, service_path);
        assert!(result.is_ok());

        let config = result.unwrap();
        assert_eq!(config.db_type, DatabaseType::Sqlite);
        // Vérifie que le chemin a été résolu
        assert!(config.connection_string.contains("data/db.sqlite") || 
                config.connection_string.contains("data\\db.sqlite"));
    }

    #[tokio::test]
    async fn test_parse_database_url_postgres() {
        let temp_dir = TempDir::new().unwrap();
        let service_path = temp_dir.path();
        
        let url = "postgres://user:pass@localhost/mydb";
        let result = parse_database_url(url, service_path);
        assert!(result.is_ok());

        let config = result.unwrap();
        assert_eq!(config.db_type, DatabaseType::Postgres);
        // Postgres URL ne devrait pas être modifiée
        assert_eq!(config.connection_string, url);
    }

    #[tokio::test]
    async fn test_resolve_sqlite_relative_path() {
        let temp_dir = TempDir::new().unwrap();
        let service_path = temp_dir.path();
        
        // Créer un fichier de base de données pour le test
        let data_dir = service_path.join("data");
        fs::create_dir(&data_dir).unwrap();
        let db_path = data_dir.join("dev.db");
        fs::write(&db_path, b"").unwrap();
        
        // Tester avec un chemin relatif
        let url = "sqlite://data/dev.db?mode=rwc";
        let result = parse_database_url(url, service_path);
        assert!(result.is_ok());
        
        let config = result.unwrap();
        assert_eq!(config.db_type, DatabaseType::Sqlite);
        // Le chemin devrait maintenant être absolu
        assert!(config.connection_string.starts_with("sqlite://"));
        assert!(config.connection_string.contains("dev.db"));
        assert!(config.connection_string.contains("?mode=rwc"));
        // Vérifier que c'est un chemin absolu (contient le temp_dir)
        let expected_substr = service_path.display().to_string();
        assert!(config.connection_string.contains(&expected_substr) || 
                // Sur Windows, les chemins peuvent être différents
                config.connection_string.contains(&expected_substr.replace('\\', "/")));
    }

    #[test]
    fn test_sanitize_connection_string() {
        let url = "postgres://user:password@localhost/mydb";
        let sanitized = sanitize_connection_string(url);
        assert_eq!(sanitized, "postgres://***@localhost/mydb");
    }

    #[test]
    fn test_sanitize_connection_string_no_password() {
        let url = "sqlite://data/db.sqlite";
        let sanitized = sanitize_connection_string(url);
        assert_eq!(sanitized, "sqlite://data/db.sqlite");
    }
}
