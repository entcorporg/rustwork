# MCP Server - Model Context Protocol

Le serveur MCP de Rustwork fournit une **cartographie en direct** de votre projet Rust REST API.

## 🎯 Objectifs

- **Indexation dynamique** du code source (fichiers, fonctions, structs)
- **Analyse des routes** Axum (mapping vers handlers)
- **Call graph** (qui appelle quoi)
- **Diagnostics** en temps réel (erreurs, warnings)
- **Mises à jour live** via file watcher

## 🚀 Démarrage

### Automatique avec `rustwork dev`
```bash
rustwork dev
```
Le serveur MCP démarre automatiquement sur `127.0.0.1:4000`.

### Manuel
```bash
rustwork mcp --host 127.0.0.1 --port 4000 --project .
```

## 📡 Protocole

### JSON-RPC 2.0 over TCP

Le serveur utilise JSON-RPC 2.0 sur TCP avec messages délimités par `\n`.

**Format de requête :**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "get_routes",
  "params": {}
}
```

**Format de réponse :**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": { ... }
}
```

## 🔌 Endpoints (Méthodes)

### 1. `get_project_info`
Informations générales sur le projet.

**Params:** `{}`

**Résultat:**
```json
{
  "rustwork_version": "0.1.0",
  "project_name": "my-api",
  "database": { ... }
}
```

---

### 2. `get_files`
Liste tous les fichiers .rs indexés.

**Params:** `{}`

**Résultat:**
```json
{
  "files": [
    {
      "path": "src/main.rs",
      "module_path": "main",
      "functions_count": 5,
      "structs_count": 2
    }
  ],
  "total": 10
}
```

---

### 3. `get_file_doc`
Documentation complète d'un fichier (fonctions, structs).

**Params:**
```json
{
  "path": "src/controllers/user.rs"
}
```

**Résultat:**
```json
{
  "path": "src/controllers/user.rs",
  "module_path": "controllers::user",
  "functions": [
    {
      "name": "list_users",
      "is_public": true,
      "is_async": true,
      "line": 10,
      "signature": "pub async fn list_users(...) -> Result<ApiResponse<Vec<User>>, AppError>",
      "calls": ["fetch_users", "map"],
      "parameters": [
        {"name": "state", "type_name": "State<AppState>"}
      ],
      "return_type": "Result<ApiResponse<Vec<User>>, AppError>"
    }
  ],
  "structs": [...]
}
```

---

### 4. `get_routes`
Liste toutes les routes HTTP Axum détectées.

**Params:** `{}`

**Résultat:**
```json
{
  "routes": [
    {
      "method": "GET",
      "path": "/users",
      "handler": "list_users",
      "handler_function": "list_users",
      "file": "src/routes.rs",
      "line": 15
    },
    {
      "method": "POST",
      "path": "/users",
      "handler": "create_user",
      "handler_function": "create_user",
      "file": "src/routes.rs",
      "line": 16
    }
  ],
  "total": 25
}
```

---

### 5. `get_functions`
Liste des fonctions (optionnellement filtrées par fichier).

**Params:**
```json
{
  "file": "src/controllers/user.rs"  // optionnel
}
```

**Résultat:**
```json
{
  "functions": [
    {
      "name": "list_users",
      "is_public": true,
      "is_async": true,
      "line": 10,
      "signature": "pub async fn list_users(...)",
      "file": "src/controllers/user.rs"
    }
  ],
  "total": 50
}
```

---

### 6. `get_function_usage`
Trouve qui appelle une fonction donnée + routes associées.

**Params:**
```json
{
  "function": "fetch_users"
}
```

**Résultat:**
```json
{
  "function": "fetch_users",
  "callers": [
    "controllers::user::list_users",
    "controllers::admin::get_all_users"
  ],
  "routes": [
    {
      "method": "GET",
      "path": "/users",
      "handler": "list_users",
      ...
    }
  ]
}
```

---

### 7. `get_call_graph`
Graphe d'appels d'une fonction (profondeur configurable).

**Params:**
```json
{
  "function": "list_users",
  "depth": 5  // optionnel, défaut: 5
}
```

**Résultat:**
```json
{
  "function": "list_users",
  "depth": 5,
  "calls": {
    "fetch_users": 1,
    "map": 1,
    "User::from_model": 2,
    "db_query": 2
  }
}
```

La valeur indique la profondeur dans le graphe.

---

### 8. `get_route_impact`
Analyse l'impact d'une route : handler + fonctions appelées + fichiers affectés.

**Params:**
```json
{
  "method": "GET",
  "path": "/users"
}
```

**Résultat:**
```json
{
  "route": {
    "method": "GET",
    "path": "/users",
    "handler": "list_users",
    ...
  },
  "handler": "list_users",
  "called_functions": {
    "fetch_users": 1,
    "User::from_model": 2
  },
  "affected_files": [
    "src/routes.rs",
    "src/controllers/user.rs",
    "src/models/user.rs"
  ]
}
```

---

### 9. `get_diagnostics`
Diagnostics du projet (erreurs rustc, warnings, clippy).

**Params:** `{}`

**Résultat:**
```json
{
  "diagnostics": [
    {
      "severity": "error",
      "message": "cannot find value `x` in this scope",
      "file": "src/main.rs",
      "line": 42,
      "column": 5,
      "code": "E0425",
      "source": "rustc",
      "timestamp": 1704672000
    }
  ],
  "errors": 1,
  "warnings": 3,
  "last_build_success": false
}
```

---

### 10. `get_conventions`
Documentation des conventions Rustwork.

**Params:** `{}`

**Résultat:**
```json
{
  "error_handling": {
    "type": "AppError",
    "location": "src/errors.rs",
    "variants": ["NotFound", "BadRequest", ...],
    "usage": "Return Result<T, AppError> from handlers"
  },
  "response_format": { ... },
  "handler_signature": { ... },
  "configuration": { ... },
  "middleware": { ... },
  "database": { ... }
}
```

---

## 🔄 Mises à jour dynamiques

Le serveur surveille les fichiers `.rs` dans `src/` :
- **Modification** : réindexation automatique
- **Création** : ajout à l'index
- **Suppression** : retrait de l'index

Les appels aux endpoints retournent toujours les données **à jour**.

---

## 🔒 Sécurité

- **Localhost uniquement** (`127.0.0.1`)
- **Read-only** (pas d'exécution de code, pas d'écriture fs)
- **Sanitization** des secrets (mots de passe, tokens, clés)

---

## 📝 Exemple de client

### Python (simple)
```python
import socket
import json

def mcp_call(method, params=None):
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.connect(("127.0.0.1", 4000))
    
    request = {
        "jsonrpc": "2.0",
        "id": 1,
        "method": method,
        "params": params or {}
    }
    
    sock.sendall((json.dumps(request) + "\n").encode())
    response = sock.recv(4096).decode()
    sock.close()
    
    return json.loads(response)

# Exemples
print(mcp_call("get_project_info"))
print(mcp_call("get_routes"))
print(mcp_call("get_file_doc", {"path": "src/main.rs"}))
```

### Node.js
```javascript
const net = require('net');

function mcpCall(method, params = {}) {
  return new Promise((resolve, reject) => {
    const client = net.connect(4000, '127.0.0.1');
    
    const request = {
      jsonrpc: "2.0",
      id: 1,
      method,
      params
    };
    
    client.write(JSON.stringify(request) + '\n');
    
    client.on('data', (data) => {
      resolve(JSON.parse(data.toString()));
      client.end();
    });
    
    client.on('error', reject);
  });
}

// Exemples
mcpCall('get_routes').then(console.log);
mcpCall('get_function_usage', { function: 'list_users' }).then(console.log);
```

---

## 🧪 Tests

### Test manuel avec netcat
```bash
echo '{"jsonrpc":"2.0","id":1,"method":"get_project_info","params":{}}' | nc 127.0.0.1 4000
```

### Test manuel avec telnet
```bash
telnet 127.0.0.1 4000
{"jsonrpc":"2.0","id":1,"method":"get_routes","params":{}}
```

---

## 🏗️ Architecture

```
mcp/
├── mod.rs              # Module exports
├── server.rs           # TCP server + JSON-RPC handler
├── protocol.rs         # JSON-RPC types (Request, Response, Error)
├── handlers.rs         # Implémentation des méthodes
├── state.rs            # LiveProjectState (état partagé)
├── indexer.rs          # Indexation du code avec syn
├── routes.rs           # Analyse des routes Axum
├── diagnostics.rs      # Collecte des erreurs/warnings
└── watcher.rs          # File watcher (notify)
```

---

## 🎯 Cas d'usage

### 1. IDE Integration
Fournir de l'auto-complétion contextuelle, navigation intelligente.

### 2. Documentation automatique
Générer de la doc à partir du code en direct.

### 3. Impact analysis
"Si je modifie cette fonction, quels endpoints sont affectés ?"

### 4. Code search
"Où est utilisée la fonction `authenticate` ?"

### 5. Real-time monitoring
Surveiller les erreurs de build pendant le développement.

---

## 🚧 Limitations actuelles

- **Parsing limité** : détection de routes basique (amélioration possible)
- **Appels indirects** : call graph ne capture pas les closures/trait objects
- **Diagnostics** : collecte en cours d'amélioration
- **Performance** : full rescan au lieu d'incremental (TODO)

---

## 🔮 Roadmap

- [ ] Incremental indexing (ne rescanner que les fichiers modifiés)
- [ ] Meilleure détection des routes (macros Axum complexes)
- [ ] Support des workspaces multi-crates
- [ ] Cache sur disque pour grandes codebases
- [ ] Parsing des tests et benchmarks
- [ ] Métriques de complexité (cyclomatic, cognitive)

---

## 📚 Ressources

- [JSON-RPC 2.0 Specification](https://www.jsonrpc.org/specification)
- [syn crate](https://docs.rs/syn/)
- [notify crate](https://docs.rs/notify/)
