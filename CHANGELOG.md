# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **gRPC native support** with DSL `.rwk` (Rustwork Contract)
  - Parser DSL pour définir services, RPCs et messages
  - Génération automatique de fichiers `.proto` (dans `target/rustwork/grpc/`)
  - Génération automatique de `build.rs` pour compilation proto
  - Génération de traits Rust ergonomiques (`*Handler`)
  - Génération de serveurs gRPC (via tonic)
  - Génération de clients gRPC
  - Support monorepo/micro-services (scan `services/*/grpc/*.rwk`)
  - Types supportés: string, int, bool, uuid, datetime, optional<T>, list<T>
  - Commande CLI `rustwork grpc build`
  - Export MCP pour introspection des contrats gRPC
  - Documentation complète dans `docs/GRPC.md`
  - Exemples DSL dans `examples/grpc/`
- **Multi-database support** (SQLite, PostgreSQL, MySQL) with zero-config SQLite default
- Laravel-style database configuration via `.env` variables:
  - `DB_CONNECTION` (sqlite/postgres/mysql)
  - `DB_HOST`, `DB_PORT`, `DB_DATABASE`, `DB_USERNAME`, `DB_PASSWORD`
  - `DB_URL` (highest priority)
  - `DB_SQLITE_PATH` for SQLite location
- `DatabaseConfig::resolved_url()` method to build connection strings
- `DatabaseConfig::sanitized_url()` method to mask passwords in logs
- `DbConnection` enum (Sqlite, Postgres, Mysql)
- `PoolConfig` for database connection pool settings
- `connect_db()` function as new standard (replaces `init_database`)
- `/db/info` endpoint for debugging database configuration
- `rustwork db migrate` command for running migrations
- `rustwork db status` command to view migration status
- `rustwork db rollback` command (basic implementation)
- SQL migration support (files in `./migrations/` directory)
- Automatic creation of `./data/` directory for SQLite databases
- Comprehensive tests for database configuration (11 test cases)
- Support for all three backends in SeaORM features

### Changed
- **BREAKING**: Database configuration structure in TOML files
  - Old: `database.url` only
  - New: `database.connection`, `database.sqlite_path`, `database.pool.*`, etc.
- Project templates now default to SQLite with `./data/app.db`
- `.env.example` now includes comprehensive database examples
- Updated documentation (README, QUICKREF) with multi-DB examples
- `rustwork-cli` now depends on `rustwork` crate for DB commands

### Removed
- **GraphQL support** (features and dependencies)
  - Removed `graphql` feature from rustwork
  - Removed `async-graphql` and `async-graphql-axum` dependencies
  - Removed `#[cfg(feature = "graphql")]` blocks
  - Updated templates to remove GraphQL scaffolding

## [0.1.0] - 2026-01-07

### Added

#### Core Framework (rustwork)
- Initial release of Rustwork framework
- `AppConfig` with multi-environment configuration support (.env + TOML profiles)
- `AppError` unified error handling with automatic Axum `IntoResponse` conversion
- `ApiResponse<T>` standard JSON response format
- `AppState` with SeaORM database connection management
- Database connection pooling with SeaORM
- Pagination helper utilities
- Middleware implementations:
  - Request ID tracking
  - CORS with permissive defaults
  - Request tracing integration
- Rust 2021 edition support
- Axum 0.7 as HTTP runtime
- SeaORM 1.0 for database operations

#### CLI Tool (rustwork-cli)
- `rustwork new <name>` - Generate complete project structure
- `rustwork make controller <Name>` - Generate REST controllers with CRUD operations
- `rustwork make model <Name>` - Generate models, services, and migrations
- `rustwork dev` - Development server with hot-reload (requires cargo-watch)
- Template system using minijinja
- Automatic route generation and registration
- Automatic mod.rs updates
- Project manifest (.rustwork/manifest.json) for metadata

#### Project Generation
- Complete project scaffolding with:
  - Main application entry point
  - Router configuration
  - Health check endpoint
  - Configuration files (default.toml, dev.toml)
  - Environment variable examples
  - Controllers, models, services, middlewares directories
  - Migrations directory
  - .gitignore template

#### Documentation
- Comprehensive README with examples and API reference
- EXAMPLES.md with 5 practical examples:
  - Simple REST API
  - Custom middleware
  - Database pagination
  - Custom configuration
  - Error handling patterns
- CONTRIBUTING.md with development guidelines
- VSCode configuration documentation
- Inline code documentation

#### Features
- Optional GraphQL support via `graphql` feature flag
- Optional OpenTelemetry support via `otel` feature flag
- Workspace-based project structure
- Path-relative dependencies for generated projects

### Changed
- N/A (initial release)

### Deprecated
- N/A (initial release)

### Removed
- N/A (initial release)

### Fixed
- N/A (initial release)

### Security
- N/A (initial release)

## Roadmap

### [0.2.0] - Planned
- [ ] Full GraphQL integration with examples
- [ ] OpenTelemetry tracing implementation
- [ ] Test generation commands
- [ ] MCP (Model Context Protocol) server for IDE integration
- [ ] JWT authentication helpers
- [ ] Rate limiting middleware

### [0.3.0] - Planned
- [ ] Multi-database support (MySQL, SQLite)
- [ ] WebSocket support
- [ ] File upload helpers
- [ ] Email sending utilities
- [ ] Background job processing
- [ ] CLI interactive mode

### Future Considerations
- Custom template support
- Plugin system
- Admin panel generator
- Docker integration
- Kubernetes deployment templates
- API documentation generator (OpenAPI/Swagger)

[Unreleased]: https://github.com/your-org/rustwork/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/your-org/rustwork/releases/tag/v0.1.0
