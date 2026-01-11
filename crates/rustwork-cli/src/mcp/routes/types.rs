use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// HTTP method
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[allow(clippy::upper_case_acronyms)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    HEAD,
    OPTIONS,
}

/// Represents an Axum route
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteInfo {
    pub method: HttpMethod,
    pub path: String,
    pub handler: String,
    pub handler_function: Option<String>,
    pub file: String,
    pub line: usize,
}

/// Route registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteRegistry {
    pub routes: Vec<RouteInfo>,
    pub handler_to_routes: HashMap<String, Vec<usize>>, // handler -> route indices
}

impl RouteRegistry {
    pub fn new() -> Self {
        Self {
            routes: Vec::new(),
            handler_to_routes: HashMap::new(),
        }
    }

    /// Add a route to the registry
    pub fn add_route(&mut self, route: RouteInfo) {
        let handler = route.handler.clone();
        let index = self.routes.len();
        self.routes.push(route);

        self.handler_to_routes
            .entry(handler)
            .or_default()
            .push(index);
    }

    /// Get routes by handler function
    pub fn get_routes_by_handler(&self, handler: &str) -> Vec<&RouteInfo> {
        self.handler_to_routes
            .get(handler)
            .map(|indices| indices.iter().filter_map(|&i| self.routes.get(i)).collect())
            .unwrap_or_default()
    }

    /// Find route by method and path
    pub fn find_route(&self, method: &HttpMethod, path: &str) -> Option<&RouteInfo> {
        self.routes
            .iter()
            .find(|r| r.method == *method && r.path == path)
    }
}
