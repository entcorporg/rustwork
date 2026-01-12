use super::constants::MAX_LINE_SIZE;
use crate::mcp::common::dispatcher::handle_request;
use crate::mcp::common::protocol::{RpcError, RpcRequest, RpcResponse};
use crate::mcp::common::state::LiveProjectState;
use anyhow::{Context, Result};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

/// Run the MCP server
pub async fn run_server(host: &str, port: u16, project_path: PathBuf) -> Result<()> {
    // Validate that we're binding to localhost only
    if host != "127.0.0.1" && host != "localhost" {
        anyhow::bail!("MCP server can only bind to localhost (127.0.0.1) for security reasons");
    }

    let addr = format!("{}:{}", host, port);
    let listener = TcpListener::bind(&addr)
        .await
        .context(format!("Failed to bind to {}", addr))?;

    println!("🚀 MCP server listening on {}", addr);
    println!("📁 Project path: {}", project_path.display());

    // Initialize live project state with workspace root detection
    let state = match LiveProjectState::new(project_path.clone()) {
        Ok(s) => Arc::new(s),
        Err(e) => {
            eprintln!("❌ Failed to detect workspace root: {}", e);
            return Err(e);
        }
    };

    // Perform initial scan
    println!();
    state.initial_scan().await?;
    println!();

    // Start file watcher
    state.start_watching().await?;

    // Start diagnostics collector
    state.start_diagnostics_collector().await?;

    println!("✨ MCP server ready!");
    println!("Press Ctrl+C to stop\n");

    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                let project_path = project_path.clone();
                let state = Arc::clone(&state);
                tokio::spawn(async move {
                    if let Err(e) = handle_connection(stream, project_path, state).await {
                        eprintln!("Error handling connection from {}: {}", addr, e);
                    }
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}

/// Handle a single TCP connection
async fn handle_connection(
    stream: TcpStream,
    project_path: PathBuf,
    state: Arc<LiveProjectState>,
) -> Result<()> {
    let peer_addr = stream.peer_addr()?;
    println!("📥 New connection from {}", peer_addr);

    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    loop {
        line.clear();

        // Read one line (newline-delimited JSON)
        let bytes_read = reader.read_line(&mut line).await?;

        // Connection closed
        if bytes_read == 0 {
            println!("📤 Connection closed: {}", peer_addr);
            break;
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
            send_response(&mut writer, &error_response).await?;
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
                send_response(&mut writer, &error_response).await?;
                continue;
            }
        };

        // Validate JSON-RPC version
        if request.jsonrpc != "2.0" {
            let error_response = RpcResponse::error(
                request.id.clone(),
                RpcError::invalid_request("jsonrpc must be '2.0'"),
            );
            send_response(&mut writer, &error_response).await?;
            continue;
        }

        println!("📨 Request: {} (id: {:?})", request.method, request.id);

        // Handle the request
        let response = handle_request(request, &project_path, Some(&state)).await;

        // Send response
        send_response(&mut writer, &response).await?;
    }

    Ok(())
}

/// Send a JSON-RPC response over the stream
async fn send_response(
    writer: &mut tokio::net::tcp::OwnedWriteHalf,
    response: &RpcResponse,
) -> Result<()> {
    let json = serde_json::to_string(response).context("Failed to serialize response")?;

    writer.write_all(json.as_bytes()).await?;
    writer.write_all(b"\n").await?;
    writer.flush().await?;

    Ok(())
}
