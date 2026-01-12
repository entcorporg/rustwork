/// Testeur de connectivité gRPC
use super::types::*;
use std::net::TcpStream;
use std::time::{Duration, Instant};

/// Testeur de connectivité gRPC
pub struct ConnectivityTester {
    config: TestConfig,
}

impl ConnectivityTester {
    pub fn new(config: TestConfig) -> Self {
        Self { config }
    }

    /// Teste la connectivité vers un service gRPC
    ///
    /// IMPORTANT :
    /// - Timeout court (5s par défaut)
    /// - Aucun retry
    /// - Aucun panic possible
    /// - Erreurs claires
    pub fn test(&self, service_name: &str, address: &str) -> ConnectivityTestResult {
        let start = Instant::now();

        // Parser l'adresse
        let socket_addr = match address.parse::<std::net::SocketAddr>() {
            Ok(addr) => addr,
            Err(e) => {
                return ConnectivityTestResult {
                    service_name: service_name.to_string(),
                    target_address: address.to_string(),
                    status: ConnectivityStatus::Failed,
                    latency_ms: None,
                    error: Some(format!("Invalid address: {}", e)),
                };
            }
        };

        // Tester la connexion TCP (sans appel gRPC complet pour simplifier)
        let result = self.test_tcp_connection(socket_addr);

        let latency = start.elapsed().as_millis() as u64;

        match result {
            Ok(()) => ConnectivityTestResult {
                service_name: service_name.to_string(),
                target_address: address.to_string(),
                status: ConnectivityStatus::Connected,
                latency_ms: Some(latency),
                error: None,
            },
            Err(e) => {
                let status = if e.contains("timeout") {
                    ConnectivityStatus::Timeout
                } else {
                    ConnectivityStatus::Failed
                };

                ConnectivityTestResult {
                    service_name: service_name.to_string(),
                    target_address: address.to_string(),
                    status,
                    latency_ms: Some(latency),
                    error: Some(e),
                }
            }
        }
    }

    /// Teste la connexion TCP brute
    fn test_tcp_connection(&self, addr: std::net::SocketAddr) -> Result<(), String> {
        let timeout = Duration::from_millis(self.config.timeout_ms);

        match TcpStream::connect_timeout(&addr, timeout) {
            Ok(_) => Ok(()),
            Err(e) => {
                if e.kind() == std::io::ErrorKind::TimedOut {
                    Err(format!(
                        "Connection timeout after {}ms",
                        self.config.timeout_ms
                    ))
                } else {
                    Err(format!("Connection failed: {}", e))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_address() {
        let tester = ConnectivityTester::new(TestConfig::default());
        let result = tester.test("TestService", "invalid-address");

        assert!(matches!(result.status, ConnectivityStatus::Failed));
        assert!(result.error.is_some());
    }

    #[test]
    fn test_unreachable_service() {
        let tester = ConnectivityTester::new(TestConfig {
            timeout_ms: 1000, // 1s pour accélérer le test
        });

        // Adresse valide mais service inexistant
        let result = tester.test("TestService", "127.0.0.1:59999");

        assert!(matches!(
            result.status,
            ConnectivityStatus::Failed | ConnectivityStatus::Timeout
        ));
    }
}
