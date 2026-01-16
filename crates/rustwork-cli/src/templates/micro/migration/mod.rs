pub const MIGRATION_UP_SQL: &str = r#"-- Migration UP: {{ migration_name }}
-- Created: {{ timestamp }}

-- Example: Create a table
-- CREATE TABLE IF NOT EXISTS users (
--     id INTEGER PRIMARY KEY AUTOINCREMENT,
--     email TEXT NOT NULL UNIQUE,
--     created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
-- );

-- Add your migration SQL here
"#;

pub const MIGRATION_DOWN_SQL: &str = r#"-- Migration DOWN: {{ migration_name }}
-- Created: {{ timestamp }}

-- Example: Drop a table
-- DROP TABLE IF EXISTS users;

-- Add your rollback SQL here
"#;

pub const MIGRATION_README: &str = r#"# Migrations SQL

Ce dossier contient les migrations de base de données au format SQL pur, compatible avec sqlx.

## Structure

Chaque migration se compose de deux fichiers :
- `YYYYMMDD_HHMMSS_<nom>.up.sql` : migration à appliquer
- `YYYYMMDD_HHMMSS_<nom>.down.sql` : rollback de la migration

## Utilisation avec sqlx-cli

```bash
# Installer sqlx-cli
cargo install sqlx-cli

# Appliquer les migrations
sqlx migrate run

# Rollback de la dernière migration
sqlx migrate revert

# Créer une nouvelle migration
sqlx migrate add <nom>
```

## Exemple

```sql
-- 20240101_120000_create_users.up.sql
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 20240101_120000_create_users.down.sql
DROP TABLE IF EXISTS users;
```
"#;

pub const MIGRATION_INITIAL_UP: &str = r#"-- Initial migration
-- This migration is automatically created but empty
-- Add your initial database schema here if needed
"#;

pub const MIGRATION_INITIAL_DOWN: &str = r#"-- Initial migration rollback
-- Add rollback SQL here if needed
"#;
