# Rustwork Examples

## Example 1: Simple REST API

### Create a new project

```bash
rustwork new blog-api
cd blog-api
```

### Generate a Post controller

```bash
rustwork make controller Post
```

This creates:
- `src/controllers/post.rs` with CRUD methods
- Routes automatically added to `src/routes.rs`

### Generate the Post model

```bash
rustwork make model Post
```

This creates:
- `src/models/post.rs` (SeaORM entity)
- `src/services/post_service.rs` (business logic)
- `migrations/<timestamp>_create_posts.rs`

### Customize the model

Edit `src/models/post.rs`:

```rust
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "posts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    pub content: String,
    pub published: bool,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
```

### Implement the controller logic

Edit `src/controllers/post.rs`:

```rust
use axum::{extract::{State, Path}, Json};
use rustwork::{AppState, AppResult, ApiResponse, ok, created};
use serde::{Deserialize, Serialize};
use crate::services::post_service::PostService;

#[derive(Debug, Serialize)]
pub struct PostResponse {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub published: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
    pub published: bool,
}

/// GET /api/posts
pub async fn index(
    State(state): State<AppState>,
) -> AppResult<(axum::http::StatusCode, Json<ApiResponse<Vec<PostResponse>>>)> {
    let service = PostService::new(state.db.clone());
    let posts = service.find_all().await?;
    
    let responses: Vec<PostResponse> = posts
        .into_iter()
        .map(|p| PostResponse {
            id: p.id,
            title: p.title,
            content: p.content,
            published: p.published,
        })
        .collect();
    
    Ok(ok(responses))
}

/// POST /api/posts
pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreatePostRequest>,
) -> AppResult<(axum::http::StatusCode, Json<ApiResponse<PostResponse>>)> {
    let service = PostService::new(state.db.clone());
    let post = service.create(payload).await?;
    
    let response = PostResponse {
        id: post.id,
        title: post.title,
        content: post.content,
        published: post.published,
    };
    
    Ok(created(response))
}

// ... implement other methods
```

### Configure the database

Edit `.env`:

```bash
APP_ENV=dev
APP__DATABASE__URL=postgres://postgres:postgres@localhost/blog_db
```

### Run migrations

```bash
# Install sea-orm-cli if not already installed
cargo install sea-orm-cli

# Run migrations
sea-orm-cli migrate up
```

### Start the server

```bash
rustwork dev
```

### Test the API

```bash
# Health check
curl http://localhost:3000/api/health

# Create a post
curl -X POST http://localhost:3000/api/posts \
  -H "Content-Type: application/json" \
  -d '{"title":"My First Post","content":"Hello World","published":true}'

# Get all posts
curl http://localhost:3000/api/posts
```

## Example 2: Custom Middleware

Create `src/middlewares/auth.rs`:

```rust
use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use rustwork::AppError;

pub async fn auth_middleware(
    req: Request,
    next: Next,
) -> Result<Response, AppError> {
    // Extract token from header
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "));

    match token {
        Some(token) => {
            // Validate token here
            if token == "valid-token" {
                Ok(next.run(req).await)
            } else {
                Err(AppError::Unauthorized("Invalid token".to_string()))
            }
        }
        None => Err(AppError::Unauthorized("Missing token".to_string())),
    }
}
```

Add to `src/routes.rs`:

```rust
use axum::middleware as axum_middleware;
use crate::middlewares::auth;

pub fn api_routes(state: AppState) -> Router {
    Router::new()
        .route("/api/health", get(health::check))
        .route("/api/posts", 
            get(post::index).post(post::create)
                .layer(axum_middleware::from_fn(auth::auth_middleware))
        )
        .with_state(state)
}
```

## Example 3: Database Pagination

In `src/services/post_service.rs`:

```rust
use rustwork::{AppResult, db::Paginator};
use sea_orm::{DatabaseConnection, EntityTrait, QueryOrder};
use crate::models::post;

pub struct PostService {
    db: DatabaseConnection,
}

impl PostService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_paginated(
        &self,
        page: u64,
        per_page: u64,
    ) -> AppResult<(Vec<post::Model>, u64)> {
        let paginator = Paginator::new(page, per_page);
        
        // Get total count
        let total = post::Entity::find().count(&self.db).await?;
        
        // Get paginated results
        let posts = post::Entity::find()
            .order_by_desc(post::Column::CreatedAt)
            .limit(paginator.limit())
            .offset(paginator.offset())
            .all(&self.db)
            .await?;
        
        Ok((posts, total))
    }
}
```

Use in controller:

```rust
#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    #[serde(default = "default_page")]
    page: u64,
    #[serde(default = "default_per_page")]
    per_page: u64,
}

fn default_page() -> u64 { 1 }
fn default_per_page() -> u64 { 20 }

pub async fn index(
    State(state): State<AppState>,
    Query(params): Query<PaginationQuery>,
) -> AppResult<(StatusCode, Json<ApiResponse<PaginatedResponse>>)> {
    let service = PostService::new(state.db.clone());
    let (posts, total) = service.find_paginated(params.page, params.per_page).await?;
    
    let response = PaginatedResponse {
        items: posts,
        total,
        page: params.page,
        per_page: params.per_page,
    };
    
    Ok(ok(response))
}
```

## Example 4: Custom Configuration

Add custom config in `config/default.toml`:

```toml
[server]
host = "0.0.0.0"
port = 3000

[database]
url = "postgres://postgres:postgres@localhost/mydb"
max_connections = 10
min_connections = 2

[auth]
jwt_secret = "your-secret-key"
jwt_expiration = 86400

[custom]
upload_path = "./uploads"
max_file_size = 10485760  # 10MB
```

Extend AppConfig in `src/main.rs`:

```rust
use rustwork::AppConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtendedConfig {
    #[serde(flatten)]
    pub base: AppConfig,
    pub custom: CustomConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomConfig {
    pub upload_path: String,
    pub max_file_size: usize,
}
```

## Example 5: Error Handling

Custom error handling:

```rust
use rustwork::{AppError, AppResult};

pub async fn risky_operation() -> AppResult<String> {
    // Database operation
    let result = fetch_from_db()
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
    
    // Validation
    if result.is_empty() {
        return Err(AppError::Validation("Result cannot be empty".to_string()));
    }
    
    // Business logic error
    if !result.starts_with("valid_") {
        return Err(AppError::BadRequest("Invalid prefix".to_string()));
    }
    
    Ok(result)
}
```

All errors are automatically converted to proper HTTP responses with JSON format.

## More Examples

Check the `test-api` directory in the repository for a complete working example with:
- User controller
- Post model and service
- Migrations
- Full project structure
