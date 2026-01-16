/// Test d'intégration pour rustwork_get_database_schema
///
/// Ce test crée une structure de projet complète avec une base SQLite
/// et vérifie que le tool MCP peut détecter et introspecter la base de données.
use std::fs;
use tempfile::TempDir;

#[tokio::test]
async fn test_database_schema_detection_with_env_example() {
    // Créer un workspace temporaire
    let temp_dir = TempDir::new().unwrap();
    let workspace = temp_dir.path();

    // Créer structure Backend/services/auth
    let auth_service = workspace.join("Backend/services/auth");
    fs::create_dir_all(&auth_service).unwrap();

    // Créer Cargo.toml
    fs::write(
        auth_service.join("Cargo.toml"),
        r#"[package]
name = "auth"
version = "0.1.0"
edition = "2021"
"#,
    )
    .unwrap();

    // Créer dossier data avec dev.db
    let data_dir = auth_service.join("data");
    fs::create_dir(&data_dir).unwrap();

    // Créer une vraie base SQLite avec une table
    let db_path = data_dir.join("dev.db");
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&format!("sqlite://{}?mode=rwc", db_path.display()))
        .await
        .expect("Failed to create test database");

    // Créer une table de test
    sqlx::query(
        r#"
        CREATE TABLE users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            email TEXT NOT NULL UNIQUE,
            username TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(&pool)
    .await
    .expect("Failed to create test table");

    pool.close().await;

    // Créer .env.example avec DATABASE_URL commenté (chemin absolu vers la vraie base)
    fs::write(
        auth_service.join(".env.example"),
        format!("# DATABASE_URL=sqlite://{}?mode=rwc\n", db_path.display()),
    )
    .unwrap();

    // Maintenant testons la détection
    let service_path = &auth_service;

    // Test 1: Détection de la config
    println!("🔍 Test de détection de configuration...");
    let config =
        rustwork_cli::mcp::rustwork_get_database_schema::introspection::detect_database_config(
            service_path,
        )
        .await
        .expect("Should detect database config");

    assert_eq!(
        config.db_type,
        rustwork_cli::mcp::rustwork_get_database_schema::introspection::DatabaseType::Sqlite
    );
    assert!(config.connection_string.contains("sqlite://"));
    println!("✅ Configuration détectée: {:?}", config.connection_string);

    // Test 2: Introspection de la base
    println!("🔍 Test d'introspection de la base...");
    let schema =
        rustwork_cli::mcp::rustwork_get_database_schema::introspection::introspect_database(
            &config,
        )
        .await
        .expect("Should introspect database");

    assert_eq!(schema.database_type, "sqlite");
    assert_eq!(schema.tables.len(), 1);
    assert_eq!(schema.tables[0].name, "users");
    assert_eq!(schema.tables[0].columns.len(), 4);

    // Vérifier les colonnes
    let column_names: Vec<&str> = schema.tables[0]
        .columns
        .iter()
        .map(|c| c.name.as_str())
        .collect();

    assert!(column_names.contains(&"id"));
    assert!(column_names.contains(&"email"));
    assert!(column_names.contains(&"username"));
    assert!(column_names.contains(&"created_at"));

    println!("✅ Schema introspectée:");
    println!("   - Table: {}", schema.tables[0].name);
    println!("   - Colonnes: {}", column_names.join(", "));
}

#[tokio::test]
async fn test_database_schema_detection_with_dev_db() {
    // Test avec juste un fichier dev.db sans .env
    let temp_dir = TempDir::new().unwrap();
    let workspace = temp_dir.path();

    let service = workspace.join("services/myservice");
    let data_dir = service.join("data");
    fs::create_dir_all(&data_dir).unwrap();
    fs::write(service.join("Cargo.toml"), "[package]").unwrap();

    // Créer une base vide
    let db_path = data_dir.join("dev.db");
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&format!("sqlite://{}?mode=rwc", db_path.display()))
        .await
        .expect("Failed to create test database");

    pool.close().await;

    // Test de détection
    let config =
        rustwork_cli::mcp::rustwork_get_database_schema::introspection::detect_database_config(
            &service,
        )
        .await
        .expect("Should detect dev.db");

    assert_eq!(
        config.db_type,
        rustwork_cli::mcp::rustwork_get_database_schema::introspection::DatabaseType::Sqlite
    );
    assert!(config.connection_string.contains("dev.db"));
}
