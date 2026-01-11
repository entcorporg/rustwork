#[allow(dead_code)]
pub const ENV_EXAMPLE: &str = r#"APP_ENV=dev

# Database Configuration (Laravel-style)
# ==========================================
# Par défaut: SQLite (zéro configuration)
DB_CONNECTION=sqlite
DB_SQLITE_PATH=./data/app.db

# Alternative: PostgreSQL
# DB_CONNECTION=postgres
# DB_HOST=127.0.0.1
# DB_PORT=5432
# DB_DATABASE={{ project_name }}
# DB_USERNAME=postgres
# DB_PASSWORD=secret
# Ou directement via URL (priorité sur les variables ci-dessus):
# DB_URL=postgres://postgres:secret@127.0.0.1:5432/{{ project_name }}

# Alternative: MySQL
# DB_CONNECTION=mysql
# DB_HOST=127.0.0.1
# DB_PORT=3306
# DB_DATABASE={{ project_name }}
# DB_USERNAME=root
# DB_PASSWORD=secret
# Ou directement via URL:
# DB_URL=mysql://root:secret@127.0.0.1:3306/{{ project_name }}

# Server Configuration (override via APP__*)
# APP__SERVER__HOST=0.0.0.0
# APP__SERVER__PORT=3000

# CORS Configuration
# APP__CORS__ENABLED=true
# APP__CORS__ALLOWED_ORIGINS=["http://localhost:3000"]
"#;
