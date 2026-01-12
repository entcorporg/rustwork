/// Model representation from Rust source code
#[derive(Debug, Clone)]
pub struct RustModel {
    pub name: String,
    pub service: String,
    pub file_path: String,
    pub line: usize,
    pub model_type: ModelType,
    pub fields: Vec<ModelField>,
    pub derives: Vec<String>,
    pub relations: Vec<ModelRelation>,
    pub is_public: bool,
}

/// Type of model
#[derive(Debug, Clone, PartialEq)]
pub enum ModelType {
    Entity,   // SeaORM entity
    Dto,      // Data Transfer Object
    Request,  // HTTP request payload
    Response, // HTTP response payload
    Domain,   // Domain model
    Unknown,  // Could not determine
}

/// Model field
#[derive(Debug, Clone)]
pub struct ModelField {
    pub name: String,
    pub rust_type: String,
    pub nullable: bool,          // Is Option<T>
    pub attributes: Vec<String>, // Serde, validation, etc.
}

/// Model relation (SeaORM)
#[derive(Debug, Clone)]
pub struct ModelRelation {
    pub relation_type: RelationType,
    pub target_entity: String,
    pub field_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum RelationType {
    HasOne,
    HasMany,
    BelongsTo,
    ManyToMany,
}
