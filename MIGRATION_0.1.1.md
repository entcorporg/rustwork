# Migration vers 0.1.2

## Changement: Suppression des références SeaORM migration

### Contexte

Rustwork utilise maintenant sqlx pour les migrations (fichiers SQL) au lieu de SeaORM.
Les projets générés avec des versions antérieures à 0.1.2 contenaient des références aux crates `*-migration` dans chaque `Cargo.toml` de service.

### Problème

Si vous avez un projet généré avant la version 0.1.2, vous pourriez voir cette erreur:

```
error: failed to load manifest for workspace member `/path/to/Backend/services/auth`
referenced by workspace at `/path/to/Backend/Cargo.toml`

Caused by:
  failed to load manifest for dependency `auth-migration`

Caused by:
  failed to read `/path/to/Backend/services/auth/migration/Cargo.toml`

Caused by:
  No such file or directory (os error 2)
```

### Solution

Éditez chaque fichier `Cargo.toml` de vos services et supprimez la ligne faisant référence à `*-migration`:

**Avant** (`services/auth/Cargo.toml`):
```toml
[package]
name = "auth"
version = "0.1.0"
edition = "2021"

[dependencies]
rustwork = { git = "https://github.com/entcorporg/rustwork.git", branch = "main" }
shared = { path = "../shared" }
auth-migration = { path = "migration" }  # ← À supprimer
axum = "0.7"
...
```

**Après** (`services/auth/Cargo.toml`):
```toml
[package]
name = "auth"
version = "0.1.0"
edition = "2021"

[dependencies]
rustwork = { git = "https://github.com/entcorporg/rustwork.git", branch = "main" }
shared = { path = "../shared" }
axum = "0.7"
...
```

Répétez pour tous vos services (auth, user, test_service, etc.).

### Script de correction automatique

Vous pouvez utiliser cette commande pour corriger automatiquement tous vos services:

```bash
cd Backend/services
for service in */; do
    sed -i '/-migration = { path = "migration" }/d' "$service/Cargo.toml"
done
```

### Vérification

Après modification, exécutez:

```bash
cargo check
```

Puis testez le lancement des services:

```bash
rustwork dev
```

### Migrations SQL

Les migrations sont maintenant gérées via des fichiers `.sql` dans le dossier `migrations/` de chaque service:

```
services/
└── auth/
    ├── migrations/
    │   ├── 20240101_000001_initial.up.sql
    │   └── 20240101_000001_initial.down.sql
    ├── src/
    └── Cargo.toml
```

Consultez la documentation sur sqlx pour plus d'informations sur la gestion des migrations.
