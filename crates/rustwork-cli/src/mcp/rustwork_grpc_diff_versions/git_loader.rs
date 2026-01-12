/// Logique de comparaison Git pour récupérer une version antérieure d'un fichier .rwk
use std::path::Path;
use std::process::Command;

/// Représente une référence Git valide
#[derive(Debug, Clone)]
pub enum GitRef {
    Main,
    Commit(String),
    Tag(String),
}

impl GitRef {
    /// Parse le paramètre compare_with
    pub fn parse(input: &str) -> Result<Self, String> {
        match input {
            "main" => Ok(GitRef::Main),
            s if s.starts_with("commit:") => {
                let sha = s.strip_prefix("commit:").unwrap().trim();
                if sha.is_empty() {
                    return Err("Commit SHA cannot be empty".to_string());
                }
                Ok(GitRef::Commit(sha.to_string()))
            }
            s if s.starts_with("tag:") => {
                let tag = s.strip_prefix("tag:").unwrap().trim();
                if tag.is_empty() {
                    return Err("Tag name cannot be empty".to_string());
                }
                Ok(GitRef::Tag(tag.to_string()))
            }
            _ => Err(format!(
                "Invalid compare_with value: '{}'. Expected 'main', 'commit:<sha>', or 'tag:<name>'",
                input
            )),
        }
    }

    /// Convertit en référence Git utilisable avec `git show`
    fn to_git_ref(&self) -> String {
        match self {
            GitRef::Main => "main".to_string(),
            GitRef::Commit(sha) => sha.clone(),
            GitRef::Tag(tag) => tag.clone(),
        }
    }
}

/// Récupère le contenu d'un fichier à une référence Git donnée
pub fn get_file_at_ref(
    workspace_root: &Path,
    file_path: &Path,
    git_ref: &GitRef,
) -> Result<String, String> {
    // Construire le chemin relatif depuis la racine du workspace
    let relative_path = file_path
        .strip_prefix(workspace_root)
        .map_err(|_| format!("File {} is not within workspace", file_path.display()))?;

    // Vérifier que la référence existe
    verify_ref_exists(workspace_root, git_ref)?;

    // Exécuter git show
    let git_path = format!("{}:{}", git_ref.to_git_ref(), relative_path.display());
    let output = Command::new("git")
        .arg("show")
        .arg(&git_path)
        .current_dir(workspace_root)
        .output()
        .map_err(|e| format!("Failed to execute git: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "Git show failed for '{}': {}",
            git_path,
            stderr.trim()
        ));
    }

    let content = String::from_utf8(output.stdout)
        .map_err(|e| format!("Invalid UTF-8 in git output: {}", e))?;

    Ok(content)
}

/// Vérifie qu'une référence Git existe
fn verify_ref_exists(workspace_root: &Path, git_ref: &GitRef) -> Result<(), String> {
    let ref_str = git_ref.to_git_ref();

    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--verify")
        .arg(&ref_str)
        .current_dir(workspace_root)
        .output()
        .map_err(|e| format!("Failed to verify git ref: {}", e))?;

    if !output.status.success() {
        return Err(format!("Git reference '{}' does not exist", ref_str));
    }

    Ok(())
}

/// Vérifie que le workspace est dans un dépôt Git valide
pub fn verify_git_repository(workspace_root: &Path) -> Result<(), String> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--git-dir")
        .current_dir(workspace_root)
        .output()
        .map_err(|e| format!("Failed to check git repository: {}", e))?;

    if !output.status.success() {
        return Err("Workspace is not a valid git repository".to_string());
    }

    Ok(())
}
