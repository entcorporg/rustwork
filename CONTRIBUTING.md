# Contributing to Rustwork

Thank you for your interest in contributing to Rustwork! This document provides guidelines and instructions for contributing.

## Development Setup

### Prerequisites

- Rust 1.70+ (stable)
- PostgreSQL (for testing)
- Git

### Clone and Build

```bash
git clone https://github.com/your-org/rustwork.git
cd rustwork
cargo build --workspace
```

### Run Tests

```bash
cargo test --workspace
```

### Install CLI locally

```bash
cargo install --path crates/rustwork-cli
```

## Project Structure

```
rustwork/
├── crates/
│   ├── rustwork/           # Core framework library
│   │   ├── src/
│   │   │   ├── lib.rs      # Public exports
│   │   │   ├── config.rs   # Configuration management
│   │   │   ├── errors.rs   # Error types
│   │   │   ├── response.rs # API response helpers
│   │   │   ├── app.rs      # Router builder
│   │   │   ├── state.rs    # Application state
│   │   │   ├── db.rs       # Database utilities
│   │   │   └── middleware.rs # Middleware implementations
│   │   └── Cargo.toml
│   │
│   └── rustwork-cli/       # CLI tool
│       ├── src/
│       │   ├── main.rs
│       │   ├── commands/
│       │   │   ├── new.rs          # Project generation
│       │   │   ├── make_controller.rs
│       │   │   ├── make_model.rs
│       │   │   ├── dev.rs          # Dev server
│       │   │   └── utils.rs
│       │   └── templates/
│       │       ├── mod.rs
│       │       └── project.rs      # Template strings
│       └── Cargo.toml
│
├── README.md
├── EXAMPLES.md
└── CONTRIBUTING.md
```

## Code Guidelines

### Rust Style

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` for formatting: `cargo fmt --all`
- Use `clippy` for linting: `cargo clippy --all`
- Write documentation for public APIs

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add GraphQL support
fix: handle database connection errors properly
docs: update README with examples
refactor: simplify error handling
test: add integration tests for controllers
```

### Pull Requests

1. Fork the repository
2. Create a feature branch: `git checkout -b feat/my-feature`
3. Make your changes
4. Add tests if applicable
5. Run tests: `cargo test --workspace`
6. Run clippy: `cargo clippy --all`
7. Commit with conventional commits
8. Push and create a PR

## Adding Features

### Adding a New CLI Command

1. Create a new file in `crates/rustwork-cli/src/commands/`
2. Implement the command handler
3. Add the command to `main.rs` enum
4. Update documentation

Example:

```rust
// crates/rustwork-cli/src/commands/my_command.rs
use anyhow::Result;

pub async fn execute(args: &Args) -> Result<()> {
    println!("Executing my command");
    Ok(())
}
```

### Adding a New Template

1. Add template string to `crates/rustwork-cli/src/templates/project.rs`
2. Register template in `templates/mod.rs`
3. Use template in command handler

Example:

```rust
pub const MY_TEMPLATE: &str = r#"
// Generated code
pub fn my_function() {
    println!("{{ message }}");
}
"#;
```

### Extending Core Framework

1. Add new module to `crates/rustwork/src/`
2. Export public APIs in `lib.rs`
3. Add tests
4. Update documentation

## Testing

### Unit Tests

Place unit tests in the same file:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        assert_eq!(2 + 2, 4);
    }
}
```

### Integration Tests

Create files in `tests/` directory:

```rust
// tests/integration_test.rs
#[tokio::test]
async fn test_full_flow() {
    // Test code
}
```

### Testing CLI Commands

Generate a test project and verify:

```bash
cargo build --release
./target/release/rustwork new test-project
cd test-project
cargo check
cargo test
```

## Documentation

- Update README.md for major features
- Add examples to EXAMPLES.md
- Write inline documentation with `///`
- Generate docs: `cargo doc --open`

## Release Process

1. Update version in `Cargo.toml` files
2. Update CHANGELOG.md
3. Create git tag: `git tag v0.1.0`
4. Push tag: `git push origin v0.1.0`
5. GitHub Actions will handle the release

## Getting Help

- Open an issue for bugs or feature requests
- Start a discussion for questions
- Join our community chat (if applicable)

## Code of Conduct

Be respectful and inclusive. We follow the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
