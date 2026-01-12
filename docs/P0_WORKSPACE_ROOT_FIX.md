# CORRECTION P0 : Détection du Workspace Root

**Version :** 0.2.0  
**Date :** 12 janvier 2026  
**Priorité :** P0 (Critique)

## Problème Résolu

Le MCP Rustwork ne détectait pas correctement le workspace root dans les environnements multi-services, causant :
- Indexation incomplète ou erronée des services
- `get_file_doc` inutilisable sur certains fichiers
- `rustwork dev` qui lançait depuis un mauvais répertoire
- Perte de confiance de l'IA dans les réponses MCP

### Cas Réel Observé

```
Commande lancée depuis : /home/linux/test
MCP détectait         : /home/linux/api_test
Services réels situés : /home/linux/test/backend/services/*
```

## Solution Implémentée

### 1. Détection Formelle du Workspace Root

**Règles appliquées (ordre strict) :**

1. **Si `--path` est fourni** → utiliser ce chemin sans discussion
2. **Sinon, remontée récursive depuis le CWD :**
   - Chercher un `Cargo.toml` avec section `[workspace]`
   - OU un dossier contenant PLUSIEURS projets Rustwork valides
3. **Si aucun workspace trouvé** → FAIL FAST avec erreur explicite

**Définition d'un "projet Rustwork valide" :**
- `.rustwork/manifest.json` (obligatoire)
- `Cargo.toml` (obligatoire)
- `src/main.rs` (obligatoire)

### 2. Fichiers Modifiés

#### `/crates/rustwork-cli/src/mcp/common/workspace_root/detection.rs`
- Refactorisation complète de la détection
- Ajout de `detect_with_explicit()` pour supporter `--path`
- Validation stricte des workspaces (Cargo.toml workspace OU multiples projets Rustwork)
- Fail fast avec messages d'erreur explicites

#### `/crates/rustwork-cli/src/mcp/common/workspace_root/helpers.rs`
- Nouvelle fonction `is_valid_rustwork_project()` : vérifie les 3 fichiers requis
- Nouvelle fonction `count_rustwork_projects_in_children()` : compte les projets valides
- Support des patterns : `services/`, `backend/services/`, et enfants directs

#### `/crates/rustwork-cli/src/mcp/common/service_resolver/resolution.rs`
- Résolution dynamique des services basée sur `find_all_rustwork_services()`
- Support de plusieurs patterns de dossiers de services
- Messages d'erreur précis pour les fichiers hors services

#### `/crates/rustwork-cli/src/mcp/common/service_resolver/helpers.rs`
- Nouvelle fonction `find_all_rustwork_services()` : scan intelligent du workspace
- Support des layouts : `services/`, `backend/services/`, monolithe

#### `/crates/rustwork-cli/src/commands/dev.rs`
- Refactorisation complète de `execute()` avec paramètre `explicit_path`
- Utilisation systématique de `WorkspaceRoot::detect()` ou `detect_with_explicit()`
- Élimination de la logique d'ancêtre commun heuristique
- Logs enrichis : affichage du workspace root détecté et du layout

#### `/crates/rustwork-cli/src/main.rs`
- Ajout de l'option `--path` à la commande `dev`

### 3. Tests Unitaires

Tous les tests ont été mis à jour pour respecter la définition stricte d'un projet Rustwork valide :
- `test_detect_monolith` : workspace monolithe avec projet valide
- `test_detect_microservices` : workspace avec 2+ services valides
- `test_cargo_workspace_detection` : détection via `[workspace]` dans Cargo.toml
- `test_resolve_service` : résolution de service avec manifest.json
- `test_list_services` : liste des services avec projets valides

**Résultat :** 71 tests passent, 0 échecs

## Impacts

### Positifs
✅ Détection robuste et prévisible du workspace root  
✅ Support de `--path` pour spécification explicite  
✅ Fail fast avec messages d'erreur clairs  
✅ Élimination de toute dépendance implicite au CWD  
✅ Support multi-patterns : `services/`, `backend/services/`, monolithe  
✅ MCP travaille sur le bon workspace dès le démarrage  

### Breaking Changes
⚠️ **Les projets DOIVENT maintenant avoir :**
- `.rustwork/manifest.json`
- `Cargo.toml`
- `src/main.rs`

⚠️ **Les services sans ces fichiers ne seront PLUS détectés**

## Utilisation

### Option 1 : Détection Automatique

```bash
cd /home/user/my-workspace
rustwork dev --mcp
```

Le MCP remonte automatiquement pour trouver le workspace root valide.

### Option 2 : Spécification Explicite

```bash
cd /home/user/anywhere
rustwork dev --mcp --path /home/user/my-workspace
```

Le MCP utilise le chemin fourni sans recherche.

### Vérification du Workspace Détecté

Les logs MCP affichent désormais :
```
✅ Workspace root detected: /home/user/my-workspace
📐 Layout: MicroServices
🔍 Detected 3 Rustwork service(s):
  - auth (services/auth)
  - user (services/user)
  - payment (services/payment)
```

## Validation

- ✅ Compilation sans erreurs ni warnings
- ✅ Tous les tests unitaires passent (71/71)
- ✅ Installation du binaire réussie
- ✅ Version incrémentée : 0.1.1 → 0.2.0

## Prochaines Étapes Recommandées

1. Tester sur un vrai workspace multi-services
2. Valider `get_file_doc` sur tous les services
3. Vérifier la stabilité de l'indexation MCP
4. Documenter les patterns de workspace supportés
