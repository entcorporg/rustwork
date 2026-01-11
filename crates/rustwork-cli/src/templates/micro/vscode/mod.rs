pub const VSCODE_MCP_JSON: &str = r#"{
  "servers": {
    "rustwork": {
      "type": "stdio",
      "command": "rustwork",
      "args": ["mcp", "--stdio", "--project", "{{ project_path }}"]
    }
  }
}
"#;

pub const VSCODE_SETTINGS_JSON: &str = r#"{
  "files.watcherExclude": {
    "**/target": true,
    "**/.rustwork": true
  }
}
"#;
