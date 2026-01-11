use super::types::{HttpMethod, RouteInfo};
use syn::visit::Visit;

/// Visitor to extract route definitions from Axum router code
pub(crate) struct RouteVisitor {
    pub routes: Vec<RouteInfo>,
    current_file: String,
}

impl RouteVisitor {
    pub fn new(file: String) -> Self {
        Self {
            routes: Vec::new(),
            current_file: file,
        }
    }

    /// Try to extract route info from a method call expression
    fn extract_route_from_expr(&mut self, expr: &syn::Expr) {
        if let syn::Expr::MethodCall(method_call) = expr {
            let method_name = method_call.method.to_string();

            // Check if this is a route registration method (get, post, etc.)
            let http_method = match method_name.as_str() {
                "get" => Some(HttpMethod::GET),
                "post" => Some(HttpMethod::POST),
                "put" => Some(HttpMethod::PUT),
                "patch" => Some(HttpMethod::PATCH),
                "delete" => Some(HttpMethod::DELETE),
                "head" => Some(HttpMethod::HEAD),
                "options" => Some(HttpMethod::OPTIONS),
                _ => None,
            };

            if let Some(method) = http_method {
                // Extract path (first argument)
                if let Some(syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(lit_str),
                    ..
                })) = method_call.args.first()
                {
                    let path = lit_str.value();

                    // Extract handler (second argument if present)
                    let handler = if let Some(handler_arg) = method_call.args.iter().nth(1) {
                        self.extract_handler_name(handler_arg)
                    } else {
                        "unknown".to_string()
                    };

                    let line = 0; // proc_macro2::Span doesn't provide easy line access

                    self.routes.push(RouteInfo {
                        method,
                        path,
                        handler: handler.clone(),
                        handler_function: Some(handler),
                        file: self.current_file.clone(),
                        line,
                    });
                }
            }

            // Also check for route() method which takes a path and method
            if method_name == "route" {
                if let Some(syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(lit_str),
                    ..
                })) = method_call.args.first()
                {
                    let path = lit_str.value();

                    // Second arg should be method_routing::get(handler) or similar
                    if let Some(syn::Expr::Call(call_expr)) = method_call.args.iter().nth(1) {
                        if let syn::Expr::Path(path_expr) = &*call_expr.func {
                            if let Some(segment) = path_expr.path.segments.last() {
                                let method_str = segment.ident.to_string();
                                if let Some(method) = self.parse_http_method(&method_str) {
                                    let handler = if let Some(handler_arg) = call_expr.args.first()
                                    {
                                        self.extract_handler_name(handler_arg)
                                    } else {
                                        "unknown".to_string()
                                    };

                                    let line = 0; // proc_macro2::Span doesn't provide easy line access

                                    self.routes.push(RouteInfo {
                                        method,
                                        path,
                                        handler: handler.clone(),
                                        handler_function: Some(handler),
                                        file: self.current_file.clone(),
                                        line,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn parse_http_method(&self, s: &str) -> Option<HttpMethod> {
        match s {
            "get" => Some(HttpMethod::GET),
            "post" => Some(HttpMethod::POST),
            "put" => Some(HttpMethod::PUT),
            "patch" => Some(HttpMethod::PATCH),
            "delete" => Some(HttpMethod::DELETE),
            "head" => Some(HttpMethod::HEAD),
            "options" => Some(HttpMethod::OPTIONS),
            _ => None,
        }
    }

    fn extract_handler_name(&self, expr: &syn::Expr) -> String {
        match expr {
            syn::Expr::Path(path_expr) => path_expr
                .path
                .segments
                .last()
                .map(|s| s.ident.to_string())
                .unwrap_or_else(|| "unknown".to_string()),
            _ => "unknown".to_string(),
        }
    }
}

impl<'ast> Visit<'ast> for RouteVisitor {
    fn visit_expr(&mut self, expr: &'ast syn::Expr) {
        self.extract_route_from_expr(expr);
        syn::visit::visit_expr(self, expr);
    }

    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        // Check if this is a router building function
        let fn_name = node.sig.ident.to_string();
        if fn_name.contains("router") || fn_name.contains("routes") {
            syn::visit::visit_item_fn(self, node);
        }
    }
}
