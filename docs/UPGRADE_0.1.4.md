# Mise à jour vers Rustwork 0.1.4

## 🎯 Résumé

La version **0.1.4** corrige un bug critique P0 : `rustwork_get_diagnostics` capture désormais correctement les erreurs de compilation du projet utilisateur.

## 📦 Installation

### Depuis crates.io (lorsque disponible)

```bash
cargo install rustwork-cli
```

### Depuis les sources

```bash
git clone https://github.com/entcorporg/rustwork.git
cd rustwork
git checkout main  # ou test pour la version de développement
cargo install --path crates/rustwork-cli --force
```

## ✅ Vérification de l'installation

```bash
rustwork --version
# Doit afficher : rustwork 0.1.4
```

## 🔄 Redémarrage du serveur MCP (OBLIGATOIRE)

Après l'installation, le serveur MCP doit être redémarré pour utiliser la nouvelle version.

### Pour VS Code

1. **Méthode 1** : Recharger la fenêtre
   - Appuyez sur `Ctrl+Shift+P` (ou `Cmd+Shift+P` sur Mac)
   - Tapez "Developer: Reload Window"
   - Appuyez sur Entrée

2. **Méthode 2** : Fermer et rouvrir VS Code
   - Fermez complètement VS Code
   - Rouvrez votre projet

### Vérification

Après le redémarrage, vérifiez que le nouveau serveur MCP est actif :

```bash
ps aux | grep "rustwork mcp"
```

Le processus doit avoir été créé **après** l'installation de 0.1.4.

## 🧪 Test du fix

### Scénario de test

1. Ouvrez votre projet Rustwork dans VS Code
2. Introduisez une erreur volontaire :
   ```rust
   // Dans n'importe quel fichier .rs
   mod module_inexistant;
   ```
3. Appelez `rustwork_get_diagnostics` via Copilot ou le MCP
4. **Attendez 10-15 secondes** (le collector fait un `cargo check` périodique)
5. Rappelez `rustwork_get_diagnostics`

### Résultat attendu

```json
{
  "errors": 1,
  "warnings": 0,
  "total": 1,
  "last_build_success": false,
  "index_state": "ready",
  "index_files_count": 30,
  "index_is_ready": true,
  "diagnostics": [
    {
      "severity": "error",
      "message": "file not found for module `module_inexistant`",
      "file": "src/main.rs",
      "line": 6,
      "column": 1
    }
  ]
}
```

## ⏱️ Timing important

Le diagnostic collector lance `cargo check` :
- **Immédiatement** au démarrage du serveur MCP
- **Puis toutes les 10 secondes**

Si vous venez d'introduire une erreur, attendez jusqu'à 15 secondes avant de rappeler `rustwork_get_diagnostics`.

## 🐛 Dépannage

### Le serveur MCP ne démarre pas

```bash
# Vérifier les logs stderr
# Le serveur MCP log sur stderr quand lancé en stdio mode
```

### `rustwork_get_diagnostics` retourne toujours 0 erreur

1. **Vérifiez la version du binaire** :
   ```bash
   rustwork --version  # Doit être 0.1.4
   ```

2. **Vérifiez le processus en cours** :
   ```bash
   ps aux | grep "rustwork mcp"
   ```
   Le processus doit être récent (créé après l'installation)

3. **Tuez le processus et rechargez VS Code** :
   ```bash
   pkill -f "rustwork mcp"
   # Puis rechargez VS Code
   ```

4. **Vérifiez le workspace** :
   - Le serveur MCP doit être lancé dans le **projet utilisateur**, pas dans le workspace Rustwork lui-même
   - Vérifiez votre `.vscode/mcp.json` :
     ```json
     {
       "servers": {
         "rustwork": {
           "type": "stdio",
           "command": "rustwork",
           "args": ["mcp", "--stdio", "--project", "."]
         }
       }
     }
     ```

### Les diagnostics sont retardés

C'est normal : le collector lance `cargo check` toutes les 10 secondes. Après une modification, attendez ce délai avant de rappeler `rustwork_get_diagnostics`.

## 📝 Changements techniques

Pour les développeurs :

- **DiagnosticCollector** : accepte maintenant `workspace_path: PathBuf`
- **cargo check** : utilise `.current_dir(&workspace_path)` pour s'exécuter dans le bon répertoire
- **LiveProjectState** : passe `workspace_root.path()` au collector

Voir [BUG_FIX_DIAGNOSTICS_P0.md](BUG_FIX_DIAGNOSTICS_P0.md) pour l'analyse complète.

## 🔗 Ressources

- [CHANGELOG.md](../CHANGELOG.md)
- [BUG_FIX_DIAGNOSTICS_P0.md](BUG_FIX_DIAGNOSTICS_P0.md)
- [VSCODE_MCP.md](VSCODE_MCP.md)
