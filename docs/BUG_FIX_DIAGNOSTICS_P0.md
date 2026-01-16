# Bug Fix P0 : rustwork_get_diagnostics ne capturait pas les diagnostics du projet utilisateur

## 📋 Résumé

**Version corrigée** : 0.1.4  
**Priorité** : P0 (critique)  
**Catégorie** : MCP Server  

## 🐛 Problème identifié

Le tool MCP `rustwork_get_diagnostics` retournait systématiquement un état "ready" avec 0 erreurs, alors que le projet utilisateur contenait des erreurs de compilation réelles visibles dans VS Code via rust-analyzer.

### Symptômes

```json
// rustwork_get_diagnostics retournait :
{
  "diagnostics": [],
  "errors": 0,
  "warnings": 0,
  "total": 0,
  "last_build_success": true,
  "index_state": "ready",
  "index_files_count": 30,
  "index_is_ready": true
}
```

Alors que VS Code/rust-analyzer rapportait :

```json
{
  "resource": "/home/user/project/services/login/src/main.rs",
  "severity": 8,
  "message": "file not found for module `errors`",
  "source": "rustc",
  "startLineNumber": 6
}
```

## 🔍 Analyse de la cause racine

### Cause technique #1 : Pas de spécification du répertoire

Le `DiagnosticCollector` lançait `cargo check` **sans spécifier le répertoire de travail** :

```rust
// AVANT (bugué)
Command::new("cargo")
    .args(["check", "--message-format=json", "--all-targets"])
    .stdout(Stdio::piped())
    .spawn()
```

**Résultat** : `cargo check` s'exécutait dans le répertoire courant du **processus MCP**, c'est-à-dire le workspace Rustwork lui-même (`/home/linux/rustwork`) au lieu du projet utilisateur (`/home/user/project`).

### Cause technique #2 : Mauvais répertoire cible

Le `WorkspaceRoot` détecte la **racine du projet Rustwork** mais pas le **répertoire du workspace Cargo** :

```
test_rustwork/              ← workspace_root.path() pointait ici
├── .rustwork/
└── Backend/               ← cargo check doit s'exécuter ICI
    ├── Cargo.toml
    └── services/
```

Dans un projet microservices, `cargo check` doit s'exécuter dans `Backend/` où se trouve le `Cargo.toml` du workspace, pas à la racine.

### Architecture défaillante

```
┌─────────────────────────────────────┐
│  VS Code Workspace                  │
│  /home/user/project/                │
│                                     │
│  ┌─────────────────────────────┐   │
│  │ MCP Server (rustwork mcp)   │   │
│  │ project_path: .             │   │
│  │ workspace_root: detected ✓  │   │
│  │                             │   │
│  │ DiagnosticCollector         │   │
│  │   cargo check               │   │
│  │   ❌ CWD: /home/linux/      │   │  ← BUG ICI
│  │      rustwork (MAUVAIS)     │   │
│  └─────────────────────────────┘   │
└─────────────────────────────────────┘
```

### Pourquoi le problème n'a pas été détecté ?

1. **Le workspace Rustwork lui-même compile sans erreur** → pas d'alerte
2. **Le MCP fonctionnait pour d'autres tools** (get_routes, get_file_doc) car ils n'exécutent pas de commandes externes
3. **Aucune validation de la localisation de `cargo check`**

## ✅ Solution implémentée

### Changements structurels

#### 1. Modification de `DiagnosticCollector`

**Fichier** : `crates/rustwork-cli/src/mcp/common/diagnostics/collector.rs`

```rust
// Structure étendue
pub struct DiagnosticCollector {
    collection: Arc<RwLock<DiagnosticCollection>>,
    workspace_path: PathBuf,  // ← AJOUT
}

// Constructeur modifié
impl DiagnosticCollector {
    pub fn new(workspace_path: PathBuf) -> Self {  // ← PARAMÈTRE
        Self {
            collection: Arc::new(RwLock::new(DiagnosticCollection::new())),
            workspace_path,
        }
    }

    pub async fn start_collecting(&self) -> Result<()> {
        let workspace_path = self.workspace_path.clone();
        
        // cargo check avec .current_dir()
        Command::new("cargo")
            .args(["check", "--message-format=json", "--all-targets"])
            .current_dir(&workspace_path)  // ← FIX CRITIQUE
            .stdout(Stdio::piped())
            .spawn()
    }
}
```

#### 2. Ajout de `cargo_workspace_dir()` à `WorkspaceRoot`

**Fichier** : `crates/rustwork-cli/src/mcp/common/workspace_root/types.rs`

```rust
impl WorkspaceRoot {
    /// Get the Cargo workspace directory path
    /// 
    /// Returns Backend/ if it exists (microservices with Backend/Cargo.toml),
    /// otherwise returns the root path (legacy structure)
    pub fn cargo_workspace_dir(&self) -> PathBuf {
        let backend_dir = self.path.join("Backend");
        if backend_dir.join("Cargo.toml").exists() {
            backend_dir  // ← Structure moderne : Backend/Cargo.toml
        } else {
            self.path.clone()  // ← Structure legacy : ./Cargo.toml
        }
    }
}
```

#### 3. Utilisation de `cargo_workspace_dir()` dans le collector

**Fichier** : `crates/rustwork-cli/src/mcp/common/state/watchers.rs`

```rust
pub async fn start_diagnostics_collector(&self) -> Result<()> {
    // Utiliser cargo_workspace_dir() au lieu de path()
    let collector = DiagnosticCollector::new(
        self.workspace_root.cargo_workspace_dir()  // ← FIX: Backend/ au lieu de ./
    );
    // ...
}
```

### Architecture corrigée

```
┌─────────────────────────────────────┐
│  VS Code Workspace                  │
│  /home/user/project/                │
│                                     │
│  ┌─────────────────────────────┐   │
│  │ MCP Server (rustwork mcp)   │   │
│  │ workspace_root: detected ✓  │   │
│  │   ↓                         │   │
│  │ cargo_workspace_dir()       │   │  ← NOUVELLE MÉTHODE
│  │   → /home/user/project/     │   │
│  │      Backend/               │   │
│  │                             │   │
│  │ DiagnosticCollector         │   │
│  │   workspace_path: Backend/  │   │
│  │   cargo check               │   │
│  │   ✓ CWD: /home/user/        │   │  ← CORRIGÉ
│  │      project/Backend/       │   │
│  └─────────────────────────────┘   │
└─────────────────────────────────────┘
```

## 🧪 Validation

### Tests réalisés

1. **Build release** : `cargo build --release --bin rustwork` ✅
2. **Tests unitaires** : `cargo test --workspace` → 100 tests passed ✅
3. **Nouveaux tests** : `test_cargo_workspace_dir_backend_structure` et `test_cargo_workspace_dir_legacy_structure` ✅
4. **Installation** : `cargo install --path crates/rustwork-cli --force` ✅

### Vérification manuelle

```bash
# 1. Créer un projet avec erreur
cd /tmp
rustwork new test-service
cd test-service/Backend/services/test-service/src
echo "mod missing_module;" >> main.rs

# 2. Lancer MCP
rustwork mcp --stdio --project /tmp/test-service

# 3. Appeler rustwork_get_diagnostics
# → Doit maintenant retourner l'erreur "cannot find module 'missing_module'"
```

## ⚠️ Après installation : redémarrer le serveur MCP

**IMPORTANT** : Après avoir installé la version 0.1.4, vous devez redémarrer le serveur MCP.

### Option 1 : Recharger VS Code (recommandé)

```
Ctrl+Shift+P → "Developer: Reload Window"
```

### Option 2 : Tuer le processus MCP manuellement

```bash
# Trouver le PID
ps aux | grep "rustwork mcp"

# Tuer le processus
kill <PID>

# VS Code relancera automatiquement le serveur avec la nouvelle version
```

### Vérifier que la nouvelle version est active

```bash
# Vérifier l'installation
rustwork --version  # Doit afficher 0.1.4

# Vérifier le processus en cours
ps aux | grep "rustwork mcp"  # Doit être lancé APRÈS l'installation
```

## 📊 Impact

### Criticité

- **Priorité P0** : Tool MCP fondamental non fonctionnel
- **Scope** : Tous les utilisateurs du MCP server
- **Workaround** : Aucun (impossible de voir les diagnostics)

### Effets du fix

| Avant | Après |
|-------|-------|
| `rustwork_get_diagnostics` ne détecte jamais les erreurs utilisateur | Capture correcte des erreurs du projet |
| IA ne voit pas les problèmes de compilation | IA peut proposer des corrections |
| Debugging impossible via MCP | Debugging complet via MCP |

## 🔄 Implémentations futures possibles

### Limitation actuelle

Le MCP lance toujours `cargo check` dans un processus séparé. Il ne capture **pas** les diagnostics de rust-analyzer déjà présents dans VS Code.

### Évolutions envisageables

1. **Extension VS Code dédiée**  
   Bridge entre rust-analyzer diagnostics et serveur MCP

2. **Fichier de cache partagé**  
   rust-analyzer → JSON → MCP

3. **Intégration Language Server Protocol**  
   MCP expose directement les diagnostics LSP

## 📝 Leçons apprises

### Pour éviter ce type de bug

1. ✅ **Toujours passer les chemins explicites** aux outils externes (cargo, git)
2. ✅ **Valider le répertoire de travail** dans les tests d'intégration
3. ✅ **Logger le CWD** lors du lancement de commandes externes
4. ✅ **Tests end-to-end** avec projets utilisateur simulés

### Pattern à suivre

```rust
// ❌ ÉVITER
Command::new("cargo").args(["check"]).spawn()

// ✅ PRÉFÉRER
Command::new("cargo")
    .args(["check"])
    .current_dir(&explicit_workspace_path)
    .spawn()
```

## 📎 Références

- **PR/Commit** : Version 0.1.4
- **Issue** : Détection manuelle lors de test utilisateur
- **Fichiers modifiés** :
  - `crates/rustwork-cli/src/mcp/common/diagnostics/collector.rs`
  - `crates/rustwork-cli/src/mcp/common/workspace_root/types.rs` (ajout de `cargo_workspace_dir()`)
  - `crates/rustwork-cli/src/mcp/common/workspace_root/mod.rs` (tests)
  - `crates/rustwork-cli/src/mcp/common/state/watchers.rs`
  - `Cargo.toml` (version bump)
  - `CHANGELOG.md`
