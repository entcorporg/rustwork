# Amélioration: Mode Workspace pour `rustwork dev`

## Résumé des Changements

La commande `rustwork dev` supporte maintenant nativement les architectures micro-services, permettant de détecter et lancer automatiquement plusieurs services Rustwork depuis n'importe quel dossier parent.

## Fichiers Modifiés

### 1. `crates/rustwork-cli/src/commands/utils.rs`
**Ajouts :**
- Structure `RustworkService` : représente un service détecté
- Fonction `is_valid_rustwork_service()` : validation stricte (manifest + Cargo.toml + src/main.rs)
- Fonction `detect_rustwork_services()` : détection récursive avec remontée des parents
- Fonction `scan_directory_for_services()` : scan récursif des sous-dossiers

**Logique :**
- Remonte les dossiers parents jusqu'à trouver des services ou atteindre la racine
- Scanne récursivement tous les sous-dossiers (ignore `.`, `target`, `node_modules`)
- Retourne tous les services valides détectés

### 2. `crates/rustwork-cli/src/commands/dev.rs`
**Refactorisation complète :**
- Fonction `start_service_with_watch()` : lance un service avec préfixage des logs
- Fonction `run_single_service()` : comportement classique (1 service)
- Fonction `run_multiple_services()` : orchestration multi-services
- Fonction `execute()` : dispatch selon le nombre de services détectés

**Nouveaux comportements :**
- **0 service** : erreur explicite avec aide
- **1 service** : mode classique (compatibilité)
- **N services** : mode workspace avec orchestration parallèle

**Fonctionnalités :**
- Lancement parallèle de tous les services avec cargo-watch
- Préfixage automatique des logs : `[service-name] log...`
- Détection du workspace root (ancêtre commun)
- MCP lancé une seule fois à la racine du workspace
- Gestion des erreurs : continue si un service échoue

## Documentation

### `docs/DEV_WORKSPACE.md`
Guide complet couvrant :
- Fonctionnalités et critères de détection
- Exemples d'architectures (monolithe, micro-services, monorepo)
- Format de sortie et préfixage
- Mode MCP centralisé
- Workflow recommandé
- Dépannage

### `test_dev_workspace.sh`
Script de test automatisé créant :
- Workspace temporaire avec 3 services (auth, user, blog)
- Configuration des ports (3001, 3002, 3003)
- 5 scénarios de test différents
- Instructions pour tests manuels

## Exemples d'Utilisation

### Monolithe (comportement inchangé)
```bash
cd my-app/
rustwork dev
# → Lance le service unique
```

### Micro-services
```bash
# Structure:
# backend/
#   services/
#     auth/
#     user/
#     blog/

cd backend/
rustwork dev
# → Détecte et lance les 3 services en parallèle

# Logs:
# [auth] Compiling auth v0.1.0...
# [user] Compiling user v0.1.0...
# [blog] Compiling blog v0.1.0...
# [auth] INFO: Server listening on 0.0.0.0:3001
# [user] INFO: Server listening on 0.0.0.0:3002
# [blog] INFO: Server listening on 0.0.0.0:3003
```

### Avec MCP
```bash
cd backend/
rustwork dev --mcp
# → Lance le MCP une fois pour observer tout le workspace
# → Lance tous les services
```

## Validation

✅ **Détection automatique** : remonte les dossiers parents  
✅ **Critères stricts** : manifest + Cargo.toml + src/main.rs  
✅ **Orchestration parallèle** : tous les services en même temps  
✅ **Logs préfixés** : `[service-name]` sur chaque ligne  
✅ **MCP centralisé** : un seul serveur pour le workspace  
✅ **Gestion d'erreurs** : continue si un service échoue  
✅ **Compatibilité** : comportement classique préservé pour 1 service  
✅ **Compilation** : aucune erreur Clippy  

## Tests

### Automatisé
```bash
./test_dev_workspace.sh
```

### Manuel
```bash
# 1. Créer un workspace
mkdir -p test/backend/services
cd test/backend/services

# 2. Créer des services
cargo run --bin rustwork new auth
cargo run --bin rustwork new user
cargo run --bin rustwork new blog

# 3. Configurer les ports (3001, 3002, 3003)

# 4. Tester depuis différents niveaux
cd ../..  # → test/
cargo run --bin rustwork dev

cd backend/  # → test/backend/
cargo run --bin rustwork dev

cd services/  # → test/backend/services/
cargo run --bin rustwork dev

cd auth/  # → test/backend/services/auth/
cargo run --bin rustwork dev  # Mode single-service
```

## Impacts

### Changements Breaking
❌ **Aucun** - Compatibilité totale avec comportement existant

### Nouveaux Comportements
✅ Détection automatique depuis dossiers parents  
✅ Mode workspace multi-services  
✅ Préfixage des logs  
✅ MCP centralisé  

### Exigences
- `cargo-watch` requis (inchangé)
- Les services doivent avoir des ports différents

## Notes Techniques

### Algorithme de Détection
1. À partir du cwd, remonter les parents
2. Pour chaque niveau, scanner récursivement
3. Valider chaque service trouvé (3 critères)
4. Arrêter dès qu'au moins 1 service est trouvé
5. Éviter les boucles avec un HashSet

### Orchestration Multi-Process
- Un process `cargo-watch` par service
- Thread séparé pour streamer stdout/stderr
- Préfixage dans le thread de streaming
- Arc<Mutex<Vec<Child>>> pour la gestion centralisée
- Polling non-bloquant avec `try_wait()`

### Workspace Root
- Calculé comme l'ancêtre commun le plus haut
- Utilisé pour le MCP (observer tout le workspace)
- Les services peuvent être à différents niveaux

## Prochaines Étapes Possibles

Améliorations futures (non incluses) :
- [ ] Support de profils de lancement (launch groups)
- [ ] Configuration `.rustwork/workspace.toml`
- [ ] Orchestration avancée (dépendances entre services)
- [ ] Interface TUI pour contrôler les services individuellement
- [ ] Logs colorés par service
- [ ] Agrégation de métriques multi-services

## Validation de la Spec

| Critère | Status |
|---------|--------|
| Lance depuis n'importe quel dossier parent | ✅ |
| Détecte automatiquement tous les services | ✅ |
| Critères stricts (manifest + Cargo + main.rs) | ✅ |
| Orchestre plusieurs services en parallèle | ✅ |
| Préfixe les logs par service | ✅ |
| MCP lancé une fois pour tout le workspace | ✅ |
| Continue si un service échoue | ✅ |
| Messages d'erreur clairs | ✅ |
| Compatibilité avec mode single-service | ✅ |
| Aucune modification du runtime Rustwork | ✅ |
| Aucune modification MCP (sauf init) | ✅ |
| Documentation complète | ✅ |

**Tous les critères sont validés ✅**
