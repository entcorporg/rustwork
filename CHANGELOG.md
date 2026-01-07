# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
