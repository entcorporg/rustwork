# Support gRPC natif Rustwork

Rustwork intègre maintenant le support gRPC complet via un DSL simple (`.rwk`), permettant de créer des micro-services sans écrire de fichiers `.proto` ou `build.rs` manuellement.

## 🚀 Démarrage rapide

### 1. Créer un fichier DSL `.rwk`

Créez `grpc/user.rwk` :

```rwk
service UserService

rpc GetUser (GetUserRequest) returns (User)
rpc CreateUser (CreateUserRequest) returns (User)
rpc ListUsers (ListUsersRequest) returns (UserList)

message GetUserRequest {
  id: uuid
}

message CreateUserRequest {
  email: string
  password: string
  name: string
}

message ListUsersRequest {
  limit: int
  offset: int
}

message User {
  id: uuid
  email: string
  name: string
  created_at: datetime
}

message UserList {
  users: list<User>
  total: int
}
```

### 2. Générer le code gRPC

```bash
rustwork grpc build
```

Cette commande :
- ✅ Parse les fichiers `.rwk`
- ✅ Génère les fichiers `.proto` (dans `target/rustwork/grpc/`)
- ✅ Crée/met à jour `build.rs`
- ✅ Ajoute les dépendances nécessaires au `Cargo.toml`
- ✅ Génère le code Rust (traits, serveurs, clients)

### 3. Compiler le projet

```bash
cargo build
```

### 4. Implémenter le handler

Dans `src/handlers/user_handler.rs` :

```rust
use async_trait::async_trait;
use tonic::Status;
use crate::grpc::{
    UserServiceHandler, User, GetUserRequest, 
    CreateUserRequest, ListUsersRequest, UserList
};

pub struct MyUserHandler {
    // Votre état (DB, etc.)
}

#[async_trait]
impl UserServiceHandler for MyUserHandler {
    async fn get_user(&self, request: GetUserRequest) -> Result<User, Status> {
        // Votre logique ici
        Ok(User {
            id: request.id,
            email: "user@example.com".to_string(),
            name: "John Doe".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
        })
    }

    async fn create_user(&self, request: CreateUserRequest) -> Result<User, Status> {
        // Créer l'utilisateur...
        Ok(User {
            id: uuid::Uuid::new_v4().to_string(),
            email: request.email,
            name: request.name,
            created_at: chrono::Utc::now().to_rfc3339(),
        })
    }

    async fn list_users(&self, request: ListUsersRequest) -> Result<UserList, Status> {
        // Liste les utilisateurs...
        Ok(UserList {
            users: vec![],
            total: 0,
        })
    }
}
```

### 5. Démarrer le serveur gRPC

Dans `src/main.rs` :

```rust
use tonic::transport::Server;
use crate::grpc::grpc_service;
use crate::handlers::user_handler::MyUserHandler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let handler = MyUserHandler::new();
    
    println!("🚀 Serveur gRPC démarré sur {}", addr);
    
    Server::builder()
        .add_service(grpc_service(handler))
        .serve(addr)
        .await?;
    
    Ok(())
}
```

### 6. Utiliser le client

```rust
use crate::grpc::{user_service_client, GetUserRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = user_service_client("http://[::1]:50051").await?;
    
    let request = GetUserRequest {
        id: "123e4567-e89b-12d3-a456-426614174000".to_string(),
    };
    
    let response = client.get_user(request).await?;
    let user = response.into_inner();
    
    println!("User: {} - {}", user.name, user.email);
    
    Ok(())
}
```

## 📖 Syntaxe DSL `.rwk`

### Types supportés (v0)

| Type DSL | Type Rust | Type Proto | Description |
|----------|-----------|------------|-------------|
| `string` | `String` | `string` | Chaîne de caractères |
| `int` | `i32` | `int32` | Entier 32 bits |
| `bool` | `bool` | `bool` | Booléen |
| `uuid` | `String` | `string` | UUID (format string) |
| `datetime` | `String` | `string` | Date/heure RFC3339 |
| `optional<T>` | `Option<T>` | `optional T` | Valeur optionnelle |
| `list<T>` | `Vec<T>` | `repeated T` | Liste |

### Règles

- ✅ 1 fichier = 1 service
- ✅ Pas de package/import/options (géré automatiquement)
- ✅ Noms en PascalCase pour services et messages
- ✅ Noms en snake_case pour les champs (conversion automatique)

### Exemple avec types avancés

```rwk
service ProductService

rpc GetProduct (ProductRequest) returns (Product)
rpc SearchProducts (SearchRequest) returns (ProductList)

message ProductRequest {
  id: uuid
}

message SearchRequest {
  query: string
  category: optional<string>
  tags: list<string>
  max_price: optional<int>
}

message Product {
  id: uuid
  name: string
  description: optional<string>
  price: int
  tags: list<string>
  available: bool
  created_at: datetime
}

message ProductList {
  products: list<Product>
  total: int
  has_more: bool
}
```

## 🏗️ Architecture Monorepo/Micro-services

Pour un projet avec plusieurs micro-services :

```
my-project/
├── services/
│   ├── user/
│   │   ├── grpc/
│   │   │   └── user.rwk
│   │   ├── src/
│   │   │   └── main.rs
│   │   └── Cargo.toml
│   │
│   ├── auth/
│   │   ├── grpc/
│   │   │   └── auth.rwk
│   │   └── ...
│   │
│   └── product/
│       ├── grpc/
│       │   └── product.rwk
│       └── ...
│
└── Cargo.toml (workspace)
```

Rustwork détecte automatiquement les services et génère :
- ✅ Les serveurs pour chaque service
- ✅ Les clients pour les appels inter-services

## 🔧 Commandes CLI

### `rustwork grpc build`

Génère tous les fichiers nécessaires à partir des `.rwk`.

```bash
# Dans le projet courant
rustwork grpc build

# Dans un projet spécifique
rustwork grpc build --project ./my-service
```

### Workflow typique

```bash
# 1. Créer/modifier un fichier .rwk
vim grpc/user.rwk

# 2. Générer le code
rustwork grpc build

# 3. Compiler
cargo build

# 4. Implémenter les handlers
# 5. Démarrer le serveur
cargo run
```

## 📂 Structure générée

```
project/
├── grpc/
│   └── user.rwk              # Votre DSL
│
├── target/rustwork/grpc/
│   └── user_service.proto    # Proto généré (interne)
│
├── src/
│   └── grpc/
│       ├── mod.rs            # Module principal
│       ├── user_service.rs   # Traits et helpers
│       └── generated/        # Code tonic (gitignore)
│           └── ...
│
├── build.rs                  # Généré automatiquement
└── Cargo.toml                # Dépendances ajoutées auto
```

## 🔐 Sécurité & Robustesse

- ✅ Aucun `panic!` ou `unwrap()`
- ✅ Messages d'erreur clairs avec ligne/colonne
- ✅ Validation stricte du DSL
- ✅ Code généré avec `#![allow(dead_code)]`
- ✅ Logs verbeux sur stderr

## 🎯 Limitations actuelles (v0)

- ⚠️ Pas de streaming (unary RPCs seulement)
- ⚠️ Pas d'options proto avancées
- ⚠️ Types simples uniquement (pas d'enums, maps, oneof)
- ⚠️ Pas de validation métier dans le DSL
- ⚠️ Pas de génération de documentation OpenAPI

Ces limitations seront levées dans les versions futures.

## 🆘 Dépannage

### Erreur "tonic not found"

```bash
cargo add tonic prost tokio --features tokio/full
cargo add tonic-build --build
```

### Erreur de compilation proto

Vérifiez que `rustwork grpc build` a réussi sans erreurs.
Supprimez `target/` et recommencez :

```bash
rm -rf target/
rustwork grpc build
cargo build
```

### Code généré non trouvé

Assurez-vous que `cargo build` a été exécuté après `rustwork grpc build`.
Le code proto est généré par `build.rs` lors de `cargo build`.

## 📚 Ressources

- [Documentation tonic](https://github.com/hyperium/tonic)
- [gRPC best practices](https://grpc.io/docs/guides/performance/)
- [Protocol Buffers](https://protobuf.dev/)

## 🔮 Roadmap

- [ ] Support streaming (bidirectionnel, client, server)
- [ ] Enums et types complexes
- [ ] Génération de tests unitaires
- [ ] Validation automatique (required, range, regex)
- [ ] Génération de documentation
- [ ] Support TLS/mTLS
- [ ] Middleware gRPC intégré
- [ ] Observabilité (traces, metrics)
- [ ] Hot-reload des services
