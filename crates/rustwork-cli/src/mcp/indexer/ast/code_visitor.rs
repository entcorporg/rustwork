use super::super::types::{FieldInfo, FunctionInfo, Parameter, StructInfo};
use syn::{
    spanned::Spanned,
    visit::{self, Visit},
    ItemFn, ItemImpl, Visibility,
};

/// Visitor for extracting information from AST
pub struct CodeVisitor {
    pub functions: Vec<FunctionInfo>,
    pub structs: Vec<StructInfo>,
    pub current_function_calls: Vec<String>,
}

impl CodeVisitor {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
            structs: Vec::new(),
            current_function_calls: Vec::new(),
        }
    }

    fn is_public(vis: &Visibility) -> bool {
        matches!(vis, Visibility::Public(_))
    }

    fn extract_type_string(ty: &syn::Type) -> String {
        quote::quote!(#ty).to_string()
    }

    fn extract_return_type(output: &syn::ReturnType) -> Option<String> {
        match output {
            syn::ReturnType::Default => None,
            syn::ReturnType::Type(_, ty) => Some(Self::extract_type_string(ty)),
        }
    }
}

impl<'ast> Visit<'ast> for CodeVisitor {
    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        let is_public = Self::is_public(&node.vis);
        let is_async = node.sig.asyncness.is_some();

        // Extract line numbers from span using syn::spanned::Spanned
        // CRITICAL: These must be exact, non-zero line numbers
        let span = node.span();
        let start_line = span.start().line;
        let end_line = span.end().line;

        // Validation: line numbers must be valid
        if start_line == 0 || end_line == 0 {
            // Skip functions with invalid spans - better to exclude than give wrong data
            eprintln!(
                "Warning: Function '{}' has invalid span (start={}, end={}), skipping",
                node.sig.ident, start_line, end_line
            );
            return;
        }

        if end_line < start_line {
            eprintln!(
                "Warning: Function '{}' has invalid span (end < start), skipping",
                node.sig.ident
            );
            return;
        }

        let signature = quote::quote!(#node).to_string();

        let parameters: Vec<Parameter> = node
            .sig
            .inputs
            .iter()
            .filter_map(|arg| {
                if let syn::FnArg::Typed(pat_type) = arg {
                    if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                        return Some(Parameter {
                            name: pat_ident.ident.to_string(),
                            type_name: Self::extract_type_string(&pat_type.ty),
                        });
                    }
                }
                None
            })
            .collect();

        let return_type = Self::extract_return_type(&node.sig.output);

        // Visit function body to find calls
        let old_calls = std::mem::take(&mut self.current_function_calls);
        visit::visit_item_fn(self, node);
        let calls = std::mem::replace(&mut self.current_function_calls, old_calls);

        self.functions.push(FunctionInfo {
            name: node.sig.ident.to_string(),
            is_public,
            is_async,
            start_line,
            end_line,
            signature,
            calls,
            parameters,
            return_type,
        });
    }

    fn visit_item_struct(&mut self, node: &'ast syn::ItemStruct) {
        let is_public = Self::is_public(&node.vis);

        // Extract line numbers from span using syn::spanned::Spanned
        // CRITICAL: These must be exact, non-zero line numbers
        let span = node.span();
        let start_line = span.start().line;
        let end_line = span.end().line;

        // Validation: line numbers must be valid
        if start_line == 0 || end_line == 0 {
            eprintln!(
                "Warning: Struct '{}' has invalid span (start={}, end={}), skipping",
                node.ident, start_line, end_line
            );
            return;
        }

        if end_line < start_line {
            eprintln!(
                "Warning: Struct '{}' has invalid span (end < start), skipping",
                node.ident
            );
            return;
        }

        let fields: Vec<FieldInfo> = node
            .fields
            .iter()
            .filter_map(|field| {
                field.ident.as_ref().map(|ident| FieldInfo {
                    name: ident.to_string(),
                    type_name: Self::extract_type_string(&field.ty),
                    is_public: Self::is_public(&field.vis),
                })
            })
            .collect();

        self.structs.push(StructInfo {
            name: node.ident.to_string(),
            is_public,
            start_line,
            end_line,
            fields,
        });

        visit::visit_item_struct(self, node);
    }

    fn visit_expr_call(&mut self, node: &'ast syn::ExprCall) {
        // Extract function name from call expression
        if let syn::Expr::Path(expr_path) = &*node.func {
            if let Some(ident) = expr_path.path.segments.last() {
                self.current_function_calls.push(ident.ident.to_string());
            }
        }
        visit::visit_expr_call(self, node);
    }

    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        self.current_function_calls.push(node.method.to_string());
        visit::visit_expr_method_call(self, node);
    }

    fn visit_item_impl(&mut self, node: &'ast ItemImpl) {
        // Visit methods in impl blocks
        visit::visit_item_impl(self, node);
    }
}
