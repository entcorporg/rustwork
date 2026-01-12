use super::utils::{detect_rustwork_services, to_snake_case};
use crate::grpc;
use crate::mcp::common::workspace_root::WorkspaceRoot;
/// Commande `rustwork grpc build`
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// Execute la commande grpc build
pub async fn execute(project_path: Option<String>) -> Result<()> {
    let current_dir = std::env::current_dir()?;

    // Step 1: Detect workspace root
    let workspace_root = if let Some(ref path) = project_path {
        let explicit = PathBuf::from(path);
        WorkspaceRoot::detect_with_explicit(&current_dir, Some(&explicit))?
    } else {
        WorkspaceRoot::detect(&current_dir)?
    };

    let project_root = workspace_root.path().to_path_buf();

    // Step 2: Detect all Rustwork services
    let services = detect_rustwork_services(&project_root)?;

    // GARDE-FOU CRITIQUE : gRPC UNIQUEMENT en mode micro-services (≥ 2 services)
    if services.len() < 2 {
        anyhow::bail!(
            "❌ gRPC is only supported in micro-service layout.\n\
            Detected {} service(s). At least 2 services are required.\n\n\
            To use gRPC, create a micro-services project with:\n\
            rustwork new <name> --layout micro --services <service1>,<service2>",
            services.len()
        );
    }

    println!("🔨 Construction des services gRPC...");
    println!();

    // Scanner UNIQUEMENT services/*/grpc/*.rwk
    let services_dir = project_root.join("services");
    if !services_dir.exists() {
        anyhow::bail!("Dossier services/ introuvable");
    }

    let service_grpc_configs = find_service_grpc_configs(&services_dir)?;

    if service_grpc_configs.is_empty() {
        println!("⚠️  Aucun fichier .rwk trouvé dans services/*/grpc/");
        println!("   Créez un dossier grpc/ dans vos services avec des fichiers .rwk");
        return Ok(());
    }

    println!("📦 Services gRPC détectés: {}", service_grpc_configs.len());
    for config in &service_grpc_configs {
        println!(
            "   - {} ({} fichiers .rwk)",
            config.service_name,
            config.rwk_files.len()
        );
    }
    println!();

    // Traiter chaque service de manière isolée
    for config in &service_grpc_configs {
        process_service_grpc(&project_root, config)?;
    }

    println!();
    println!("✅ Construction gRPC terminée avec succès!");
    println!();
    println!("📌 Prochaines étapes:");
    println!("   1. Exécutez 'cargo build --workspace' pour compiler");
    println!("   2. Implémentez les traits *Handler dans vos services");
    println!("   3. Utilisez grpc_service() pour créer votre serveur");
    println!();

    Ok(())
}

#[derive(Debug)]
struct ServiceGrpcConfig {
    service_name: String,
    service_path: PathBuf,
    rwk_files: Vec<PathBuf>,
}

/// Trouve tous les services avec des fichiers .rwk
fn find_service_grpc_configs(services_dir: &Path) -> Result<Vec<ServiceGrpcConfig>> {
    let mut configs = Vec::new();

    for entry in fs::read_dir(services_dir)? {
        let entry = entry?;
        let service_path = entry.path();

        if !service_path.is_dir() {
            continue;
        }

        let service_name = service_path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| anyhow::anyhow!("Nom de service invalide"))?
            .to_string();

        let grpc_dir = service_path.join("grpc");
        if !grpc_dir.exists() || !grpc_dir.is_dir() {
            continue;
        }

        let mut rwk_files = Vec::new();
        for rwk_entry in fs::read_dir(&grpc_dir)? {
            let rwk_entry = rwk_entry?;
            let path = rwk_entry.path();

            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("rwk") {
                rwk_files.push(path);
            }
        }

        if !rwk_files.is_empty() {
            configs.push(ServiceGrpcConfig {
                service_name,
                service_path,
                rwk_files,
            });
        }
    }

    Ok(configs)
}

/// Traite un service gRPC de manière isolée
fn process_service_grpc(_project_root: &Path, config: &ServiceGrpcConfig) -> Result<()> {
    println!("🔧 Traitement du service '{}'...", config.service_name);

    // 1. Créer le dossier target pour les .proto de CE service
    let proto_dir = config.service_path.join("target/rustwork/grpc");
    fs::create_dir_all(&proto_dir).context("Impossible de créer target/rustwork/grpc")?;

    let mut contracts = Vec::new();
    let mut service_names = Vec::new();

    // 2. Parser et générer les .proto pour CE service uniquement
    for rwk_file in &config.rwk_files {
        let source = fs::read_to_string(rwk_file)
            .with_context(|| format!("Erreur lecture {}", rwk_file.display()))?;

        let contract = grpc::parse_contract(&source).map_err(|e| {
            eprintln!("\n❌ Erreur dans {}:", rwk_file.display());
            eprintln!("{}", e.format_with_context());
            anyhow::anyhow!("Erreur de parsing")
        })?;

        service_names.push(contract.service.name.clone());

        let proto_content = grpc::generate_proto(&contract).context("Erreur génération proto")?;

        let proto_filename = format!("{}_service.proto", to_snake_case(&contract.service.name));
        let proto_path = proto_dir.join(&proto_filename);

        fs::write(&proto_path, proto_content)
            .with_context(|| format!("Erreur écriture {}", proto_path.display()))?;

        println!("  ✓ Généré: {}", proto_filename);
        contracts.push(contract);
    }

    // 3. Vérifier/ajouter les dépendances gRPC dans le Cargo.toml du service
    grpc::add_grpc_dependencies(&config.service_path)
        .map_err(|e| anyhow::anyhow!("Erreur dépendances: {}", e))?;

    // 4. Générer build.rs DANS le dossier du service
    grpc::generate_service_build_rs(&config.service_path, &proto_dir)
        .context("Erreur génération build.rs")?;

    // 5. Créer le dossier pour le code généré du service
    let grpc_src_dir = config.service_path.join("src/grpc");
    fs::create_dir_all(&grpc_src_dir).context("Impossible de créer src/grpc")?;

    // 6. Générer mod.rs
    let mod_content =
        grpc::generate_grpc_mod(&service_names).context("Erreur génération mod.rs")?;

    fs::write(grpc_src_dir.join("mod.rs"), mod_content)
        .context("Erreur écriture src/grpc/mod.rs")?;

    // 7. Générer le code Rust pour chaque contrat
    for contract in &contracts {
        let rust_content = grpc::rust_gen::generate_rust_service(contract)
            .context("Erreur génération code Rust")?;

        let rust_filename = format!("{}.rs", to_snake_case(&contract.service.name));
        let rust_path = grpc_src_dir.join(&rust_filename);

        fs::write(&rust_path, rust_content)
            .with_context(|| format!("Erreur écriture {}", rust_path.display()))?;

        println!("  ✓ Généré: src/grpc/{}", rust_filename);
    }

    println!("  ✅ Service '{}' traité", config.service_name);
    println!();

    Ok(())
}
