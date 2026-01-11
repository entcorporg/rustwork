use serde::{Deserialize, Serialize};

/// Struct field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldInfo {
    pub name: String,
    pub type_name: String,
    pub is_public: bool,
}
