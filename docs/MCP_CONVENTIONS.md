# Rustwork Conventions - Système hiérarchique

## Vue d'ensemble

Le tool MCP `rustwork_get_conventions` a été restructuré pour offrir un système hiérarchique, navigable et surchargeable de conventions.

## Philosophie

Les conventions ne sont pas de la documentation.  
**Ce sont des contraintes exploitables par une IA.**

## Caractéristiques principales

### 1. Hiérarchie navigable

Les conventions sont organisées en arbre :

```
catégories racines
  └── sous-catégories
      └── sous-sous-catégories
          └── règles atomiques
```

L'IA peut explorer progressivement selon ses besoins.

### 2. Priorité absolue : projet > framework

**Règle NON NÉGOCIABLE** :
- Les conventions projet écrasent **totalement** celles du framework
- Pas de fusion implicite
- Pas d'ambiguïté

### 3. Éditable hors code

Les conventions framework sont stockées dans :
```
crates/rustwork-cli/data/conventions/framework.json
```

Les conventions projet sont dans :
```
.rustwork/conventions.json
```

Aucune recompilation nécessaire pour modifier les conventions projet.

## Utilisation du tool MCP

### Mode 1 : Catégories racines (par défaut)

```json
{
  "name": "rustwork_get_conventions"
}
```

Retourne uniquement les catégories racines sans leur contenu.

**Réponse** :
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
    },
    ...
  ],
  "hint": "Use 'category' parameter to explore a specific category, or 'path' to get a precise convention"
}
```

### Mode 2 : Explorer une catégorie

```json
{
  "name": "rustwork_get_conventions",
  "arguments": {
    "category": "database"
  }
}
```

Retourne les sous-catégories de `database` sans les règles atomiques.

**Réponse** :
```json
{
  "mode": "category",
  "category": {
    "id": "database",
    "label": "Database",
    "description": "...",
    "scope": "framework",
    "children": [
      {
        "id": "connection",
        "label": "Connection",
        "description": "...",
        "has_children": false,
        "has_rules": true
      },
      {
        "id": "migrations",
        "label": "Migrations",
        "description": "...",
        "has_children": false,
        "has_rules": true
      }
    ]
  }
}
```

### Mode 3 : Récupérer une convention précise

```json
{
  "name": "rustwork_get_conventions",
  "arguments": {
    "path": "database.migrations.naming"
  }
}
```

Retourne la convention exacte avec :
- Règles atomiques
- Exemples de code
- Notes IA
- Provenance (framework/project)

**Réponse** :
```json
{
  "mode": "path",
  "path": "database.migrations.naming",
  "convention": {
    "id": "naming",
    "label": "Migration Naming",
    "description": "...",
    "scope": "framework",
    "criticality": "required",
    "rules": [
      {
        "id": "migration_naming",
        "description": "Nommer les migrations avec un timestamp : YYYYMMDDHHMMSS_description.sql",
        "rationale": "Assure l'ordre d'exécution et évite les conflits",
        "examples": [...]
      }
    ]
  }
}
```

## Commande CLI : conventions init

Pour initialiser les conventions projet :

```bash
rustwork conventions init
```

Crée `.rustwork/conventions.json` avec une structure de base et des exemples.

### Exemple de conventions projet

```json
[
  {
    "id": "http",
    "label": "HTTP Projet",
    "description": "Mes conventions projet - ÉCRASE les conventions framework",
    "scope": "project",
    "rules": [
      {
        "id": "custom_handler",
        "description": "Handler avec logging automatique",
        "examples": [...]
      }
    ]
  }
]
```

## Structure des conventions

### Type `Convention`

```rust
pub struct Convention {
    pub id: String,                         // Identifiant stable
    pub label: String,                      // Label lisible
    pub description: String,                // Description courte
    pub scope: ConventionScope,             // framework | project
    pub criticality: Option<Criticality>,   // required | recommended | optional
    pub context: Option<ConventionContext>, // monolith | microservice | both
    pub rules: Option<Vec<ConventionRule>>, // Règles atomiques
    pub children: Option<Vec<Convention>>,  // Sous-conventions
    pub ai_note: Option<String>,            // Note destinée à l'IA
    pub metadata: Option<HashMap<...>>,     // Métadonnées supplémentaires
}
```

### Type `ConventionRule`

```rust
pub struct ConventionRule {
    pub id: String,
    pub description: String,
    pub rationale: Option<String>,          // Pourquoi cette règle existe
    pub examples: Option<Vec<ConventionExample>>,
    pub ai_note: Option<String>,
}
```

### Type `ConventionExample`

```rust
pub struct ConventionExample {
    pub description: String,
    pub code: Option<String>,               // Code d'exemple
    pub language: Option<String>,           // rust, bash, json, etc.
}
```

## Catégories framework disponibles

### 1. HTTP & Handlers (`http`)
- `http.handlers` : Patterns de handlers Axum
- `http.routing` : Organisation des routes

### 2. Error Handling (`errors`)
- Gestion centralisée avec `AppError`
- Conversions depuis erreurs externes

### 3. API Responses (`responses`)
- Type `ApiResponse<T>`
- Méthodes `success()` et `error()`

### 4. Database (`database`)
- `database.connection` : Configuration BDD
- `database.migrations` : Gestion des migrations

### 5. Configuration (`config`)
- Variables d'environnement
- Structure des `.env`

### 6. Microservices (`microservices`)
- `microservices.grpc` : Communication gRPC
- `microservices.service_discovery` : Découverte de services

### 7. Testing (`testing`)
- Organisation des tests
- Tests unitaires vs intégration

## Règles de priorité (loader)

Le `ConventionLoader` applique la règle suivante :

```rust
// 1. Charger les conventions framework (embedded)
loader.load_framework_conventions()?;

// 2. Charger les conventions projet (.rustwork/conventions.json)
loader.load_project_conventions(&workspace_root)?;

// 3. Fusionner : projet écrase framework
let merged = loader.merge_conventions();
```

### Algorithme de fusion

```rust
// Si une convention projet a le même ID qu'une convention framework
if project_ids.contains(&framework_conv.id) {
    // La convention framework est IGNORÉE
    // La convention projet est utilisée
}
```

**Important** : Il n'y a PAS de fusion partielle. C'est tout ou rien.

## Cas d'usage

### Cas 1 : IA explorant les conventions

```
1. IA appelle rustwork_get_conventions() sans paramètres
   → Reçoit les catégories racines
2. IA identifie la catégorie pertinente : "database"
3. IA appelle rustwork_get_conventions(category: "database")
   → Reçoit les sous-catégories
4. IA appelle rustwork_get_conventions(path: "database.migrations.naming")
   → Reçoit la règle exacte avec exemples
```

### Cas 2 : Projet avec conventions personnalisées

```
1. Développeur crée .rustwork/conventions.json
2. Développeur surcharge la catégorie "http" avec ses propres handlers
3. IA appelle rustwork_get_conventions(path: "http.handlers")
   → Reçoit les conventions PROJET, pas framework
```

### Cas 3 : Pas de conventions projet

```
1. Projet sans .rustwork/conventions.json
2. IA appelle rustwork_get_conventions()
   → Reçoit uniquement les conventions framework
```

## Extensibilité

### Ajouter une nouvelle catégorie framework

1. Éditer `crates/rustwork-cli/data/conventions/framework.json`
2. Ajouter la nouvelle catégorie JSON
3. Recompiler le binaire (les conventions sont embedded)

### Ajouter une convention projet

1. Lancer `rustwork conventions init` (si pas déjà fait)
2. Éditer `.rustwork/conventions.json`
3. Ajouter la nouvelle convention
4. Pas de recompilation nécessaire

## Tests

Les tests couvrent :
- ✅ Chargement des conventions framework
- ✅ Navigation par catégorie
- ✅ Navigation par path
- ✅ Priorité projet > framework
- ✅ Cas sans conventions projet
- ✅ Chemins invalides retournent None

Lancer les tests :
```bash
cargo test rustwork_get_conventions
```

## Migration depuis l'ancienne version

### Avant

```rust
pub async fn rustwork_get_conventions() -> Result<Value, RpcError> {
    Ok(json!({
        "error_handling": { ... },
        "response": { ... },
        ...
    }))
}
```

**Problème** : Bloc JSON monolithique, non filtrable, non personnalisable.

### Après

```rust
pub async fn rustwork_get_conventions(
    arguments: &Option<Value>,
    state: Option<&LiveProjectState>,
) -> Result<Value, RpcError> {
    // Navigation hiérarchique
    // Priorité projet > framework
    // Éditable hors code
}
```

**Bénéfice** : IA peut explorer progressivement, conventions projet prioritaires.

## Interdictions

❌ Retourner un gros bloc verbeux par défaut  
❌ Ignorer les conventions projet  
❌ Fusionner framework et projet sans règle claire  
❌ Rendre les conventions dépendantes du code Rust  
❌ Introduire un nouveau tool MCP  

## Critères de validation

✅ Une IA peut explorer les conventions par étapes  
✅ Les conventions projet écrasent réellement celles du framework  
✅ `get_conventions` est utilisable en contexte ciblé  
✅ Les conventions sont éditables sans toucher au code Rust  
✅ Aucun comportement existant n'est perdu  

## Voir aussi

- [docs/MCP.md](../MCP.md) : Documentation générale du MCP
- [.github/copilot-instructions.md](../.github/copilot-instructions.md) : Instructions pour l'IA
