mod constants;
mod stdio;
mod tcp;

pub use stdio::run_stdio_server;
pub use tcp::run_server;

#[cfg(test)]
mod tests {
    use crate::mcp::common::server::constants::MAX_LINE_SIZE;

    #[test]
    fn test_max_line_size() {
        assert_eq!(MAX_LINE_SIZE, 1024 * 1024);
    }
}
