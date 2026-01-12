/// Analyseur d'état des services gRPC
use super::types::*;
use crate::mcp::common::path_normalization::NormalizedPath;
use crate::mcp::common::workspace_root::WorkspaceRoot;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Analyseur d'état de service gRPC
pub struct ServiceStatusAnalyzer {
    workspace_root: Arc<WorkspaceRoot>,
}

impl ServiceStatusAnalyzer {
    pub fn new(workspace_root: Arc<WorkspaceRoot>) -> Self {
        Self { workspace_root }
    }

    /// Analyse l'état d'un service gRPC
    pub fn analyze(&self, service_name: &str) -> ServiceStatus {
        let mut inconsistencies = Vec::new();

        // Chercher le fichier .rwk
        let rwk_status = self.find_and_check_rwk(service_name, &mut inconsistencies);

        // Vérifier le code généré
        let generated_status = self.check_generated_code(service_name);

        // Déterminer l'état général
        let state = if rwk_status.is_none() {
            ServiceState::Unknown
        } else if !inconsistencies.is_empty() {
            ServiceState::Degraded
        } else {
            ServiceState::Known
        };

        ServiceStatus {
            service_name: service_name.to_string(),
            status: state,
            rwk_file: rwk_status,
            generated_code: generated_status,
            inconsistencies,
        }
    }

    /// Trouve et vérifie le fichier .rwk
    fn find_and_check_rwk(
        &self,
        service_name: &str,
        inconsistencies: &mut Vec<Inconsistency>,
    ) -> Option<RwkFileStatus> {
        // Chercher dans les emplacements possibles
        let possible_paths = self.get_possible_rwk_paths(service_name);

        for rwk_path in &possible_paths {
            if rwk_path.exists() {
                return Some(self.check_rwk_file(rwk_path, inconsistencies));
            }
        }

        // .rwk introuvable
        inconsistencies.push(Inconsistency {
            kind: InconsistencyKind::MissingRwkFile,
            message: format!("No .rwk file found for service '{}'", service_name),
        });

        None
    }

    /// Vérifie un fichier .rwk
    fn check_rwk_file(
        &self,
        rwk_path: &Path,
        inconsistencies: &mut Vec<Inconsistency>,
    ) -> RwkFileStatus {
        let normalized_path = match NormalizedPath::from_path(rwk_path, self.workspace_root.path())
        {
            Ok(path) => path.as_str().to_string(),
            Err(_) => rwk_path.display().to_string(), // Fallback
        };

        // Essayer de lire le fichier
        let source = match std::fs::read_to_string(rwk_path) {
            Ok(s) => s,
            Err(e) => {
                inconsistencies.push(Inconsistency {
                    kind: InconsistencyKind::InvalidRwkFile,
                    message: format!("Failed to read .rwk file: {}", e),
                });

                return RwkFileStatus {
                    path: normalized_path,
                    exists: true,
                    parsable: false,
                    parse_error: Some(format!("Read error: {}", e)),
                };
            }
        };

        // Essayer de parser
        match crate::grpc::parse_contract(&source) {
            Ok(_) => RwkFileStatus {
                path: normalized_path,
                exists: true,
                parsable: true,
                parse_error: None,
            },
            Err(e) => {
                inconsistencies.push(Inconsistency {
                    kind: InconsistencyKind::InvalidRwkFile,
                    message: format!("Failed to parse .rwk file: {}", e),
                });

                RwkFileStatus {
                    path: normalized_path,
                    exists: true,
                    parsable: false,
                    parse_error: Some(e.to_string()),
                }
            }
        }
    }

    /// Vérifie l'existence du code généré
    fn check_generated_code(&self, service_name: &str) -> Option<GeneratedCodeStatus> {
        let root = self.workspace_root.path();

        // Chemins possibles pour le code généré
        let proto_paths = [
            root.join(format!("grpc/generated/{}.proto", service_name)),
            root.join(format!(
                "services/{}/grpc/generated/{}.proto",
                service_name, service_name
            )),
        ];

        let client_paths = [
            root.join(format!("grpc/generated/{}_client.rs", service_name)),
            root.join(format!(
                "services/{}/grpc/generated/{}_client.rs",
                service_name, service_name
            )),
        ];

        let server_paths = [
            root.join(format!("grpc/generated/{}_server.rs", service_name)),
            root.join(format!(
                "services/{}/grpc/generated/{}_server.rs",
                service_name, service_name
            )),
        ];

        // Trouver les fichiers existants
        let proto_file = proto_paths.iter().find(|p| p.exists());
        let client_file = client_paths.iter().find(|p| p.exists());
        let server_file = server_paths.iter().find(|p| p.exists());

        Some(GeneratedCodeStatus {
            proto_file: proto_file.map(|p| match NormalizedPath::from_path(p, root) {
                Ok(path) => path.as_str().to_string(),
                Err(_) => p.display().to_string(),
            }),
            rust_client: client_file.map(|p| match NormalizedPath::from_path(p, root) {
                Ok(path) => path.as_str().to_string(),
                Err(_) => p.display().to_string(),
            }),
            rust_server: server_file.map(|p| match NormalizedPath::from_path(p, root) {
                Ok(path) => path.as_str().to_string(),
                Err(_) => p.display().to_string(),
            }),
            proto_exists: proto_file.is_some(),
            client_exists: client_file.is_some(),
            server_exists: server_file.is_some(),
        })
    }

    /// Chemins possibles pour les fichiers .rwk
    fn get_possible_rwk_paths(&self, service_name: &str) -> Vec<PathBuf> {
        let root = self.workspace_root.path();
        vec![
            // Nouvelle structure Backend/services/
            root.join(format!(
                "Backend/services/{}/grpc/{}.rwk",
                service_name, service_name
            )),
            // Structure legacy services/
            root.join(format!(
                "services/{}/grpc/{}.rwk",
                service_name, service_name
            )),
        ]
    }
}
