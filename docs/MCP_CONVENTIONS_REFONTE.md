# Refonte du tool MCP rustwork_get_conventions

## ✅ Objectif atteint

Le tool `rustwork_get_conventions` a été restructuré pour passer d'un système monolithique à un système **hiérarchique, navigable et surchargeable**.

## 🎯 Résultat

### Avant (v0.2.3)

```rust
// Retournait un gros bloc JSON non filtrable
{
  "error_handling": { ... },
  "response": { ... },
  "handler_patterns": { ... },
  ...
}
```

**Problèmes** :
- ❌ Bloc monolithique verbeux
- ❌ Non filtrable par l'IA
- ❌ Impossible à personnaliser par projet
- ❌ Conventions mélangées au code Rust

### Après (v0.2.4)

```rust
// 3 modes d'exploration
rustwork_get_conventions()                           // → catégories racines
rustwork_get_conventions(category: "database")       // → sous-catégories
rustwork_get_conventions(path: "database.migrations") // → règle précise
```

**Avantages** :
- ✅ Navigation progressive par l'IA
- ✅ Conventions projet écrasent celles du framework
- ✅ Éditable hors code (`.rustwork/conventions.json`)
- ✅ CLI : `rustwork conventions init`

## 📦 Nouveaux fichiers créés

### Structure de données
- `crates/rustwork-cli/src/mcp/rustwork_get_conventions/types.rs`
  - `Convention`, `ConventionRule`, `ConventionExample`
  - Enums : `ConventionScope`, `Criticality`, `ConventionContext`

### Loader avec priorité
- `crates/rustwork-cli/src/mcp/rustwork_get_conventions/loader.rs`
  - `ConventionLoader` : charge framework + projet
  - Règle : **projet > framework** (non négociable)

### Conventions framework (embedded)
- `crates/rustwork-cli/data/conventions/framework.json`
  - Catégories : `http`, `errors`, `responses`, `database`, `config`, `microservices`, `testing`

### Template conventions projet
- `crates/rustwork-cli/data/conventions/template_project_conventions.json`
  - Exemple de surcharge de catégorie
  - Généré par `rustwork conventions init`

### Commande CLI
- `crates/rustwork-cli/src/commands/conventions.rs`
  - `rustwork conventions init` : crée `.rustwork/conventions.json`

### Tests
- `crates/rustwork-cli/src/mcp/rustwork_get_conventions/tests.rs`
  - 8 tests unitaires couvrant tous les cas

### Documentation
- `docs/MCP_CONVENTIONS.md`
  - Guide complet du nouveau système

## 🔑 Règles respectées

### 1. Aucun nouveau tool MCP
✅ Évolution interne uniquement de `rustwork_get_conventions`

### 2. Priorité absolue : projet > framework
✅ Implémenté dans `ConventionLoader::merge_conventions()`

### 3. Éditable hors code
✅ Conventions dans `.rustwork/conventions.json`
✅ Pas de recompilation nécessaire pour les conventions projet

### 4. Hiérarchie navigable
✅ Arbre : catégories → sous-catégories → règles atomiques
✅ 3 modes : root, category, path

### 5. Aucune perte d'information
✅ Toutes les conventions existantes migrées vers `framework.json`
✅ Structure enrichie (exemples, rationale, ai_note)

## 🧪 Tests

### Tests unitaires
```bash
cargo test rustwork_get_conventions
```

**Résultat** : 8 tests passent
- ✅ Chargement framework
- ✅ Navigation par catégorie
- ✅ Navigation par path
- ✅ Priorité projet > framework
- ✅ Cas sans conventions projet
- ✅ Chemins invalides

### Tests d'intégration
```bash
# Test complet
cd /tmp && mkdir test-project && cd test-project
rustwork conventions init
# ✅ Fichier .rustwork/conventions.json créé
```

### Test suite complète
```bash
cargo test --workspace
```

**Résultat** : 73 tests passent (65 avant + 8 nouveaux)

## 📊 Modifications de fichiers

### Fichiers modifiés
1. `crates/rustwork-cli/src/mcp/rustwork_get_conventions/mod.rs`
   - Signature : ajout `arguments` et `state`
   - 3 modes d'exploration
   - Utilise `ConventionLoader`

2. `crates/rustwork-cli/src/mcp/tools/mcp_protocol/call_tool.rs`
   - Passe `arguments` et `state` au tool

3. `crates/rustwork-cli/src/mcp/tools/mcp_protocol/list_tools.rs`
   - Nouvelle description avec paramètres `category` et `path`

4. `crates/rustwork-cli/src/mcp/common/dispatcher/routes.rs`
   - Route avec `&None` pour compatibilité

5. `crates/rustwork-cli/src/commands/mod.rs`
   - Ajout `pub mod conventions;`

6. `crates/rustwork-cli/src/main.rs`
   - Ajout sous-commande `Conventions { Init }`

7. `Cargo.toml`
   - Version : `0.2.3` → `0.2.4`

8. `CHANGELOG.md`
   - Ajout section "Hierarchical conventions system"

### Fichiers créés
- `types.rs`, `loader.rs`, `tests.rs` (conventions)
- `framework.json`, `template_project_conventions.json`
- `conventions.rs` (commande CLI)
- `docs/MCP_CONVENTIONS.md`

## 🚀 Utilisation

### 1. Pour les utilisateurs

```bash
# Créer un projet Rustwork
rustwork new myproject

cd myproject

# Initialiser les conventions projet
rustwork conventions init

# Éditer .rustwork/conventions.json selon vos besoins
# Les conventions projet écrasent celles du framework
```

### 2. Pour les IA (via MCP)

```json
// Étape 1 : Explorer les catégories racines
{
  "name": "rustwork_get_conventions"
}

// Étape 2 : Explorer une catégorie
{
  "name": "rustwork_get_conventions",
  "arguments": { "category": "database" }
}

// Étape 3 : Récupérer une règle précise
{
  "name": "rustwork_get_conventions",
  "arguments": { "path": "database.migrations.naming" }
}
```

## 📝 Exemple de réponse

### Mode root
```json
{
  "mode": "root",
  "categories": [
    {
      "id": "http",
      "label": "HTTP & Handlers",
      "description": "...",
      "scope": "framework",
      "has_children": true,
      "has_rules": false
    }
  ],
  "hint": "Use 'category' parameter..."
}
```

### Mode path
```json
{
  "mode": "path",
  "path": "http.handlers",
  "convention": {
    "id": "handlers",
    "label": "Handler Patterns",
    "scope": "framework",
    "rules": [
      {
        "id": "basic_handler",
        "description": "Handler basique avec State uniquement",
        "examples": [...]
      }
    ]
  }
}
```

## 🎓 Documentation

Voir [docs/MCP_CONVENTIONS.md](../docs/MCP_CONVENTIONS.md) pour :
- Philosophie du système
- Exemples complets
- Structure des types
- Catégories disponibles
- Règles de priorité
- Cas d'usage

## ✅ Critères de validation

| Critère | Statut |
|---------|--------|
| IA peut explorer progressivement | ✅ 3 modes d'exploration |
| Conventions projet > framework | ✅ Implémenté et testé |
| Utilisable en contexte ciblé | ✅ Paramètres category/path |
| Éditable hors code | ✅ `.rustwork/conventions.json` |
| Aucune perte d'information | ✅ Migration complète |
| Tests passent | ✅ 73/73 tests |

## 🔒 Interdictions respectées

| Interdit | Respecté |
|----------|----------|
| Nouveau tool MCP | ✅ Évolution interne uniquement |
| Gros bloc verbeux par défaut | ✅ Catégories racines seulement |
| Ignorer conventions projet | ✅ Priorité absolue |
| Fusion implicite | ✅ Tout ou rien |
| Conventions dans le code | ✅ Fichiers JSON externes |

## 🎉 Conclusion

La refonte est **COMPLÈTE** et **VALIDÉE**.

- ✅ Tous les objectifs atteints
- ✅ Tous les tests passent
- ✅ Version 0.2.4 publiée
- ✅ Documentation complète
- ✅ CLI fonctionnelle

Le système de conventions est maintenant :
- **Hiérarchique** : navigation par niveaux
- **Surchargeable** : projet > framework
- **Extensible** : éditable hors code
- **Exploitable** : conçu pour l'IA
