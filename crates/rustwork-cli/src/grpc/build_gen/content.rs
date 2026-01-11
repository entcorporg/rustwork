use std::path::Path;

/// Génère le contenu du fichier build.rs pour un service isolé
/// UTILISE STRICTEMENT OUT_DIR comme requis par tonic
pub(crate) fn generate_service_build_rs_content(proto_dir: &Path) -> String {
    format!(
        r#"// Généré automatiquement par Rustwork
// Ne pas modifier manuellement

fn main() -> Result<(), Box<dyn std::error::Error>> {{
    let proto_dir = std::path::PathBuf::from("{}");
    
    if !proto_dir.exists() {{
        eprintln!("Aucun fichier .proto trouvé dans {{}}", proto_dir.display());
        return Ok(());
    }}

    let mut proto_files = Vec::new();
    
    // Collecter tous les fichiers .proto
    for entry in std::fs::read_dir(&proto_dir)? {{
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("proto") {{
            proto_files.push(path);
        }}
    }}

    if proto_files.is_empty() {{
        eprintln!("Aucun fichier .proto trouvé");
        return Ok(());
    }}

    // CRITIQUE: utiliser EXCLUSIVEMENT OUT_DIR
    let out_dir = std::env::var("OUT_DIR")?;
    
    // Compiler avec tonic - génération serveur ET client
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir(&out_dir)  // STRICTEMENT OUT_DIR
        .compile(&proto_files, &[&proto_dir])?;

    println!("cargo:rerun-if-changed={{}}", proto_dir.display());
    
    Ok(())
}}
"#,
        proto_dir.display()
    )
}

/// Génère le contenu du fichier build.rs (ancienne version - deprecated)
#[allow(dead_code)]
pub(crate) fn generate_build_rs_content() -> String {
    r#"// Généré automatiquement par Rustwork
// Ne pas modifier manuellement

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_dir = std::path::PathBuf::from("target/rustwork/grpc");
    
    if !proto_dir.exists() {
        eprintln!("Aucun fichier .proto trouvé dans {}", proto_dir.display());
        return Ok(());
    }

    let mut proto_files = Vec::new();
    
    // Collecter tous les fichiers .proto
    for entry in std::fs::read_dir(&proto_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("proto") {
            proto_files.push(path);
        }
    }

    if proto_files.is_empty() {
        eprintln!("Aucun fichier .proto trouvé");
        return Ok(());
    }

    // CRITIQUE: utiliser EXCLUSIVEMENT OUT_DIR
    let out_dir = std::env::var("OUT_DIR")?;

    // Compiler avec tonic
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir(&out_dir)
        .compile(&proto_files, &[&proto_dir])?;

    println!("cargo:rerun-if-changed=target/rustwork/grpc");
    
    Ok(())
}
"#
    .to_string()
}
