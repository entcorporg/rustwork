mod scanner;
mod types;
mod visitor;

pub use scanner::scan_routes;
pub use types::{HttpMethod, RouteRegistry};

#[cfg(test)]
mod tests {
    use super::types::{HttpMethod, RouteInfo, RouteRegistry};
    use super::visitor::RouteVisitor;
    use syn::{visit::Visit, File};

    #[test]
    fn test_parse_simple_route() {
        let code = r#"
            use axum::{Router, routing::get};

            pub fn build_router() -> Router {
                Router::new()
                    .route("/users", get(list_users))
                    .route("/users/:id", get(get_user))
            }
        "#;

        let syntax_tree: File = syn::parse_str(code).unwrap();
        let mut visitor = RouteVisitor::new("test.rs".to_string());
        visitor.visit_file(&syntax_tree);

        assert_eq!(visitor.routes.len(), 2);
        assert_eq!(visitor.routes[0].path, "/users");
        assert_eq!(visitor.routes[0].method, HttpMethod::GET);
        assert_eq!(visitor.routes[0].handler, "list_users");
    }

    #[test]
    fn test_route_registry() {
        let mut registry = RouteRegistry::new();

        registry.add_route(RouteInfo {
            method: HttpMethod::GET,
            path: "/users".to_string(),
            handler: "list_users".to_string(),
            handler_function: Some("list_users".to_string()),
            file: "routes.rs".to_string(),
            line: 10,
        });

        let routes = registry.get_routes_by_handler("list_users");
        assert_eq!(routes.len(), 1);
        assert_eq!(routes[0].path, "/users");
    }
}
