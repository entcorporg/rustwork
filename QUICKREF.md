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
├── migrations/            # Database migrations
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
│   ├── middlewares/      # Custom middleware
│   └── graphql/          # GraphQL schema
└── .rustwork/
    └── manifest.json     # Project metadata
```

## Configuration

### Environment Variables

```bash
# .env
APP_ENV=dev
APP__SERVER__PORT=3000
APP__DATABASE__URL=postgres://user:pass@localhost/dbname
APP__AUTH__JWT_SECRET=your-secret
```

### Config Files

```toml
# config/default.toml
[server]
host = "0.0.0.0"
port = 3000

[database]
url = "postgres://postgres:postgres@localhost/mydb"
max_connections = 10
min_connections = 2

[auth]
jwt_secret = "change-me"
jwt_expiration = 86400
```

## Common Workflows

### 1. Create REST API

```bash
# Create project
rustwork new blog-api
cd blog-api

# Generate resources
rustwork make controller Post
rustwork make model Post

# Configure database
cp .env.example .env
# Edit .env with your database URL

# Run migrations (requires sea-orm-cli)
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
- `Unauthorized` → 401
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
