# Guide de Test: rustwork dev - Mode Workspace

## Test Rapide

### Étape 1: Créer un workspace de test

```bash
# Créer la structure
mkdir -p /tmp/test-workspace/backend/services
cd /tmp/test-workspace/backend/services

# Créer 3 services
/home/linux/rustwork/target/release/rustwork new auth
/home/linux/rustwork/target/release/rustwork new user
/home/linux/rustwork/target/release/rustwork new blog
```

### Étape 2: Configurer les ports

```bash
# auth: port 3001
cat > auth/config/default.toml <<'EOF'
[server]
host = "0.0.0.0"
port = 3001
workers = 4

[database]
url = "sqlite://data/auth.db"
max_connections = 10

[log]
level = "info"
format = "json"
EOF

# user: port 3002
cat > user/config/default.toml <<'EOF'
[server]
host = "0.0.0.0"
port = 3002
workers = 4

[database]
url = "sqlite://data/user.db"
max_connections = 10

[log]
level = "info"
format = "json"
EOF

# blog: port 3003
cat > blog/config/default.toml <<'EOF'
[server]
host = "0.0.0.0"
port = 3003
workers = 4

[database]
url = "sqlite://data/blog.db"
max_connections = 10

[log]
level = "info"
format = "json"
EOF
```

### Étape 3: Tester la détection

```bash
# Test 1: Depuis services/
cd /tmp/test-workspace/backend/services
/home/linux/rustwork/target/release/rustwork dev

# Résultat attendu:
# 🔧 Starting Rustwork development workspace...
# 🔍 Detected 3 Rustwork service(s):
#   - auth (auth)
#   - user (user)
#   - blog (blog)
# 
# ℹ️  MCP server disabled. Use --mcp to enable it.
# 
# ▶ Starting auth...
# ▶ Starting user...
# ▶ Starting blog...
# 
# ✅ All services started. Press Ctrl+C to stop all services.
```

```bash
# Test 2: Depuis backend/ (un niveau au-dessus)
cd /tmp/test-workspace/backend
/home/linux/rustwork/target/release/rustwork dev

# Résultat attendu: même comportement, mais avec chemins relatifs
# 🔧 Starting Rustwork development workspace...
# 🔍 Detected 3 Rustwork service(s):
#   - auth (services/auth)
#   - user (services/user)
#   - blog (services/blog)
```

```bash
# Test 3: Depuis test-workspace/ (deux niveaux au-dessus)
cd /tmp/test-workspace
/home/linux/rustwork/target/release/rustwork dev

# Résultat attendu: même comportement
# Les chemins seront relatifs: backend/services/auth, etc.
```

```bash
# Test 4: Depuis un service individuel
cd /tmp/test-workspace/backend/services/auth
/home/linux/rustwork/target/release/rustwork dev

# Résultat attendu: mode single-service (compatibilité)
# 🔧 Starting development server with hot-reload...
#    Watching for changes in src/
# ℹ️  MCP server disabled. Use --mcp to enable it.
```

```bash
# Test 5: Avec MCP activé
cd /tmp/test-workspace/backend
/home/linux/rustwork/target/release/rustwork dev --mcp

# Résultat attendu:
# 🔧 Starting Rustwork development workspace...
# 🔍 Detected 3 Rustwork service(s):
#   - auth (services/auth)
#   - user (services/user)
#   - blog (services/blog)
# 
# 🚀 Starting MCP server on 127.0.0.1:4000... (development only)
#    MCP observing workspace: /tmp/test-workspace/backend
#    Press Ctrl+C to stop
```

## Vérification des Logs

Une fois les services démarrés, vous devriez voir des logs préfixés :

```
[auth] Compiling auth v0.1.0 (/tmp/test-workspace/backend/services/auth)
[user] Compiling user v0.1.0 (/tmp/test-workspace/backend/services/user)
[blog] Compiling blog v0.1.0 (/tmp/test-workspace/backend/services/blog)
[auth]     Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.23s
[user]     Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.45s
[blog]     Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.67s
[auth]      Running `target/debug/auth`
[user]      Running `target/debug/user`
[blog]      Running `target/debug/blog`
[auth] INFO: Starting Rustwork server...
[auth] INFO: Server listening on 0.0.0.0:3001
[user] INFO: Starting Rustwork server...
[user] INFO: Server listening on 0.0.0.0:3002
[blog] INFO: Starting Rustwork server...
[blog] INFO: Server listening on 0.0.0.0:3003
```

## Test de Modification (Hot-reload)

Dans un autre terminal:

```bash
# Modifier un fichier du service auth
echo '// test change' >> /tmp/test-workspace/backend/services/auth/src/main.rs

# Vous devriez voir dans les logs:
# [auth] Restarting due to changes...
# [auth] Compiling auth v0.1.0...
# [auth] Finished in 2.34s
# [auth] Running `target/debug/auth`
# [auth] INFO: Server listening on 0.0.0.0:3001

# Les autres services continuent de tourner normalement:
# [user] INFO: Handling request...
# [blog] INFO: Handling request...
```

## Test des Services

```bash
# Terminal 1: lancer les services
cd /tmp/test-workspace/backend
/home/linux/rustwork/target/release/rustwork dev

# Terminal 2: tester les endpoints
curl http://localhost:3001/health  # auth
curl http://localhost:3002/health  # user
curl http://localhost:3003/health  # blog
```

## Nettoyage

```bash
# Arrêter les services (Ctrl+C dans le terminal de rustwork dev)
# Supprimer le workspace de test
rm -rf /tmp/test-workspace
```

## Validation Finale

✅ Détection depuis différents niveaux de dossiers  
✅ Lancement parallèle de tous les services  
✅ Logs préfixés par service  
✅ Hot-reload indépendant par service  
✅ Mode single-service préservé  
✅ MCP centralisé (si --mcp)  
✅ Messages d'erreur clairs  
✅ Pas d'erreurs de compilation  

## Troubleshooting

### "cargo-watch not found"
```bash
cargo install cargo-watch
```

### Ports déjà utilisés
```bash
# Vérifier les ports
lsof -i :3001
lsof -i :3002
lsof -i :3003

# Tuer les processus si nécessaire
kill -9 <PID>
```

### Services ne démarrent pas
```bash
# Vérifier la structure de chaque service
ls -la /tmp/test-workspace/backend/services/auth/.rustwork/
ls -la /tmp/test-workspace/backend/services/auth/src/
```

### Logs trop verbeux
```bash
# Lancer un seul service
cd /tmp/test-workspace/backend/services/auth
/home/linux/rustwork/target/release/rustwork dev
```
