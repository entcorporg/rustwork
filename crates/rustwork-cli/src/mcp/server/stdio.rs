use super::constants::MAX_LINE_SIZE;
use crate::mcp::dispatcher::handle_request;
use crate::mcp::protocol::{RpcError, RpcRequest, RpcResponse};
use crate::mcp::state::LiveProjectState;
use anyhow::{Context, Result};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};

/// Run the MCP server with stdio transport (for VS Code integration)
pub async fn run_stdio_server(project_path: PathBuf) -> Result<()> {
    // Initialize live project state with workspace root detection
    let state = match LiveProjectState::new(project_path.clone()) {
        Ok(s) => Arc::new(s),
        Err(e) => {
            eprintln!("❌ Failed to detect workspace root: {}", e);
            eprintln!("💡 Make sure the project has one of:");
            eprintln!("   - .rustwork/ marker directory");
            eprintln!("   - services/ directory (microservices)");
            eprintln!("   - src/ directory (monolith)");
            return Err(e);
        }
    };

    // Log to stderr only (stdout is reserved for JSON-RPC)
    eprintln!("🚀 MCP server starting (stdio mode)");
    eprintln!("📁 Project path: {}", project_path.display());

    // Perform initial scan (log to stderr)
    match state.initial_scan_quiet().await {
        Ok(_) => eprintln!("✨ MCP server ready!"),
        Err(e) => eprintln!("⚠️  Initial scan error: {}", e),
    }

    // Start file watcher
    if let Err(e) = state.start_watching().await {
        eprintln!("⚠️  File watcher error: {}", e);
    }

    // Start diagnostics collector
    if let Err(e) = state.start_diagnostics_collector().await {
        eprintln!("⚠️  Diagnostics collector error: {}", e);
    }

    // Read from stdin, write to stdout
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut reader = BufReader::new(stdin);
    let mut line = String::new();

    loop {
        line.clear();

        // Read one line (newline-delimited JSON)
        match reader.read_line(&mut line).await {
            Ok(0) => {
                // EOF - connection closed
                eprintln!("📤 Connection closed (EOF)");
                break;
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("❌ Read error: {}", e);
                break;
            }
        }

        // Check line size limit
        if line.len() > MAX_LINE_SIZE {
            let error_response = RpcResponse::error(
                None,
                RpcError::invalid_request(format!(
                    "Request too large: {} bytes (max {})",
                    line.len(),
                    MAX_LINE_SIZE
                )),
            );
            if let Err(e) = send_stdout_response(&mut stdout, &error_response).await {
                eprintln!("❌ Failed to send error response: {}", e);
            }
            continue;
        }

        // Trim whitespace
        let line_trimmed = line.trim();
        if line_trimmed.is_empty() {
            continue;
        }

        // Parse JSON-RPC request
        let request: RpcRequest = match serde_json::from_str(line_trimmed) {
            Ok(req) => req,
            Err(e) => {
                let error_response = RpcResponse::error(
                    None,
                    RpcError::invalid_request(format!("Invalid JSON: {}", e)),
                );
                if let Err(e) = send_stdout_response(&mut stdout, &error_response).await {
                    eprintln!("❌ Failed to send error response: {}", e);
                }
                continue;
            }
        };

        // Validate JSON-RPC version
        if request.jsonrpc != "2.0" {
            let error_response = RpcResponse::error(
                request.id.clone(),
                RpcError::invalid_request("jsonrpc must be '2.0'"),
            );
            if let Err(e) = send_stdout_response(&mut stdout, &error_response).await {
                eprintln!("❌ Failed to send error response: {}", e);
            }
            continue;
        }

        eprintln!("📨 Request: {} (id: {:?})", request.method, request.id);

        // Handle the request
        let response = handle_request(request, &project_path, Some(&state)).await;

        // Send response to stdout
        if let Err(e) = send_stdout_response(&mut stdout, &response).await {
            eprintln!("❌ Failed to send response: {}", e);
        }
    }

    Ok(())
}

/// Send a JSON-RPC response to stdout
async fn send_stdout_response(stdout: &mut io::Stdout, response: &RpcResponse) -> Result<()> {
    let json = serde_json::to_string(response).context("Failed to serialize response")?;

    stdout.write_all(json.as_bytes()).await?;
    stdout.write_all(b"\n").await?;
    stdout.flush().await?;

    Ok(())
}
