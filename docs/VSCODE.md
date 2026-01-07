# VSCode Configuration

Recommended VSCode settings for Rustwork development.

## settings.json

Create `.vscode/settings.json`:

```json
{
  "rust-analyzer.linkedProjects": [
    "./Cargo.toml"
  ],
  "rust-analyzer.cargo.features": "all",
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.checkOnSave.allTargets": true,
  "editor.formatOnSave": true,
  "editor.defaultFormatter": "rust-lang.rust-analyzer",
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer",
    "editor.formatOnSave": true
  },
  "files.exclude": {
    "**/target": true,
    "**/.rustwork": true
  },
  "search.exclude": {
    "**/target": true,
    "**/Cargo.lock": true
  }
}
```

## tasks.json

Create `.vscode/tasks.json`:

```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "Build Workspace",
      "type": "shell",
      "command": "cargo",
      "args": ["build", "--workspace"],
      "group": {
        "kind": "build",
        "isDefault": true
      },
      "problemMatcher": ["$rustc"]
    },
    {
      "label": "Build Release",
      "type": "shell",
      "command": "cargo",
      "args": ["build", "--workspace", "--release"],
      "group": "build",
      "problemMatcher": ["$rustc"]
    },
    {
      "label": "Test Workspace",
      "type": "shell",
      "command": "cargo",
      "args": ["test", "--workspace"],
      "group": {
        "kind": "test",
        "isDefault": true
      },
      "problemMatcher": ["$rustc"]
    },
    {
      "label": "Clippy",
      "type": "shell",
      "command": "cargo",
      "args": ["clippy", "--workspace", "--all-targets"],
      "problemMatcher": ["$rustc"]
    },
    {
      "label": "Generate Test Project",
      "type": "shell",
      "command": "rm -rf test-api && cargo build --release --bin rustwork && ./target/release/rustwork new test-api"
    }
  ]
}
```

## Recommended Extensions

- rust-analyzer
- CodeLLDB (for debugging)
- crates (dependency management)
- Better TOML
- Error Lens
