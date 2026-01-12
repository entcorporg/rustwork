# rustwork_get_env_setup - Environment Setup Verification Tool

## Description

Le tool MCP `rustwork_get_env_setup` analyse la configuration des variables d'environnement dans un projet Rustwork (monolithe ou micro-services) et détecte les conflits de configuration AVANT l'exécution.

## Objectif

Ce tool aide l'IA et l'utilisateur à :

1. **Vérifier** que les fichiers `.env` sont correctement initialisés à partir des `.env.example`
2. **Détecter** les variables d'environnement manquantes
3. **Identifier** les conflits de ports entre services
4. **Proposer** des actions correctives claires

## Caractéristiques

### Sécurité

- ✅ **Aucune écriture** de fichier
- ✅ **Aucune modification** automatique
- ✅ **Aucune exposition** de valeurs sensibles
- ✅ Seuls les ports numériques sont exposés

### Périmètre d'analyse

Le tool scanne :

- **Monolithe** : `.env` et `.env.example` à la racine du projet
- **Micro-services** : `.env` et `.env.example` dans chaque service sous `services/`

### Détection de ports

Le tool identifie automatiquement les variables de port :

- Variables contenant `PORT`, `_PORT`, `SERVICE_PORT`, `APP_PORT`
- Extraction des valeurs numériques uniquement
- Détection des conflits (même port utilisé par plusieurs services)
- Suggestion de ports alternatifs non utilisés

## Format de réponse

```json
{
  "status": "ok" | "action_required" | "conflict_detected",
  "confidence": "high",
  "architecture": "monolith" | "microservices",
  "services": [
    {
      "name": "auth",
      "path": "/path/to/service",
      "env_example": "present" | "absent",
      "env": "present" | "absent",
      "required": ["DB_URL", "JWT_SECRET"],
      "optional": ["LOG_LEVEL"],
      "missing": ["JWT_SECRET"],
      "overridden": [],
      "ports": [3000]
    }
  ],
  "port_conflicts": [
    {
      "port": 3000,
      "services": ["auth", "user"],
      "suggested_port": 3002
    }
  ],
  "recommendations": [
    {
      "action": "copy_env_example",
      "severity": "error",
      "message": "Copy .env.example to .env for: user",
      "services": ["user"]
    },
    {
      "action": "resolve_port_conflicts",
      "severity": "error",
      "message": "Assign unique ports to each service to avoid runtime collision",
      "conflicts": [...]
    }
  ]
}
```

## Cas d'usage

### Cas 1 : .env manquant

**Situation** : `.env.example` existe, `.env` absent

**Réponse** :
```json
{
  "status": "action_required",
  "recommendations": [
    {
      "action": "copy_env_example",
      "severity": "error",
      "message": "Copy .env.example to .env for: auth"
    }
  ]
}
```

### Cas 2 : Variables manquantes

**Situation** : `.env` existe mais certaines variables sont manquantes

**Réponse** :
```json
{
  "status": "action_required",
  "services": [{
    "name": "auth",
    "missing": ["JWT_SECRET", "DB_URL"]
  }],
  "recommendations": [
    {
      "action": "set_missing_variables",
      "severity": "error",
      "message": "Set missing variables in auth: JWT_SECRET, DB_URL"
    }
  ]
}
```

### Cas 3 : Conflit de ports

**Situation** : Plusieurs services utilisent le même port

**Réponse** :
```json
{
  "status": "conflict_detected",
  "port_conflicts": [
    {
      "port": 3000,
      "services": ["auth", "user"],
      "suggested_port": 3002
    }
  ],
  "recommendations": [
    {
      "action": "resolve_port_conflicts",
      "severity": "error",
      "message": "Assign unique ports to each service to avoid runtime collision"
    }
  ]
}
```

### Cas 4 : Configuration correcte

**Situation** : Tout est bien configuré

**Réponse** :
```json
{
  "status": "ok",
  "recommendations": [
    {
      "action": "none",
      "severity": "info",
      "message": "Environment configuration is correct. No action required."
    }
  ]
}
```

## Intégration avec d'autres tools

Le tool `rustwork_get_diagnostics` suggère automatiquement d'appeler `rustwork_get_env_setup` lorsqu'il détecte des erreurs liées à :

- Ports (`address already in use`)
- Variables d'environnement
- Configuration runtime

**Exemple de suggestion** :
```json
{
  "suggestions": [
    "Request rustwork_get_env_setup to verify environment variables and detect port conflicts before retrying"
  ]
}
```

## Utilisation via MCP

### Protocole MCP Standard

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "rustwork_get_env_setup",
    "arguments": {}
  }
}
```

### Appel direct (VS Code)

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "rustwork_get_env_setup",
  "params": {}
}
```

## Workflow recommandé

1. **Avant le premier lancement** :
   ```
   Utilisateur : "Lance le serveur"
   IA : Appelle rustwork_get_env_setup
   → Détecte .env manquant
   → Guide l'utilisateur pour copier .env.example
   ```

2. **En cas d'erreur runtime** :
   ```
   Utilisateur : "J'ai une erreur 'address already in use'"
   IA : Appelle rustwork_get_env_setup
   → Détecte conflit de port 3000
   → Propose port 3002
   ```

3. **Configuration micro-services** :
   ```
   Utilisateur : "Configure mon workspace micro-services"
   IA : Appelle rustwork_get_env_setup
   → Analyse tous les services
   → Liste les .env manquants
   → Détecte les conflits de ports
   → Fournit un plan d'action
   ```

## Limitations

- ✅ Analyse uniquement les fichiers `.env` et `.env.example`
- ✅ Ne détecte pas les variables définies dans le système (sauf via heuristique)
- ✅ Ne valide pas la syntaxe des valeurs (seulement la présence)
- ✅ Les variables "optionnelles" sont détectées par heuristique (présence de valeur par défaut)

## Sécurité et confidentialité

- **Aucune valeur sensible** n'est exposée (mots de passe, secrets, tokens)
- **Seuls les ports** (valeurs numériques) sont extraits et exposés
- **Aucune modification** n'est effectuée sur les fichiers
- **Lecture seule** : le tool est un outil d'analyse et de recommandation

## Exemples de flux complet

### Scénario : Nouveau développeur

```bash
# Développeur clone le projet
git clone <repo>
cd backend

# IA détecte automatiquement via rustwork_get_env_setup
→ Status: action_required
→ Services sans .env: [auth, user, api]
→ Recommendation: "Copy .env.example to .env for: auth, user, api"

# IA guide l'utilisateur
"Pour démarrer, vous devez créer les fichiers .env :
  cp services/auth/.env.example services/auth/.env
  cp services/user/.env.example services/user/.env
  cp services/api/.env.example services/api/.env

Ensuite, configurez les variables suivantes :
  - auth: JWT_SECRET, DB_URL
  - user: DB_URL
  - api: API_KEY"
```

### Scénario : Conflit de ports détecté

```bash
# Utilisateur lance les services
cargo run

# Erreur : "address already in use"

# IA appelle rustwork_get_env_setup
→ Status: conflict_detected
→ Port 3000 utilisé par: [auth, user]
→ Suggestion: port 3002 disponible

# IA propose la solution
"Conflit détecté : les services 'auth' et 'user' utilisent tous deux le port 3000.

Solution proposée :
  - auth: garder le port 3000
  - user: changer vers le port 3002

Modifier services/user/.env :
  APP_PORT=3002"
```

## Implémentation

Le tool est implémenté dans :
- **Module** : `crates/rustwork-cli/src/mcp/tools/rustwork/get_env_setup.rs`
- **Intégration MCP** : Automatique via `tools/list` et `tools/call`
- **Routage** : `dispatcher/routes.rs`

## Tests

Pour tester le tool :

```bash
# Créer un environnement de test
mkdir -p test-project/services/{auth,user}

# Créer des .env.example avec conflits
echo "APP_PORT=3000" > test-project/services/auth/.env.example
echo "APP_PORT=3000" > test-project/services/user/.env.example

# Lancer le serveur MCP et appeler le tool
# Le tool devrait détecter le conflit de port
```

## Conclusion

Le tool `rustwork_get_env_setup` est un outil de **prévention** et d'**orientation** qui aide à éviter les erreurs de configuration courantes. Il ne remplace pas la configuration manuelle mais la guide intelligemment.
