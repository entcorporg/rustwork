# Rustwork - Quick Reference

## Installation

```bash
# From source
git clone https://github.com/your-org/rustwork.git
cd rustwork
cargo install --path crates/rustwork-cli

# Verify installation
rustwork --version
```

## Commands

### Create New Project

```bash
rustwork new <project-name>
```

Example:
```bash
rustwork new my-api
cd my-api
```

### Database Commands

```bash
# Show migration status
rustwork db status

# Run all pending migrations
rustwork db migrate

# Run N migrations
rustwork db migrate --steps 2

# Rollback last migration
rustwork db rollback

# Rollback N migrations
rustwork db rollback --steps 2
```

**Note**: Les migrations sont maintenant basées sur **sea-orm-migration** et sont portables entre SQLite, PostgreSQL et MySQL. Elles sont écrites en Rust, pas en SQL brut.

### Generate Controller

```bash
rustwork make controller <ControllerName>
```

Example:
```bash
rustwork make controller User
# Creates: src/controllers/user.rs with CRUD methods
# Updates: src/routes.rs with REST routes
```

Generated routes:
- `GET    /api/users` → `index()`
- `GET    /api/users/:id` → `show()`
- `POST   /api/users` → `create()`
- `PUT    /api/users/:id` → `update()`
- `DELETE /api/users/:id` → `delete()`

### Generate Model

```bash
rustwork make model <ModelName>
```

Example:
```bash
rustwork make model Post
# Creates: src/models/post.rs (SeaORM entity)
# Creates: src/services/post_service.rs
# Creates: migrations/<timestamp>_create_posts.rs
```

### Development Server

```bash
rustwork dev
```

Requires `cargo-watch`:
```bash
cargo install cargo-watch
```

### MCP Server (Model Context Protocol)

Start an MCP server for IDE integration:

```bash
rustwork mcp
```

With custom options:
```bash
rustwork mcp --host 127.0.0.1 --port 4000 --project /path/to/project
```

#### Protocol

The MCP server implements JSON-RPC 2.0 over TCP with newline-delimited JSON messages.

**Security**: The server binds to `127.0.0.1` by default for security. It provides read-only access to project metadata and never exposes secrets.

#### Available Methods

##### get_manifest

Get the project manifest content.

Request:
```json
{"jsonrpc":"2.0","id":1,"method":"get_manifest","params":{}}
```

Response:
```json
{
  "jsonrpc":"2.0",
  "id":1,
  "result":{
    "version":"0.1.0",
    "routes":[],
    "models":[],
    "controllers":["health"]
  }
}
```

##### get_conventions

Get Rustwork framework conventions and best practices.

Request:
```json
{"jsonrpc":"2.0","id":2,"method":"get_conventions","params":{}}
```

Response:
```json
{
  "jsonrpc":"2.0",
  "id":2,
  "result":{
    "error_handling":{
      "type":"AppError",
      "location":"src/errors.rs",
      "variants":["NotFound","BadRequest","Forbidden","InternalError","DatabaseError"]
    },
    "response_format":{
      "type":"ApiResponse<T>",
      "structure":{
        "success":"ApiResponse::success(data)",
        "error":"ApiResponse::error(message)"
      }
    },
    "handler_signature":{
      "standard":"async fn handler(State(state): State<AppState>) -> Result<ApiResponse<T>, AppError>"
    },
    "configuration":{
      "env_keys":{
        "database":["DB_CONNECTION","DB_HOST","DB_PORT","DB_DATABASE","DB_USERNAME","DB_PASSWORD"],
        "application":["APP_HOST","APP_PORT","APP_ENV"]
      }
    },
    "middleware":{
      "builtin":["RequestId","Logger","Cors"]
    }
  }
}
```

##### get_routes

Get registered routes from the manifest.

Request:
```json
{"jsonrpc":"2.0","id":3,"method":"get_routes","params":{}}
```

Response:
```json
{
  "jsonrpc":"2.0",
  "id":3,
  "result":[]
}
```

##### get_models

Get registered models from the manifest.

Request:
```json
{"jsonrpc":"2.0","id":4,"method":"get_models","params":{}}
```

Response:
```json
{
  "jsonrpc":"2.0",
  "id":4,
  "result":[]
}
```

##### get_project_info

Get general project information.

Request:
```json
{"jsonrpc":"2.0","id":5,"method":"get_project_info","params":{}}
```

Response:
```json
{
  "jsonrpc":"2.0",
  "id":5,
  "result":{
    "rustwork_version":"0.1.0",
    "project_name":"my-api",
    "database":null,
    "features":[]
  }
}
```

#### Error Responses

When an error occurs, the server returns a JSON-RPC error:

```json
{
  "jsonrpc":"2.0",
  "id":1,
  "error":{
    "code":-32601,
    "message":"Method not found: unknown_method"
  }
}
```

Error codes:
- `-32600` - Invalid Request
- `-32601` - Method Not Found
- `-32602` - Invalid Params
- `-32603` - Internal Error (e.g., manifest not found)

#### Example: Testing with netcat

```bash
# Start the server
rustwork mcp

# In another terminal, connect and send requests
echo '{"jsonrpc":"2.0","id":1,"method":"get_manifest","params":{}}' | nc 127.0.0.1 4000

# Or use telnet
telnet 127.0.0.1 4000
{"jsonrpc":"2.0","id":1,"method":"get_conventions","params":{}}
```

## Project Structure

After `rustwork new my-api`:

```
my-api/
├── Cargo.toml
├── .env.example
├── .gitignore
├── config/
│   ├── default.toml       # Base configuration
│   └── dev.toml           # Development overrides
├── data/                  # SQLite databases (default)
│   └── .gitkeep
├── migrations/            # Database migrations (SQL)
├── src/
│   ├── main.rs           # Entry point
│   ├── app.rs            # Router builder
│   ├── routes.rs         # Route definitions
│   ├── errors.rs         # Custom errors
│   ├── controllers/      # Request handlers
│   │   ├── mod.rs
│   │   └── health.rs
│   ├── models/           # Database entities
│   ├── services/         # Business logic
│   └── middlewares/      # Custom middleware
└── .rustwork/
    └── manifest.json     # Project metadata
```

## Configuration

### Environment Variables (Laravel-style)

```bash
# .env
APP_ENV=dev

# Database - SQLite (default, zero config)
DB_CONNECTION=sqlite
DB_SQLITE_PATH=./data/app.db

# Database - PostgreSQL
# DB_CONNECTION=postgres
# DB_HOST=127.0.0.1
# DB_PORT=5432
# DB_DATABASE=mydb
# DB_USERNAME=postgres
# DB_PASSWORD=secret

# Database - MySQL
# DB_CONNECTION=mysql
# DB_HOST=127.0.0.1
# DB_PORT=3306
# DB_DATABASE=mydb
# DB_USERNAME=root
# DB_PASSWORD=secret

# Or use direct URL (highest priority)
# DB_URL=postgres://user:pass@localhost:5432/dbname

# Server overrides
APP__SERVER__PORT=3000

# CORS Configuration
# APP__CORS__ENABLED=true
# APP__CORS__ALLOWED_ORIGINS=["http://localhost:3000"]
```

### Config Files

```toml
# config/default.toml
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

## Multi-Database Support

Rustwork supports **SQLite, PostgreSQL, and MySQL** out of the box.

### SQLite (Default)

**Zero configuration** - works immediately:

```bash
rustwork new my-api
cd my-api
cargo run  # Uses ./data/app.db automatically
```

### Switch to PostgreSQL

1. Start PostgreSQL (Docker example):

```bash
docker run -d \
  --name postgres \
  -e POSTGRES_PASSWORD=secret \
  -e POSTGRES_DB=mydb \
  -p 5432:5432 \
  postgres:16
```

2. Update `.env`:

```bash
DB_CONNECTION=postgres
DB_DATABASE=mydb
DB_USERNAME=postgres
DB_PASSWORD=secret
```

3. Run:

```bash
cargo run
```

### Switch to MySQL

1. Start MySQL (Docker example):

```bash
docker run -d \
  --name mysql \
  -e MYSQL_ROOT_PASSWORD=secret \
  -e MYSQL_DATABASE=mydb \
  -p 3306:3306 \
  mysql:8
```

2. Update `.env`:

```bash
DB_CONNECTION=mysql
DB_DATABASE=mydb
DB_USERNAME=root
DB_PASSWORD=secret
```

3. Run:

```bash
cargo run
```

## Common Workflows

### 1. Create REST API with SQLite (quickest)

```bash
# Create project
rustwork new blog-api
cd blog-api

# Generate resources
rustwork make controller Post
rustwork make model Post

# Run (SQLite works out of the box)
cargo run
```

### 2. Create REST API with PostgreSQL

```bash
# Create project
rustwork new blog-api
cd blog-api

# Start PostgreSQL
docker-compose up -d  # or use docker run

# Configure .env for PostgreSQL
cp .env.example .env
# Edit DB_CONNECTION=postgres and credentials

# Generate resources
rustwork make controller Post
rustwork make model Post

# Run migrations
rustwork db migrate

# Start server
cargo run
```
sea-orm-cli migrate up

# Start server
rustwork dev
```

### 2. Test API

```bash
# Health check
curl http://localhost:3000/api/health

# Create resource
curl -X POST http://localhost:3000/api/posts \
  -H "Content-Type: application/json" \
  -d '{"title":"Hello","content":"World"}'

# List resources
curl http://localhost:3000/api/posts
```

### 3. Add Custom Routes

Edit `src/routes.rs`:

```rust
pub fn api_routes(state: AppState) -> Router {
    Router::new()
        .route("/api/health", get(health::check))
        .route("/api/posts", get(post::index).post(post::create))
        .route("/api/custom", get(my_handler))
        .with_state(state)
}
```

## Error Handling

All errors implement `AppError`:

```rust
use rustwork::{AppError, AppResult};

pub async fn my_handler() -> AppResult<Json<ApiResponse<Data>>> {
    let item = get_item()
        .ok_or_else(|| AppError::NotFound("Item not found".into()))?;
    
    Ok(ok(item))
}
```

Error types:
- `NotFound` → 404
- `BadRequest` → 400
- `Forbidden` → 403
- `Validation` → 422
- `Conflict` → 409
- `Database` → 500
- `InternalError` → 500

## Response Format

Success:
```json
{
  "success": true,
  "data": { ... },
  "message": "Optional"
}
```

Error:
```json
{
  "success": false,
  "error": "Error message",
  "status": 404
}
```

## Helpers

```rust
use rustwork::{ok, created, error};

// 200 OK
Ok(ok(data))

// 201 Created
Ok(created(data))

// Custom error
Ok(error(StatusCode::BAD_REQUEST, "Invalid input".into()))
```

## Database

```rust
use rustwork::db::Paginator;

let paginator = Paginator::new(page, per_page);
let items = Entity::find()
    .limit(paginator.limit())
    .offset(paginator.offset())
    .all(&db)
    .await?;
```

## Troubleshooting

### cargo-watch not found

```bash
cargo install cargo-watch
```

### Database connection error

Check your `.env` file and ensure PostgreSQL is running:
```bash
APP__DATABASE__URL=postgres://user:pass@localhost/dbname
```

### Compilation errors

```bash
# Clean and rebuild
cargo clean
cargo build
```

## More Information

- Full documentation: [README.md](README.md)
- Examples: [EXAMPLES.md](EXAMPLES.md)
- Contributing: [CONTRIBUTING.md](CONTRIBUTING.md)
- Changelog: [CHANGELOG.md](CHANGELOG.md)

## Support

- Issues: https://github.com/your-org/rustwork/issues
- Discussions: https://github.com/your-org/rustwork/discussions
