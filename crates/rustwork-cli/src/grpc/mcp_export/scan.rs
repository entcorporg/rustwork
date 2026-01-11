use std::path::Path;

/// Trouve tous les fichiers .rwk dans le projet
#[allow(dead_code)]
pub(crate) fn find_rwk_files(project_root: &Path) -> Result<Vec<std::path::PathBuf>, String> {
    let mut rwk_files = Vec::new();

    // Chercher dans grpc/
    let grpc_dir = project_root.join("grpc");
    if grpc_dir.exists() && grpc_dir.is_dir() {
        for entry in
            std::fs::read_dir(&grpc_dir).map_err(|e| format!("Erreur lecture grpc/: {}", e))?
        {
            let entry = entry.map_err(|e| format!("Erreur entrée: {}", e))?;
            let path = entry.path();

            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("rwk") {
                rwk_files.push(path);
            }
        }
    }

    // Chercher dans services/*/grpc/
    let services_dir = project_root.join("services");
    if services_dir.exists() && services_dir.is_dir() {
        for service_entry in std::fs::read_dir(&services_dir)
            .map_err(|e| format!("Erreur lecture services/: {}", e))?
        {
            let service_entry = service_entry.map_err(|e| format!("Erreur entrée: {}", e))?;
            let service_grpc_dir = service_entry.path().join("grpc");

            if service_grpc_dir.exists() && service_grpc_dir.is_dir() {
                for entry in std::fs::read_dir(&service_grpc_dir)
                    .map_err(|e| format!("Erreur lecture {}: {}", service_grpc_dir.display(), e))?
                {
                    let entry = entry.map_err(|e| format!("Erreur entrée: {}", e))?;
                    let path = entry.path();

                    if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("rwk") {
                        rwk_files.push(path);
                    }
                }
            }
        }
    }

    Ok(rwk_files)
}
