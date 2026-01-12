use super::collection::DiagnosticCollection;
use super::types::{Diagnostic, Severity};
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Parse cargo JSON message format
pub(crate) async fn parse_cargo_message(
    line: &str,
    collection: &Arc<RwLock<DiagnosticCollection>>,
) -> Result<()> {
    let value: serde_json::Value = serde_json::from_str(line)?;

    if let Some(diagnostic) = parse_json_diagnostic(&value, "cargo") {
        let mut coll = collection.write().await;
        coll.add(diagnostic);
    }

    Ok(())
}

/// Parse JSON diagnostic from cargo
pub(crate) fn parse_json_diagnostic(value: &serde_json::Value, source: &str) -> Option<Diagnostic> {
    let reason = value.get("reason")?.as_str()?;

    if reason == "compiler-message" {
        let message = value.get("message")?;
        let level = message.get("level")?.as_str()?;

        let severity = match level {
            "error" => Severity::Error,
            "warning" => Severity::Warning,
            "help" => Severity::Help,
            "note" | "info" => Severity::Info,
            _ => return None,
        };

        let msg_text = message.get("message")?.as_str()?.to_string();
        let code = message
            .get("code")
            .and_then(|c| c.get("code"))
            .and_then(|c| c.as_str())
            .map(|s| s.to_string());

        // Extract primary span
        let spans = message.get("spans")?.as_array()?;
        let primary_span = spans.iter().find(|s| {
            s.get("is_primary")
                .and_then(|v| v.as_bool())
                .unwrap_or(false)
        });

        let (file, line, column) = if let Some(span) = primary_span {
            (
                span.get("file_name")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                span.get("line_start")
                    .and_then(|v| v.as_u64())
                    .map(|n| n as usize),
                span.get("column_start")
                    .and_then(|v| v.as_u64())
                    .map(|n| n as usize),
            )
        } else {
            (None, None, None)
        };

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        return Some(Diagnostic {
            severity,
            message: msg_text,
            file,
            line,
            column,
            code,
            source: source.to_string(),
            timestamp,
        });
    }

    None
}

/// Parse text diagnostic from compiler output
#[allow(dead_code)]
pub(crate) fn parse_text_diagnostic(line: &str, source: &str) -> Option<Diagnostic> {
    // Match patterns like:
    // error[E0425]: cannot find value `x` in this scope
    // warning: unused variable: `y`
    // src/main.rs:10:5: error: ...

    let line = line.trim();

    // Pattern 1: error[CODE]: message
    if line.starts_with("error") || line.starts_with("warning") {
        let parts: Vec<&str> = line.splitn(2, ':').collect();
        if parts.len() < 2 {
            return None;
        }

        let severity_part = parts[0];
        let message = parts[1].trim().to_string();

        let (severity, code) = if let Some(code_start) = severity_part.find('[') {
            let code_end = severity_part.find(']')?;
            let code = severity_part[code_start + 1..code_end].to_string();
            let sev = if severity_part.starts_with("error") {
                Severity::Error
            } else if severity_part.starts_with("warning") {
                Severity::Warning
            } else {
                Severity::Info
            };
            (sev, Some(code))
        } else {
            let sev = if severity_part.starts_with("error") {
                Severity::Error
            } else if severity_part.starts_with("warning") {
                Severity::Warning
            } else {
                Severity::Info
            };
            (sev, None)
        };

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        return Some(Diagnostic {
            severity,
            message,
            file: None,
            line: None,
            column: None,
            code,
            source: source.to_string(),
            timestamp,
        });
    }

    // Pattern 2: file:line:col: level: message
    if let Some(arrow_pos) = line.find(" --> ") {
        let location = &line[arrow_pos + 5..];
        let parts: Vec<&str> = location.split(':').collect();

        if parts.len() >= 2 {
            let file = Some(parts[0].to_string());
            let line_num = parts[1].parse::<usize>().ok();
            let column = parts.get(2).and_then(|c| c.parse::<usize>().ok());

            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();

            return Some(Diagnostic {
                severity: Severity::Info,
                message: "See error above".to_string(),
                file,
                line: line_num,
                column,
                code: None,
                source: source.to_string(),
                timestamp,
            });
        }
    }

    None
}
