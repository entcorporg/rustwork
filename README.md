# Rustwork

**Rustwork** est un mini-framework Rust inspiré de Laravel, conçu pour faciliter le développement d'APIs REST modernes avec une structure claire et des conventions établies.

## 🚀 Caractéristiques

- **Architecture Laravel-style** avec conventions claires
- **Axum** comme runtime HTTP performant
- **SeaORM** pour l'ORM et les migrations
- **Configuration flexible** avec support des profils (dev/test/prod)
- **Gestion d'erreurs unifiée** avec `AppError` et `ApiResponse<T>`
- **CLI puissant** pour la génération de code (controllers, models, migrations)
- **Mode développement** avec hot-reload via cargo-watch
- **Tracing et logging** intégrés avec support OpenTelemetry optionnel
- **GraphQL** optionnel via async-graphql

## 📦 Structure du Workspace

```
rustwork/
├── crates/
│   ├── rustwork/           # Framework core library
│   └── rustwork-cli/       # CLI tool
├── templates/              # Code generation templates (embedded)
└── README.md
```

## 🛠️ Installation

### Depuis le code source

```bash
git clone https://github.com/your-org/rustwork.git
cd rustwork
cargo build --release
cargo install --path crates/rustwork-cli
```

Le binaire `rustwork` sera disponible dans votre PATH.

## 🎯 Quick Start

### Créer un nouveau projet

```bash
rustwork new mon-api
cd mon-api
```

### Configuration

```bash
cp .env.example .env
# Éditez .env avec vos paramètres de base de données
```

### Structure générée

```
mon-api/
├── src/
│   ├── main.rs              # Point d'entrée
│   ├── app.rs               # Construction du router
│   ├── routes.rs            # Définition des routes
│   ├── errors.rs            # Erreurs personnalisées
│   ├── controllers/         # Controllers REST
│   │   ├── mod.rs
│   │   └── health.rs
│   ├── models/              # Entités SeaORM
│   ├── services/            # Logique métier
│   ├── middlewares/         # Middlewares custom
│   └── graphql/             # Schema GraphQL (optionnel)
├── config/
│   ├── default.toml         # Config par défaut
│   └── dev.toml             # Config développement
├── migrations/              # Migrations SeaORM
├── .rustwork/
│   └── manifest.json        # Métadonnées du projet
└── Cargo.toml
```

### Lancer le serveur

```bash
# Mode normal
cargo run

# Mode développement avec hot-reload
rustwork dev
```

Le serveur démarre sur `http://localhost:3000` par défaut.

### Tester la route health

```bash
curl http://localhost:3000/api/health
```

## 🎨 Génération de Code

### Créer un Controller

```bash
rustwork make controller User
```

Génère :
- `src/controllers/user.rs` avec méthodes CRUD (index, show, create, update, delete)
- Ajoute automatiquement les routes dans `src/routes.rs`
- Met à jour `src/controllers/mod.rs`

Routes créées :
- `GET    /api/users`
- `GET    /api/users/:id`
- `POST   /api/users`
- `PUT    /api/users/:id`
- `DELETE /api/users/:id`

### Créer un Model

```bash
rustwork make model Post
```

Génère :
- `src/models/post.rs` (entité SeaORM)
- `src/services/post_service.rs` (service avec logique métier)
- `migrations/<timestamp>_create_posts.rs` (migration)
- Met à jour les fichiers `mod.rs`

## 📝 Conventions

### Controllers

Les controllers suivent la convention REST :

```rust
use axum::{extract::{State, Path}, Json};
use rustwork::{AppState, AppResult, ApiResponse, ok, created};

pub async fn index(
    State(state): State<AppState>,
) -> AppResult<(axum::http::StatusCode, Json<ApiResponse<Vec<Item>>>)> {
    // Récupérer tous les items
    Ok(ok(items))
}

pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreateRequest>,
) -> AppResult<(axum::http::StatusCode, Json<ApiResponse<Item>>)> {
    // Créer un item
    Ok(created(item))
}
```

### Gestion des Erreurs

Toutes les erreurs passent par `AppError` :

```rust
use rustwork::{AppError, AppResult};

pub async fn my_handler() -> AppResult<Json<ApiResponse<Data>>> {
    let item = fetch_item()
        .await
        .ok_or_else(|| AppError::NotFound("Item not found".to_string()))?;
    
    Ok(ok(item))
}
```

Types d'erreurs disponibles :
- `AppError::NotFound` → 404
- `AppError::BadRequest` → 400
- `AppError::Unauthorized` → 401
- `AppError::Forbidden` → 403
- `AppError::Validation` → 422
- `AppError::Conflict` → 409
- `AppError::Database` → 500
- `AppError::InternalError` → 500

### Réponses API

Format standard via `ApiResponse<T>` :

```rust
// Success
{
  "success": true,
  "data": { ... },
  "message": "Optional message"
}

// Error
{
  "success": false,
  "error": "Error message",
  "status": 404
}
```

Helpers disponibles :
- `ok(data)` → 200 OK
- `created(data)` → 201 Created
- `error(status, msg)` → Custom error

### Configuration

La configuration se charge par couches :

1. `config/default.toml` (base)
2. `config/{profile}.toml` (dev/test/prod)
3. Variables d'environnement `APP__*`

Exemple `config/default.toml` :

```toml
[server]
host = "0.0.0.0"
port = 3000

[database]
url = "postgres://user:pass@localhost/dbname"
max_connections = 10
min_connections = 2

[auth]
jwt_secret = "change-me-in-production"
jwt_expiration = 86400
```

Variables d'environnement :

```bash
APP_ENV=prod
APP__SERVER__PORT=8080
APP__DATABASE__URL=postgres://...
```

## 🔧 Features

Le crate `rustwork` supporte plusieurs features optionnelles :

```toml
[dependencies]
rustwork = { version = "0.1", features = ["graphql", "otel"] }
```

- `rest` (défaut) : Support REST de base
- `graphql` : Active async-graphql et intégration Axum
- `otel` : Active OpenTelemetry pour le tracing distribué

## 📚 API Reference

### Core Exports

```rust
use rustwork::{
    // Configuration
    AppConfig, DatabaseConfig, ServerConfig, AuthConfig,
    
    // State & App
    AppState,
    build_router,
    
    // Erreurs
    AppError, AppResult,
    
    // Réponses
    ApiResponse, ok, created, error,
    
    // Database
    init_database,
};
```

### Middlewares

```rust
use rustwork::middleware::{
    request_id_middleware,  // Ajoute X-Request-ID
    cors_layer,            // CORS permissif par défaut
};
```

### Database Helpers

```rust
use rustwork::db::Paginator;

let paginator = Paginator::new(page, per_page);
let items = Entity::find()
    .limit(paginator.limit())
    .offset(paginator.offset())
    .all(&db)
    .await?;
```

## 🧪 Tests

```bash
# Tester le workspace complet
cargo test --workspace

# Tester un projet généré
cd mon-api
cargo test
```

## 🚧 Roadmap

- [x] CLI avec génération de projet
- [x] Génération de controllers REST
- [x] Génération de models + services + migrations
- [x] Mode dev avec hot-reload
- [x] Configuration multi-environnements
- [x] Gestion d'erreurs unifiée
- [x] Middlewares de base (CORS, request_id, tracing)
- [ ] Support GraphQL complet
- [ ] Support OpenTelemetry
- [ ] Génération de tests
- [ ] Commande MCP pour introspection
- [ ] Templates personnalisables
- [ ] Support multi-DB (MySQL, SQLite)
- [ ] Auth/JWT helpers
- [ ] CLI interactive

## 📄 License

MIT

## 🤝 Contributing

Les contributions sont les bienvenues! Ouvrez une issue ou une PR sur GitHub.

---

**Made with ❤️ for the Rust community**

