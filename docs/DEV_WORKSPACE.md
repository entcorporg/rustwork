# Rustwork Dev - Mode Workspace Multi-Services

## Vue d'ensemble

La commande `rustwork dev` supporte maintenant nativement les architectures micro-services, permettant de lancer automatiquement tous les services détectés depuis n'importe quel dossier parent.

## Fonctionnalités

### Détection Automatique

`rustwork dev` détecte automatiquement tous les services Rustwork valides :

- Remonte les dossiers parents jusqu'à la racine si nécessaire
- Scanne récursivement tous les sous-dossiers
- Détecte les services basés sur des critères stricts

### Critères de Service Valide

Un service Rustwork est reconnu s'il possède **TOUS** ces éléments :

```
service/
├── .rustwork/
│   └── manifest.json
├── Cargo.toml
└── src/
    └── main.rs
```

### Comportements

#### 1. Aucun service trouvé
```bash
$ rustwork dev
❌ Not in a Rustwork project or workspace.
   No Rustwork services found in current directory or children.
   
   A valid Rustwork service must have:
   - .rustwork/manifest.json
   - Cargo.toml
   - src/main.rs
```

#### 2. Un seul service (monolithe)
```bash
$ rustwork dev
🔧 Starting development server with hot-reload...
   Watching for changes in src/
ℹ️  MCP server disabled. Use --mcp to enable it.

[Finished running. Exit status: ...]
```

#### 3. Plusieurs services (micro-services)
```bash
$ rustwork dev
🔧 Starting Rustwork development workspace...
🔍 Detected 3 Rustwork service(s):
  - auth (services/auth)
  - user (services/user)
  - blog (services/blog)

ℹ️  MCP server disabled. Use --mcp to enable it.

▶ Starting auth...
▶ Starting user...
▶ Starting blog...

✅ All services started. Press Ctrl+C to stop all services.

[auth] Compiling auth v0.1.0 (/path/to/services/auth)
[user] Compiling user v0.1.0 (/path/to/services/user)
[blog] Compiling blog v0.1.0 (/path/to/services/blog)
...
```

## Préfixage des Logs

Chaque ligne de log est automatiquement préfixée avec le nom du service :

```
[auth] INFO: Server listening on 0.0.0.0:3001
[user] INFO: Server listening on 0.0.0.0:3002
[blog] INFO: Server listening on 0.0.0.0:3003
[auth] INFO: Database connected
[user] ERROR: Failed to connect to cache
```

## Mode MCP

### Sans MCP (par défaut)
```bash
$ rustwork dev
ℹ️  MCP server disabled. Use --mcp to enable it.
```

### Avec MCP
```bash
$ rustwork dev --mcp
🚀 Starting MCP server on 127.0.0.1:4000... (development only)
   MCP observing workspace: /path/to/workspace/root
   Press Ctrl+C to stop
```

Le serveur MCP est lancé **UNE SEULE FOIS** à la racine du workspace et observe **TOUS** les services.

## Exemples d'Architectures

### Monolithe Classique
```
my-app/
├── .rustwork/
│   └── manifest.json
├── Cargo.toml
├── src/
│   └── main.rs
└── config/
```

**Commande :**
```bash
cd my-app
rustwork dev
```

**Résultat :** Lance le service unique

---

### Micro-services Simple
```
backend/
└── services/
    ├── auth/
    │   ├── .rustwork/
    │   ├── Cargo.toml
    │   └── src/main.rs
    ├── user/
    │   ├── .rustwork/
    │   ├── Cargo.toml
    │   └── src/main.rs
    └── blog/
        ├── .rustwork/
        ├── Cargo.toml
        └── src/main.rs
```

**Commandes valides :**
```bash
# Depuis la racine
cd backend
rustwork dev

# Depuis services/
cd backend/services
rustwork dev

# Depuis n'importe quel parent
cd /path/to/parent/backend
rustwork dev
```

**Résultat :** Lance les 3 services en parallèle

---

### Monorepo Complexe
```
project/
├── frontend/
├── backend/
│   └── services/
│       ├── api/
│       │   ├── .rustwork/
│       │   ├── Cargo.toml
│       │   └── src/main.rs
│       └── worker/
│           ├── .rustwork/
│           ├── Cargo.toml
│           └── src/main.rs
└── docs/
```

**Commande :**
```bash
cd project/backend
rustwork dev
```

**Résultat :** Lance api et worker

---

## Gestion des Erreurs

### Service qui échoue
Si un service échoue au démarrage, les autres continuent :

```bash
▶ Starting auth...
▶ Starting user...
⚠️  Failed to start user: Failed to start cargo watch
   Continuing with other services...
▶ Starting blog...

✅ All services started. Press Ctrl+C to stop all services.
```

### Cargo-watch non installé
```bash
⚠️  cargo-watch not found.
   Run: cargo install cargo-watch
❌ cargo-watch is required for dev mode
```

## Avantages

✅ **Pas de CD manuel** - Lancez depuis n'importe où  
✅ **Détection automatique** - Pas de configuration  
✅ **Logs séparés** - Préfixe par service  
✅ **Hot-reload** - Cargo-watch sur chaque service  
✅ **MCP centralisé** - Un seul serveur pour tout  
✅ **Gestion des erreurs** - Continue si un service échoue  
✅ **UX unifiée** - Expérience "workspace" native  

## Limitations

- Nécessite `cargo-watch` installé
- Les services doivent avoir des ports différents (à configurer dans `config/default.toml`)
- Le MCP observe le workspace entier, pas service par service

## Workflow Recommandé

```bash
# 1. Créer le workspace
mkdir -p backend/services
cd backend/services

# 2. Créer les services
rustwork new auth
rustwork new user
rustwork new blog

# 3. Configurer les ports dans chaque service
# backend/services/auth/config/default.toml
[server]
port = 3001

# backend/services/user/config/default.toml
[server]
port = 3002

# backend/services/blog/config/default.toml
[server]
port = 3003

# 4. Lancer tout depuis la racine
cd ../..  # Retour à backend/
rustwork dev

# Ou avec MCP
rustwork dev --mcp
```

## Dépannage

### "No Rustwork services found"
Vérifiez que vos services ont bien :
- `.rustwork/manifest.json`
- `Cargo.toml`
- `src/main.rs`

### Ports en conflit
Assurez-vous que chaque service a un port unique dans `config/default.toml`.

### Logs illisibles
Les logs sont préfixés par `[service-name]`. Si trop de services, considérez lancer individuellement :
```bash
cd services/auth
rustwork dev
```
