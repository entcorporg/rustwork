mod ast;
mod scan;
mod types;

pub use scan::scan_project;
pub use types::{CodeIndex, SourceFile};

#[cfg(test)]
mod tests {
    use super::types::CodeIndex;
    use crate::mcp::indexer::ast::CodeVisitor;
    use syn::{visit::Visit, File};

    #[test]
    fn test_parse_simple_function() {
        let code = r#"
            pub async fn get_user(id: i32) -> Result<User, Error> {
                let user = fetch_user(id).await?;
                Ok(user)
            }
        "#;

        let syntax_tree: File = syn::parse_str(code).unwrap();
        let mut visitor = CodeVisitor::new();
        visitor.visit_file(&syntax_tree);

        assert_eq!(visitor.functions.len(), 1);
        let func = &visitor.functions[0];
        assert_eq!(func.name, "get_user");
        assert!(func.is_public);
        assert!(func.is_async);
        assert!(func.calls.contains(&"fetch_user".to_string()));
        // Spans are now captured
        assert!(func.start_line > 0);
        assert!(func.end_line >= func.start_line);
    }

    #[test]
    fn test_call_graph_depth() {
        let mut index = CodeIndex::new();

        // A -> B -> C
        index
            .call_graph
            .insert("A".to_string(), ["B".to_string()].into());
        index
            .call_graph
            .insert("B".to_string(), ["C".to_string()].into());

        let calls = index.get_calls("A", 2);
        assert!(calls.contains_key("B"));
        assert!(calls.contains_key("C"));
        assert_eq!(calls["B"], 1);
        assert_eq!(calls["C"], 2);
    }
}
