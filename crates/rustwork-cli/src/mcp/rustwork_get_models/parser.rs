use super::model_types::*;
use crate::mcp::common::protocol::RpcError;
use std::path::Path;
use syn::{visit::Visit, Attribute, Item, ItemStruct, Type};

/// Parse all models from a service
pub async fn parse_service_models(
    service_path: &Path,
    service_name: &str,
) -> Result<Vec<RustModel>, RpcError> {
    let mut all_models = Vec::new();

    // Primary location: src/models/
    let models_dir = service_path.join("src/models");
    if models_dir.exists() && models_dir.is_dir() {
        let models = parse_models_directory(&models_dir, service_name).await?;
        all_models.extend(models);
    }

    // Secondary location: src/entities/ (legacy convention)
    let entities_dir = service_path.join("src/entities");
    if entities_dir.exists() && entities_dir.is_dir() {
        let entities = parse_models_directory(&entities_dir, service_name).await?;
        all_models.extend(entities);
    }

    // Fallback: scan src/ for DTOs
    let src_dir = service_path.join("src");
    if src_dir.exists() && all_models.is_empty() {
        let dto_models = scan_for_dto_structs(&src_dir, service_name).await?;
        all_models.extend(dto_models);
    }

    Ok(all_models)
}

/// Parse all Rust files in a directory
async fn parse_models_directory(
    dir: &Path,
    service_name: &str,
) -> Result<Vec<RustModel>, RpcError> {
    let mut models = Vec::new();

    let entries = std::fs::read_dir(dir).map_err(|e| {
        RpcError::internal_error(format!("Failed to read directory {}: {}", dir.display(), e))
    })?;

    for entry in entries.flatten() {
        let path = entry.path();

        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("rs") {
            let file_models = parse_rust_file(&path, service_name).await?;
            models.extend(file_models);
        } else if path.is_dir() {
            // Recursively parse subdirectories using Box::pin
            let sub_models = Box::pin(parse_models_directory(&path, service_name)).await?;
            models.extend(sub_models);
        }
    }

    Ok(models)
}

/// Scan src/ for DTO-like structs (fallback)
async fn scan_for_dto_structs(
    src_dir: &Path,
    service_name: &str,
) -> Result<Vec<RustModel>, RpcError> {
    let mut models = Vec::new();

    let entries = std::fs::read_dir(src_dir)
        .map_err(|e| RpcError::internal_error(format!("Failed to read src directory: {}", e)))?;

    for entry in entries.flatten() {
        let path = entry.path();

        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("rs") {
            let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

            // Only scan files likely to contain DTOs
            if file_name.contains("dto")
                || file_name.contains("request")
                || file_name.contains("response")
            {
                let file_models = parse_rust_file(&path, service_name).await?;
                models.extend(file_models);
            }
        }
    }

    Ok(models)
}

/// Parse a single Rust file for model structs
async fn parse_rust_file(file_path: &Path, service_name: &str) -> Result<Vec<RustModel>, RpcError> {
    let content = tokio::fs::read_to_string(file_path).await.map_err(|e| {
        RpcError::internal_error(format!(
            "Failed to read file {}: {}",
            file_path.display(),
            e
        ))
    })?;

    let syntax_tree = syn::parse_file(&content).map_err(|e| {
        RpcError::internal_error(format!(
            "Failed to parse Rust file {}: {}",
            file_path.display(),
            e
        ))
    })?;

    let mut visitor = ModelVisitor::new(service_name.to_string(), file_path.display().to_string());
    visitor.visit_file(&syntax_tree);

    Ok(visitor.models)
}

/// Visitor to extract model structs from AST
struct ModelVisitor {
    service: String,
    file_path: String,
    models: Vec<RustModel>,
}

impl ModelVisitor {
    fn new(service: String, file_path: String) -> Self {
        Self {
            service,
            file_path,
            models: Vec::new(),
        }
    }

    fn extract_model_from_struct(&self, item_struct: &ItemStruct) -> Option<RustModel> {
        let name = item_struct.ident.to_string();

        // Skip internal/private structs that are unlikely to be models
        if name.starts_with('_') || name.contains("Internal") {
            return None;
        }

        // Determine model type from derives and attributes
        let derives = extract_derives(&item_struct.attrs);
        let model_type = determine_model_type(&name, &derives);

        // Extract fields
        let fields = extract_fields(&item_struct.fields);

        // Extract SeaORM relations if applicable
        let relations = if model_type == ModelType::Entity {
            extract_relations(&item_struct.attrs)
        } else {
            Vec::new()
        };

        let is_public = matches!(item_struct.vis, syn::Visibility::Public(_));

        // Get line number
        let line = item_struct.ident.span().start().line;

        Some(RustModel {
            name,
            service: self.service.clone(),
            file_path: self.file_path.clone(),
            line,
            model_type,
            fields,
            derives,
            relations,
            is_public,
        })
    }
}

impl<'ast> Visit<'ast> for ModelVisitor {
    fn visit_item(&mut self, item: &'ast Item) {
        if let Item::Struct(item_struct) = item {
            if let Some(model) = self.extract_model_from_struct(item_struct) {
                self.models.push(model);
            }
        }

        syn::visit::visit_item(self, item);
    }
}

/// Extract derive macros from attributes
fn extract_derives(attrs: &[Attribute]) -> Vec<String> {
    let mut derives = Vec::new();

    for attr in attrs {
        if attr.path().is_ident("derive") {
            if let Ok(meta_list) = attr.meta.require_list() {
                let tokens = meta_list.tokens.to_string();
                // Split by comma and clean up
                for derive in tokens.split(',') {
                    let cleaned = derive.trim().to_string();
                    if !cleaned.is_empty() {
                        derives.push(cleaned);
                    }
                }
            }
        }
    }

    derives
}

/// Determine model type from name and derives
fn determine_model_type(name: &str, derives: &[String]) -> ModelType {
    // Check derives for SeaORM
    if derives
        .iter()
        .any(|d| d.contains("DeriveEntityModel") || d.contains("FromQueryResult"))
    {
        return ModelType::Entity;
    }

    // Check name patterns
    let lower_name = name.to_lowercase();

    if lower_name.ends_with("request") || lower_name.contains("req") {
        ModelType::Request
    } else if lower_name.ends_with("response") || lower_name.contains("resp") {
        ModelType::Response
    } else if lower_name.ends_with("dto") {
        ModelType::Dto
    } else if lower_name.ends_with("model") || lower_name.ends_with("entity") {
        ModelType::Domain
    } else {
        ModelType::Unknown
    }
}

/// Extract fields from struct
fn extract_fields(fields: &syn::Fields) -> Vec<ModelField> {
    let mut result = Vec::new();

    if let syn::Fields::Named(fields_named) = fields {
        for field in &fields_named.named {
            if let Some(ident) = &field.ident {
                let name = ident.to_string();
                let (rust_type, nullable) = extract_type_info(&field.ty);
                let attributes = extract_field_attributes(&field.attrs);

                result.push(ModelField {
                    name,
                    rust_type,
                    nullable,
                    attributes,
                });
            }
        }
    }

    result
}

/// Extract type information (and check if Option<T>)
fn extract_type_info(ty: &Type) -> (String, bool) {
    let type_string = quote::quote!(#ty).to_string();

    // Check if it's Option<T>
    if type_string.starts_with("Option") {
        // Extract inner type
        let inner = type_string
            .trim_start_matches("Option")
            .trim_start_matches('<')
            .trim_end_matches('>')
            .trim();
        (inner.to_string(), true)
    } else {
        (type_string, false)
    }
}

/// Extract field-level attributes (serde, validation, etc.)
fn extract_field_attributes(attrs: &[Attribute]) -> Vec<String> {
    let mut result = Vec::new();

    for attr in attrs {
        let path = attr.path();
        if path.is_ident("serde") || path.is_ident("validate") || path.is_ident("sea_orm") {
            result.push(quote::quote!(#attr).to_string());
        }
    }

    result
}

/// Extract SeaORM relations (simplified - full implementation would parse relation macros)
fn extract_relations(_attrs: &[Attribute]) -> Vec<ModelRelation> {
    // TODO: Parse SeaORM relation attributes
    // This requires deeper macro parsing which is complex
    // For P0, return empty - can be enhanced in P1
    Vec::new()
}
