/// Détecteur de drift gRPC
use super::types::{DriftDetectionResult, DriftIssue, DriftKind, DriftSeverity, ServicePaths};
use crate::mcp::rustwork_grpc_list_definitions::types::GrpcDefinition;
use std::path::{Path, PathBuf};

pub struct DriftDetector {
    workspace_root: PathBuf,
    definitions: Vec<GrpcDefinition>,
}

impl DriftDetector {
    pub fn new(workspace_root: PathBuf, definitions: Vec<GrpcDefinition>) -> Self {
        Self {
            workspace_root,
            definitions,
        }
    }

    /// Détecte les drifts dans l'ensemble du workspace
    pub fn detect(&self) -> DriftDetectionResult {
        let mut result = DriftDetectionResult::no_drift();

        for definition in &self.definitions {
            self.check_service_drift(definition, &mut result);
        }

        result
    }

    /// Vérifie le drift pour un service
    fn check_service_drift(&self, definition: &GrpcDefinition, result: &mut DriftDetectionResult) {
        // Déterminer les chemins attendus
        let paths = self.determine_paths(definition);

        // 1. Vérifier présence du .proto
        self.check_proto_file(definition, &paths, result);

        // 2. Vérifier présence du build.rs
        self.check_build_rs(definition, &paths, result);

        // 3. Vérifier présence du code généré
        self.check_generated_code(definition, &paths, result);

        // 4. Vérifier timestamps (proto vs rwk, code vs proto)
        self.check_timestamps(definition, &paths, result);
    }

    /// Détermine les chemins attendus pour un service
    fn determine_paths(&self, definition: &GrpcDefinition) -> ServicePaths {
        // Extraire le chemin relatif du .rwk
        let rwk_path = Path::new(&definition.source_file);

        // Micro-services : Backend/services/X/grpc/service.rwk ou services/X/grpc/service.rwk
        let service_dir = rwk_path
            .parent()
            .and_then(|p| p.parent())
            .unwrap_or_else(|| Path::new(""));

        let service_name = rwk_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");

        ServicePaths {
            rwk_file: definition.source_file.clone(),
            proto_file: format!("{}/proto/{}.proto", service_dir.display(), service_name),
            build_rs: format!("{}/build.rs", service_dir.display()),
            generated_code: format!("{}/src/grpc/{}.rs", service_dir.display(), service_name),
        }
    }

    /// Vérifie la présence du fichier .proto
    fn check_proto_file(
        &self,
        definition: &GrpcDefinition,
        paths: &ServicePaths,
        result: &mut DriftDetectionResult,
    ) {
        let proto_path = self.workspace_root.join(&paths.proto_file);

        if !proto_path.exists() {
            result.add_drift(DriftIssue {
                kind: DriftKind::MissingProto,
                severity: DriftSeverity::Error,
                message: format!(
                    "Service '{}': .proto file not generated",
                    definition.service_name
                ),
                rwk_file: Some(paths.rwk_file.clone()),
                proto_file: Some(paths.proto_file.clone()),
                generated_file: None,
                impacted_service: definition.service_name.clone(),
            });
        }
    }

    /// Vérifie la présence du build.rs
    fn check_build_rs(
        &self,
        definition: &GrpcDefinition,
        paths: &ServicePaths,
        result: &mut DriftDetectionResult,
    ) {
        let build_rs_path = self.workspace_root.join(&paths.build_rs);

        if !build_rs_path.exists() {
            result.add_drift(DriftIssue {
                kind: DriftKind::MissingBuildRs,
                severity: DriftSeverity::Warning,
                message: format!("Service '{}': build.rs not found", definition.service_name),
                rwk_file: Some(paths.rwk_file.clone()),
                proto_file: None,
                generated_file: None,
                impacted_service: definition.service_name.clone(),
            });
        }
    }

    /// Vérifie la présence du code généré
    fn check_generated_code(
        &self,
        definition: &GrpcDefinition,
        paths: &ServicePaths,
        result: &mut DriftDetectionResult,
    ) {
        let generated_path = self.workspace_root.join(&paths.generated_code);

        if !generated_path.exists() {
            result.add_drift(DriftIssue {
                kind: DriftKind::MissingGeneratedCode,
                severity: DriftSeverity::Error,
                message: format!(
                    "Service '{}': generated Rust code not found",
                    definition.service_name
                ),
                rwk_file: Some(paths.rwk_file.clone()),
                proto_file: Some(paths.proto_file.clone()),
                generated_file: Some(paths.generated_code.clone()),
                impacted_service: definition.service_name.clone(),
            });
        }
    }

    /// Vérifie les timestamps pour détecter fichiers obsolètes
    fn check_timestamps(
        &self,
        definition: &GrpcDefinition,
        paths: &ServicePaths,
        result: &mut DriftDetectionResult,
    ) {
        let rwk_path = self.workspace_root.join(&paths.rwk_file);
        let proto_path = self.workspace_root.join(&paths.proto_file);
        let generated_path = self.workspace_root.join(&paths.generated_code);

        // Obtenir les timestamps
        let rwk_time = std::fs::metadata(&rwk_path).and_then(|m| m.modified()).ok();

        let proto_time = std::fs::metadata(&proto_path)
            .and_then(|m| m.modified())
            .ok();

        let generated_time = std::fs::metadata(&generated_path)
            .and_then(|m| m.modified())
            .ok();

        // Vérifier .proto vs .rwk
        if let (Some(rwk_t), Some(proto_t)) = (rwk_time, proto_time) {
            if proto_t < rwk_t {
                result.add_drift(DriftIssue {
                    kind: DriftKind::OutdatedProto,
                    severity: DriftSeverity::Warning,
                    message: format!(
                        "Service '{}': .proto file is older than .rwk (needs regeneration)",
                        definition.service_name
                    ),
                    rwk_file: Some(paths.rwk_file.clone()),
                    proto_file: Some(paths.proto_file.clone()),
                    generated_file: None,
                    impacted_service: definition.service_name.clone(),
                });
            }
        }

        // Vérifier code généré vs .proto
        if let (Some(proto_t), Some(gen_t)) = (proto_time, generated_time) {
            if gen_t < proto_t {
                result.add_drift(DriftIssue {
                    kind: DriftKind::OutdatedGeneratedCode,
                    severity: DriftSeverity::Error,
                    message: format!(
                        "Service '{}': generated code is older than .proto (needs rebuild)",
                        definition.service_name
                    ),
                    rwk_file: Some(paths.rwk_file.clone()),
                    proto_file: Some(paths.proto_file.clone()),
                    generated_file: Some(paths.generated_code.clone()),
                    impacted_service: definition.service_name.clone(),
                });
            }
        }
    }
}
