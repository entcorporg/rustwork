use serde::{Deserialize, Serialize};

/// Type de connexion DB (à la Laravel)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DbConnection {
    Sqlite,
    Postgres,
    Mysql,
}

impl std::str::FromStr for DbConnection {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "sqlite" => Ok(DbConnection::Sqlite),
            "postgres" | "postgresql" | "pgsql" => Ok(DbConnection::Postgres),
            "mysql" | "mariadb" => Ok(DbConnection::Mysql),
            _ => Err(format!(
                "Invalid DB_CONNECTION: {}. Use sqlite, postgres, or mysql",
                s
            )),
        }
    }
}
