# Rustwork — Copilot / AI Agent Instructions

## 1. Nature du projet (IMPORTANT)

Rustwork n’est PAS une application classique.

Rustwork est :
- un framework backend Rust
- un outil CLI
- un générateur de projets
- un analyseur de code
- un serveur MCP dynamique connecté à cargo watch

Le projet est conçu IA-first :  
la lisibilité machine, la stabilité structurelle et la cartographie dynamique priment sur la concision.

## 2. Architecture générale

### Crates principales

- crates/rustwork  
  Framework runtime (Axum, configuration, base de données, middleware)

- crates/rustwork-cli  
  CLI, génération de code, gRPC, MCP, watchers

Le MCP, le gRPC et la CLI vivent exclusivement dans rustwork-cli.

## 3. Règle structurante interne (NON NÉGOCIABLE)

1 fichier = 1 responsabilité claire et testable.

- Pas de fichiers utilitaires génériques
- Pas de logique complexe dans mod.rs
- Les dossiers portent le sens
- Les fichiers portent l’action

## 4. MCP (Model Context Protocol)

Le MCP de Rustwork est dynamique, non figé, branché sur cargo watch et sert de source de vérité runtime.

### Tools MCP existants

- rustwork_get_conventions
- rustwork_get_routes
- rustwork_get_file_doc
- rustwork_get_function_usage
- rustwork_get_route_impact
- rustwork_get_diagnostics
- rustwork_get_env_setup

## 4.1. Règle MCP (Model Context Protocol)

Les tools MCP doivent être rangés comme suit :

```acsii
mcp/
├── commun/ #code commun à plusieurs tools
│   └── .../ #dossiers et fichiers communs
├── <tool name>/ #dossier du tool MCP
│   └── .../ #dossiers et fichiers du tool MCP
└── mod.rs
```

## 5. Règle MCP critique : diagnostics centralisés

SEUL rustwork_get_diagnostics expose :
- état cargo watch
- erreurs de build
- warnings
- futur rust-analyzer

Tous les autres tools doivent rediriger vers ce tool en cas d’instabilité.

## 6. get_file_doc

Fournit une vue structurelle fiable d’un fichier précis à un instant donné.  
Peut refuser de répondre si l’index est instable.

## 7. MCP dynamique + cargo watch

Le MCP observe en temps réel les changements, rebuilds et erreurs.  
Toute réponse doit inclure un niveau de confiance.

## 8. Watchers & Tokio

Tout watcher doit :
- s’exécuter dans un runtime Tokio valide
- ne jamais panic
- ne jamais utiliser unwrap/expect
- utiliser spawn_blocking pour notify

## 9. CLI — rustwork dev

- Peut être lancé depuis un dossier parent
- Détecte tous les services Rustwork enfants
- Lance les services en parallèle
- Préfixe les logs
- Lance un seul MCP par workspace

## 10. Templates

Templates strictement séparés :
- monolith
- micro
- micro_shared

Aucune logique conditionnelle dans les templates.

## 11. Environnement (.env)

Le tool rustwork_get_env_setup :
- scanne tous les .env.example et .env
- détecte variables manquantes
- détecte conflits de ports
- propose des ports libres
- ne modifie jamais les fichiers

## 12. gRPC

- gRPC uniquement en micro-services
- basé sur DSL .rwk
- interdit en monolithe

## 13. Tests

Priorité des tests :
1. MCP
2. CLI
3. Templates
4. Config
5. gRPC P0

## 14. Interdictions absolues

- Pas de heuristiques best-effort
- Pas de fallback silencieux
- Pas de modification comportementale implicite
- Pas de regroupement cosmétique

## 15. Objectif final

Rustwork vise à être :
- un framework backend Rust sérieux
- un environnement micro-services
- un MCP de référence compréhensible par l’IA

## 16. Règle de versioning

Suivre strictement le versioning sémantique (semver).
A chaque modification de code validée et testée, vérifier la version dans Cargo.toml pour incrémentér +1.
la structure de version est : MAJOR.MINOR.PATCH

## 17. Règle de fin d'implémentation

A la fin de chaque tâche d'implémentation, lancer les tests unitaires et d'intégration.
Installer le binaire localement avec `cargo install --path crates/rustwork-cli --force`.