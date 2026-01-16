#[allow(dead_code)]
pub const VSCODE_MCP_JSON: &str = r#"{
  "servers": {
    "rustwork": {
      "type": "stdio",
      "command": "rustwork",
      "args": ["mcp", "--stdio", "--project", "{{ project_path }}"]
    },
  "inputs": []
  }
}
"#;
