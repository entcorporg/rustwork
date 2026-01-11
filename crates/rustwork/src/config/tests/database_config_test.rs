#[cfg(test)]
mod tests {
    use crate::config::builders::{
        resolve_database_url::resolve_database_url, sanitize_database_url::sanitize_database_url,
    };
    use crate::config::types::{DatabaseConfig, DbConnection};
    use std::env;

    fn setup_test_env() {
        // Nettoyer les variables d'environnement
        env::remove_var("DB_CONNECTION");
        env::remove_var("DB_URL");
        env::remove_var("DB_SQLITE_PATH");
        env::remove_var("DB_HOST");
        env::remove_var("DB_PORT");
        env::remove_var("DB_DATABASE");
        env::remove_var("DB_USERNAME");
        env::remove_var("DB_PASSWORD");
    }

    #[test]
    fn test_default_sqlite_url() {
        setup_test_env();

        let config = DatabaseConfig::default();
        let url = resolve_database_url(&config).unwrap();

        assert!(url.starts_with("sqlite://"));
        assert!(url.contains("data/app.db"));
    }

    #[test]
    fn test_postgres_url_construction() {
        setup_test_env();

        let config = DatabaseConfig {
            connection: DbConnection::Postgres,
            host: "localhost".to_string(),
            port: Some(5432),
            database: Some("test_db".to_string()),
            username: Some("user".to_string()),
            password: Some("pass".to_string()),
            ..Default::default()
        };

        let url = resolve_database_url(&config).unwrap();
        assert_eq!(url, "postgres://user:pass@localhost:5432/test_db");
    }

    #[test]
    fn test_mysql_url_construction() {
        setup_test_env();

        let config = DatabaseConfig {
            connection: DbConnection::Mysql,
            host: "localhost".to_string(),
            port: Some(3306),
            database: Some("test_db".to_string()),
            username: Some("root".to_string()),
            password: Some("secret".to_string()),
            ..Default::default()
        };

        let url = resolve_database_url(&config).unwrap();
        assert_eq!(url, "mysql://root:secret@localhost:3306/test_db");
    }

    #[test]
    fn test_explicit_url_takes_priority() {
        setup_test_env();

        let config = DatabaseConfig {
            url: Some("postgres://explicit:url@host:5432/db".to_string()),
            connection: DbConnection::Sqlite, // Devrait être ignoré
            ..Default::default()
        };

        let url = resolve_database_url(&config).unwrap();
        assert_eq!(url, "postgres://explicit:url@host:5432/db");
    }

    #[test]
    fn test_postgres_requires_database() {
        setup_test_env();

        let config = DatabaseConfig {
            connection: DbConnection::Postgres,
            database: None, // Pas de database
            ..Default::default()
        };

        let result = resolve_database_url(&config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("DB_DATABASE"));
    }

    #[test]
    fn test_mysql_requires_database() {
        setup_test_env();

        let config = DatabaseConfig {
            connection: DbConnection::Mysql,
            database: None,
            ..Default::default()
        };

        let result = resolve_database_url(&config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("DB_DATABASE"));
    }

    #[test]
    fn test_sanitized_url_masks_password() {
        setup_test_env();

        let config = DatabaseConfig {
            connection: DbConnection::Postgres,
            host: "localhost".to_string(),
            port: Some(5432),
            database: Some("test".to_string()),
            username: Some("user".to_string()),
            password: Some("secret123".to_string()),
            ..Default::default()
        };

        let sanitized = sanitize_database_url(&config).unwrap();

        // Le mot de passe doit être masqué
        assert!(!sanitized.contains("secret123"));
        assert!(sanitized.contains("***"));
        assert!(sanitized.contains("user"));
        assert!(sanitized.contains("localhost"));
    }

    #[test]
    fn test_sanitized_url_sqlite_no_password() {
        setup_test_env();

        let config = DatabaseConfig::default();
        let sanitized = sanitize_database_url(&config).unwrap();

        // SQLite n'a pas de mot de passe
        assert!(sanitized.starts_with("sqlite://"));
        assert!(!sanitized.contains("***"));
    }

    #[test]
    fn test_db_connection_from_str() {
        assert_eq!(
            "sqlite".parse::<DbConnection>().unwrap(),
            DbConnection::Sqlite
        );
        assert_eq!(
            "postgres".parse::<DbConnection>().unwrap(),
            DbConnection::Postgres
        );
        assert_eq!(
            "postgresql".parse::<DbConnection>().unwrap(),
            DbConnection::Postgres
        );
        assert_eq!(
            "mysql".parse::<DbConnection>().unwrap(),
            DbConnection::Mysql
        );
        assert_eq!(
            "mariadb".parse::<DbConnection>().unwrap(),
            DbConnection::Mysql
        );

        assert!("invalid".parse::<DbConnection>().is_err());
    }

    #[test]
    fn test_postgres_default_port() {
        setup_test_env();

        let config = DatabaseConfig {
            connection: DbConnection::Postgres,
            database: Some("test".to_string()),
            port: None, // Pas de port spécifié
            ..Default::default()
        };

        let url = resolve_database_url(&config).unwrap();
        assert!(url.contains(":5432/")); // Port par défaut
    }

    #[test]
    fn test_mysql_default_port() {
        setup_test_env();

        let config = DatabaseConfig {
            connection: DbConnection::Mysql,
            database: Some("test".to_string()),
            port: None,
            ..Default::default()
        };

        let url = resolve_database_url(&config).unwrap();
        assert!(url.contains(":3306/")); // Port par défaut
    }
}
