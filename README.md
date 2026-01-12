# Rustwork

**Rustwork** est un mini-framework Rust inspiré de Laravel, conçu pour faciliter le développement d'APIs REST modernes et de microservices gRPC avec une structure claire et des conventions établies.

## 🚀 Caractéristiques

- **Architecture Laravel-style** avec conventions claires
- **Axum** comme runtime HTTP performant
- **SeaORM** pour l'ORM et les migrations
- **gRPC natif** avec DSL simple (`.rwk`) - pas de proto manuel !
- **Multi-database** : SQLite, PostgreSQL, MySQL (switch via .env à la Laravel)
- **SQLite par défaut** : zéro configuration pour commencer
- **Configuration flexible** avec support des profils (dev/test/prod)
- **Gestion d'erreurs unifiée** avec `AppError` et `ApiResponse<T>`
- **CLI puissant** pour la génération de code et migrations
- **Mode développement** avec hot-reload via cargo-watch
- **Tracing et logging** intégrés avec support OpenTelemetry optionnel
- **Support monorepo/micro-services** avec génération automatique de clients

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

### ⚠️ Note importante

**Rustwork n'est pas encore publié sur crates.io**. Pour l'utiliser, vous devez cloner le dépôt localement.

### Installation depuis le code source

```bash
git clone https://github.com/entcorporg/rustwork.git
cd rustwork
cargo build --release --bin rustwork
# Optionnel: installer la CLI globalement
cargo install --path crates/rustwork-cli
```

Le binaire `rustwork` sera disponible dans `target/release/rustwork` ou dans votre PATH si installé globalement.

### Utilisation locale

Les projets générés par Rustwork utilisent une dépendance locale vers le framework. Vous devez donc :

1. Cloner Rustwork dans un répertoire accessible
2. Créer vos projets dans le même répertoire parent que Rustwork

**Exemple de structure recommandée :**
```
workspace/
├── rustwork/              # Le framework cloné
│   └── crates/rustwork/
└── mon-api/              # Votre projet (généré avec rustwork new)
    └── Cargo.toml        # → rustwork = { path = "../rustwork/crates/rustwork" }
```

Cette contrainte est **temporaire** et sera supprimée lors de la publication sur crates.io.

## 🎯 Quick Start

### Créer un nouveau projet

```bash
rustwork new mon-api
cd mon-api
```

### Configuration

```bash
cp .env.example .env
# Par défaut, SQLite est utilisé (zéro configuration)
# Pour PostgreSQL/MySQL, éditez .env
```

Le projet généré utilise **SQLite par défaut** dans `./data/app.db` - aucune configuration requise !

#### Changer de base de données

Éditez votre `.env` :

**Pour PostgreSQL :**
```bash
DB_CONNECTION=postgres
DB_HOST=127.0.0.1
DB_PORT=5432
DB_DATABASE=mon_api
DB_USERNAME=postgres
DB_PASSWORD=secret
```

**Pour MySQL :**
```bash
DB_CONNECTION=mysql
DB_HOST=127.0.0.1
DB_PORT=3306
DB_DATABASE=mon_api
DB_USERNAME=root
DB_PASSWORD=secret
```

**Ou via URL directe (prioritaire) :**
```bash
DB_URL=postgres://user:pass@localhost:5432/database
```

**Docker Compose exemples :**

PostgreSQL :
```yaml
version: '3.8'
services:
  postgres:
    image: postgres:16
    environment:
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: secret
      POSTGRES_DB: mon_api
    ports:
      - "5432:5432"
```

MySQL :
```yaml
version: '3.8'
services:
  mysql:
    image: mysql:8
    environment:
      MYSQL_ROOT_PASSWORD: secret
      MYSQL_DATABASE: mon_api
    ports:
      - "3306:3306"
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
├── migration/               # Crate de migrations SeaORM
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs           # Migrator
│       ├── main.rs          # CLI binaire
│       └── m*.rs            # Fichiers de migration
├── config/
│   ├── default.toml         # Config par défaut
│   └── dev.toml             # Config développement
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

# Mode développement avec MCP (Model Context Protocol)
rustwork dev --mcp
```

Le serveur démarre sur `http://localhost:3000` par défaut.

#### 🏗️ Mode Workspace Multi-Services

`rustwork dev` supporte nativement les architectures **micro-services** ! 

Lancez la commande depuis **n'importe quel dossier parent** et tous les services Rustwork valides seront automatiquement détectés et lancés en parallèle.

**Exemple :**
```bash
# Structure
backend/
└── services/
    ├── auth/
    ├── user/
    └── blog/

# Lancer depuis n'importe où
cd backend/
rustwork dev

# Résultat:
# 🔧 Starting Rustwork development workspace...
# 🔍 Detected 3 Rustwork service(s):
#   - auth (services/auth)
#   - user (services/user)
#   - blog (services/blog)
# 
# ▶ Starting auth...
# ▶ Starting user...
# ▶ Starting blog...
# 
# [auth] Compiling auth v0.1.0...
# [user] Compiling user v0.1.0...
# [blog] Compiling blog v0.1.0...
```

**Fonctionnalités :**
- ✅ Détection automatique depuis n'importe quel dossier parent
- ✅ Lancement parallèle de tous les services
- ✅ Logs préfixés par service : `[service-name] log...`
- ✅ Hot-reload indépendant par service
- ✅ MCP centralisé observant tout le workspace
- ✅ Mode single-service préservé pour compatibilité

📚 **Guide complet :** [docs/DEV_WORKSPACE.md](docs/DEV_WORKSPACE.md)

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
- `migration/src/m<timestamp>_create_posts.rs` (migration)
- Met à jour les fichiers `mod.rs` et `migration/src/lib.rs`

## � Support gRPC

Rustwork intègre un support gRPC complet avec un DSL simple (`.rwk`) qui génère automatiquement les fichiers `.proto`, `build.rs`, et le code Rust.

### Quick Start gRPC

1. **Créer un fichier DSL** `grpc/user.rwk` :

```rwk
service UserService

rpc GetUser (GetUserRequest) returns (User)
rpc CreateUser (CreateUserRequest) returns (User)

message GetUserRequest {
  id: uuid
}

message CreateUserRequest {
  email: string
  name: string
}

message User {
  id: uuid
  email: string
  name: string
  created_at: datetime
}
```

2. **Générer le code** :

```bash
rustwork grpc build
```

3. **Implémenter le handler** :

```rust
use async_trait::async_trait;
use crate::grpc::UserServiceHandler;

pub struct MyHandler;

#[async_trait]
impl UserServiceHandler for MyHandler {
    async fn get_user(&self, req: GetUserRequest) -> Result<User, Status> {
        // Votre logique ici
        Ok(User { ... })
    }
    
    async fn create_user(&self, req: CreateUserRequest) -> Result<User, Status> {
        // Votre logique ici
        Ok(User { ... })
    }
}
```

4. **Démarrer le serveur** :

```rust
use tonic::transport::Server;
use crate::grpc::grpc_service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    Server::builder()
        .add_service(grpc_service(MyHandler))
        .serve(addr)
        .await?;
    Ok(())
}
```

📖 **Documentation complète** : [docs/GRPC.md](docs/GRPC.md)

## �📝 Conventions

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
3. Variables d'environnement `.env` (style Laravel)
4. Variables d'environnement `APP__*` (override final)

Exemple `config/default.toml` :

```toml
[server]
host = "0.0.0.0"
port = 3000

[database]
connection = "sqlite"
sqlite_path = "./data/app.db"

[database.pool]
max_connections = 10
min_connections = 2
connect_timeout_ms = 8000

[cors]
enabled = false
allowed_origins = []
allowed_methods = ["GET", "POST", "PUT", "PATCH", "DELETE"]
allowed_headers = ["Content-Type", "Accept"]
allow_credentials = false
max_age_seconds = 3600
```

Variables d'environnement Laravel-style (`.env`) :

```bash
APP_ENV=dev

# Database (SQLite par défaut)
DB_CONNECTION=sqlite
DB_SQLITE_PATH=./data/app.db

# Ou PostgreSQL
# DB_CONNECTION=postgres
# DB_HOST=127.0.0.1
# DB_PORT=5432
# DB_DATABASE=mydb
# DB_USERNAME=user
# DB_PASSWORD=pass

# Override via APP__* (priorité finale)
APP__SERVER__PORT=8080
```

### 🗄️ Database Info Endpoint

Endpoint `/db/info` pour debug (retourne la config DB sanitisée) :

```bash
curl http://localhost:3000/db/info
```

```json
{
  "connection": "sqlite",
  "url": "sqlite://./data/app.db?mode=rwc",
  "pool": {
    "max_connections": 10,
    "min_connections": 2,
    "connect_timeout_ms": 8000
  }
}
```

## 🗃️ Migrations

Rustwork utilise **sea-orm-migration** pour des migrations portables entre SQLite, PostgreSQL et MySQL. Les migrations sont écrites en Rust, pas en SQL brut.

### Structure des migrations

Les projets créés avec `rustwork new` incluent un crate `migration/` :

```
mon-api/
├── migration/
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs              # Migrator principal
│   │   ├── main.rs             # CLI binaire
│   │   └── m<timestamp>_*.rs   # Fichiers de migration
```

### Gérer les migrations

```bash
# Voir le statut des migrations
rustwork db status

# Lancer toutes les migrations
rustwork db migrate

# Lancer N migrations spécifiques
rustwork db migrate --steps 2

# Rollback de la dernière migration
rustwork db rollback

# Rollback de N migrations
rustwork db rollback --steps 2
```

### Générer des migrations

Quand vous créez un modèle avec `rustwork make model`, une migration est automatiquement générée :

```bash
rustwork make model Post
# Crée: migration/src/m<timestamp>_create_posts.rs
# Met à jour: migration/src/lib.rs
```

Les migrations utilisent le SchemaManager de SeaORM pour être **portables** entre bases de données :

```rust
async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
        .create_table(
            Table::create()
                .table(Post::Table)
                .col(ColumnDef::new(Post::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key())
                .col(ColumnDef::new(Post::Title).string().not_null())
                .to_owned(),
        )
        .await
}
```

## 🔧 Features

Le crate `rustwork` supporte une feature optionnelle :

```toml
[dependencies]
rustwork = { version = "0.1", features = ["otel"] }
```

- `otel` : Support OpenTelemetry pour tracing distribué

- `rest` (défaut) : Support REST de base
- `graphql` : Active async-graphql et intégration Axum
- `otel` : Active OpenTelemetry pour le tracing distribué

## 📚 API Reference

### Core Exports

```rust
use rustwork::{
    // Configuration
    AppConfig, DatabaseConfig, ServerConfig, CorsConfig,
    
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
- [x] CORS sécurisé configurable (fail-by-default)
- [ ] Support OpenTelemetry
- [ ] Génération de tests
- [ ] Commande MCP pour introspection
- [ ] Templates personnalisables
- [ ] Support multi-DB (MySQL, SQLite)
- [ ] CLI interactive

## 🔒 Security Model

### Authentication

**Rustwork does NOT implement authentication by default.** This is a deliberate design decision:

- No JWT tokens, no OAuth, no sessions built-in
- Authentication should be implemented by your application layer or delegated to a reverse proxy
- This keeps the framework lightweight and flexible

If you need authentication, you have several options:
- Implement custom middleware in your application
- Use an authentication service (Auth0, Keycloak, etc.)
- Place your API behind a reverse proxy with auth (nginx, Traefik, etc.)

### CORS (Cross-Origin Resource Sharing)

CORS is the **only cross-origin security mechanism built into Rustwork**.

**Configuration is fail-by-default and secure:**

```toml
# config/default.toml
[cors]
enabled = false  # CORS is disabled by default
allowed_origins = []  # REQUIRED if enabled=true
allowed_methods = ["GET", "POST", "PUT", "PATCH", "DELETE"]
allowed_headers = ["Content-Type", "Accept"]
allow_credentials = false
max_age_seconds = 3600
```

**Environment variables:**
```bash
APP__CORS__ENABLED=true
APP__CORS__ALLOWED_ORIGINS=["http://localhost:3000", "https://myapp.com"]
```

**Important CORS rules:**
- If `cors.enabled = false`, no CORS headers are added
- If `cors.enabled = true` but `allowed_origins` is empty, **the application will panic at startup**
- No wildcards (`*`) are allowed in origins
- All origins must be valid URLs starting with `http://` or `https://`

This ensures you never accidentally expose your API to unwanted origins.

### General Security Recommendations

- Always use HTTPS in production
- Set `allow_credentials = true` only if you need to send cookies/auth headers cross-origin
- Keep `allowed_origins` as restrictive as possible
- Use environment variables for production configuration
- Never commit secrets to your repository

## 📄 License

MIT

## 🤝 Contributing

Les contributions sont les bienvenues! Ouvrez une issue ou une PR sur GitHub.

---

**Made with ❤️ for the Rust community**

