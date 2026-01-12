use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Project information for multi-service setups
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectInfo {
    pub name: String,
    pub path: PathBuf,
    pub is_service: bool, // true if part of microservices architecture
}
