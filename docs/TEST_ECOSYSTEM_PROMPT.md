# Prompt de test Rustwork - Écosystème Microservices

> À utiliser par une instance Copilot pour valider l'écosystème Rustwork après modifications.
> 
> **Branche de test :** `test` sur https://github.com/entcorporg/rustwork.git

## Contexte

Tu vas tester l'écosystème Rustwork, un framework backend Rust 100% microservices.

Rustwork est :
- Un framework backend Rust (Axum, SeaORM)
- Une CLI de génération de projets microservices
- Un serveur MCP (Model Context Protocol) pour intégration VS Code
- Un système gRPC avec DSL `.rwk`

**Important :** Rustwork est 100% microservices - aucun support monolithe.

---

## Tests à effectuer

### 1. Test de la CLI - Génération de projet

```bash
cd /tmp && rm -rf test-rustwork && mkdir test-rustwork && cd test-rustwork
rustwork new auth,user,session
```

**Vérifier :**
- [ ] Structure `Backend/services/auth/`, `Backend/services/user/`, `Backend/services/session/`
- [ ] Dossier `Backend/services/shared/` créé automatiquement
- [ ] Fichiers `.rustwork/manifest.json` dans chaque service
- [ ] `Cargo.toml` workspace à la racine du Backend
- [ ] Fichiers `.vscode/mcp.example.json` à la racine

---

### 2. Test de compilation des services générés

```bash
cd /tmp/test-rustwork/Backend
cargo build --workspace
```

**Vérifier :**
- [ ] Compilation réussie sans erreur
- [ ] Tous les services compilent (auth, user, session, shared)

---

### 3. Test de la commande rustwork dev

```bash
cd /tmp/test-rustwork
rustwork dev
```

**Vérifier :**
- [ ] Tous les services démarrent (auth, user, session)
- [ ] Logs préfixés par nom de service `[auth]`, `[user]`, `[session]`
- [ ] Ports différents pour chaque service (3001, 3002, 3003)
- [ ] Ctrl+C arrête proprement tous les services

---

### 4. Test du MCP server

```bash
cd /tmp/test-rustwork
rustwork mcp --stdio
```

Envoyer cette requête JSON-RPC sur stdin :
```json
{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}
```

**Vérifier :**
- [ ] Réponse JSON-RPC valide
- [ ] Liste des tools MCP retournée :
  - `rustwork_get_conventions`
  - `rustwork_get_routes`
  - `rustwork_get_file_doc`
  - `rustwork_get_env_setup`
  - `rustwork_get_diagnostics`
  - `rustwork_grpc_list_definitions`
  - etc.

---

### 5. Test d'ajout de service

```bash
cd /tmp/test-rustwork
rustwork add-service payment
```

**Vérifier :**
- [ ] `Backend/services/payment/` créé
- [ ] Structure identique aux autres services
- [ ] `.rustwork/manifest.json` présent
- [ ] Service ajouté au workspace Cargo.toml

---

### 6. Test sans shared library

```bash
cd /tmp && rm -rf test-no-shared && mkdir test-no-shared && cd test-no-shared
rustwork new api,worker --no-shared
```

**Vérifier :**
- [ ] Pas de dossier `shared/` dans `Backend/services/`
- [ ] Services `api` et `worker` créés normalement

---

### 7. Test des erreurs de validation

```bash
# Noms de services invalides
rustwork new 123invalid      # commence par un chiffre
rustwork new auth-service    # tirets interdits (utiliser underscores)
rustwork new ""              # vide
rustwork new auth user       # espaces au lieu de virgules
```

**Vérifier :**
- [ ] Messages d'erreur clairs et explicites
- [ ] Aucun fichier créé en cas d'erreur

---

### 8. Test de détection du workspace

```bash
cd /tmp/test-rustwork/Backend/services/auth
rustwork dev
```

**Vérifier :**
- [ ] `rustwork dev` fonctionne depuis un sous-dossier
- [ ] Détecte automatiquement la racine du workspace
- [ ] Lance tous les services (pas seulement auth)

---

## Critères de succès

| Test | Statut |
|------|--------|
| Génération de projet | ⬜ |
| Compilation des services | ⬜ |
| `rustwork dev` | ⬜ |
| MCP server | ⬜ |
| Ajout de service | ⬜ |
| Option `--no-shared` | ⬜ |
| Erreurs de validation | ⬜ |
| Détection du workspace | ⬜ |

---

## Points d'attention

1. **100% Microservices** - Aucune option `--layout`, aucun mode monolithe
2. **Syntaxe de commande** - `rustwork new auth,user,session` (services séparés par virgules)
3. **Structure obligatoire** - `Backend/services/<service>/`
4. **Shared library** - Créée par défaut, désactivable avec `--no-shared`
5. **MCP** - Serveur dynamique connecté à cargo watch

---

## Commandes utiles

```bash
# Aide
rustwork --help
rustwork new --help

# Vérifier la version
rustwork --version

# Lancer avec MCP
rustwork dev --mcp

# Générer un controller
rustwork make:controller users

# Générer un model
rustwork make:model User
```

---

## Structure attendue après `rustwork new auth,user,session`

```
./
├── .vscode/
│   └── mcp.example.json
├── Backend/
│   ├── Cargo.toml              # Workspace
│   ├── README.md
│   └── services/
│       ├── auth/
│       │   ├── .rustwork/
│       │   ├── config/
│       │   ├── migration/
│       │   ├── src/
│       │   ├── Cargo.toml
│       │   └── .env.example
│       ├── user/
│       │   └── ...
│       ├── session/
│       │   └── ...
│       └── shared/
│           ├── src/
│           │   ├── lib.rs
│           │   ├── types/
│           │   └── utils/
│           └── Cargo.toml
└── README.md
```
