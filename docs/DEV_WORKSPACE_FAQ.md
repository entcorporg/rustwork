# FAQ: rustwork dev - Mode Workspace

## Questions Fréquentes

### Q1: Comment ça marche exactement ?

`rustwork dev` remonte les dossiers parents à partir de votre position actuelle, scanne récursivement tous les sous-dossiers, et détecte automatiquement tous les services Rustwork valides (ceux qui ont `.rustwork/manifest.json`, `Cargo.toml`, et `src/main.rs`).

### Q2: Puis-je lancer `rustwork dev` depuis n'importe où ?

Oui ! C'est justement l'objectif. Vous pouvez lancer la commande depuis :
- La racine de votre projet
- Un dossier parent quelconque
- Le dossier `services/`
- Un service individuel (mode single-service)

### Q3: Que se passe-t-il si j'ai plusieurs services ?

Tous les services détectés sont lancés **en parallèle**, chacun avec son propre `cargo-watch`. Les logs sont automatiquement préfixés avec `[service-name]` pour faciliter la lecture.

### Q4: Comment différencier les logs de chaque service ?

Chaque ligne de log est préfixée avec le nom du service :
```
[auth] Compiling auth v0.1.0...
[user] Server listening on 0.0.0.0:3002
[blog] ERROR: Database connection failed
```

### Q5: Les services doivent-ils avoir des ports différents ?

**Oui, absolument !** Chaque service doit écouter sur un port unique. Configurez cela dans `config/default.toml` de chaque service :

```toml
# services/auth/config/default.toml
[server]
port = 3001

# services/user/config/default.toml
[server]
port = 3002
```

### Q6: Que se passe-t-il si un service échoue au démarrage ?

Les autres services continuent de tourner normalement. Vous verrez un message d'avertissement :
```
⚠️  Failed to start user: Failed to start cargo watch
   Continuing with other services...
```

### Q7: Comment le MCP fonctionne-t-il avec plusieurs services ?

Le serveur MCP est lancé **une seule fois** à la racine du workspace détecté et observe **tous** les services simultanément. Il n'y a pas un MCP par service.

```bash
rustwork dev --mcp
# → MCP observing workspace: /path/to/backend
```

### Q8: Puis-je lancer un seul service spécifique ?

Oui ! Placez-vous dans le dossier du service et lancez `rustwork dev` :

```bash
cd services/auth
rustwork dev
```

Le mode single-service sera automatiquement activé.

### Q9: Comment arrêter tous les services ?

Un simple `Ctrl+C` arrête proprement tous les services lancés.

### Q10: Que se passe-t-il si je n'ai pas `cargo-watch` ?

La commande échouera avec un message clair :
```
⚠️  cargo-watch not found.
   Run: cargo install cargo-watch
```

Installez-le avec : `cargo install cargo-watch`

### Q11: Est-ce compatible avec mon projet existant ?

**Oui, totalement !** Si vous avez un projet monolithe classique, le comportement est exactement le même qu'avant. La détection multi-services ne s'active que si plusieurs services sont trouvés.

### Q12: Comment définir l'ordre de démarrage des services ?

Actuellement, tous les services démarrent en parallèle. Il n'y a pas d'ordre de priorité. Si vous avez besoin d'un ordre spécifique, lancez les services individuellement.

### Q13: Puis-je désactiver le préfixage des logs ?

Non, le préfixage est automatique en mode multi-services pour éviter la confusion. En mode single-service, il n'y a pas de préfixe.

### Q14: Comment savoir quels services ont été détectés ?

Au démarrage, la commande affiche clairement :
```
🔍 Detected 3 Rustwork service(s):
  - auth (services/auth)
  - user (services/user)
  - blog (services/blog)
```

### Q15: Que signifie "workspace root" ?

C'est le dossier ancêtre commun le plus haut contenant tous les services détectés. C'est utilisé pour le MCP pour qu'il puisse observer l'ensemble du workspace.

### Q16: Puis-je mixer services Rustwork et autres projets ?

La détection ignore les dossiers qui ne sont pas des services Rustwork valides. Vous pouvez avoir d'autres projets dans le même workspace sans problème.

### Q17: Comment déboguer un service spécifique ?

Lancez ce service individuellement :
```bash
cd services/problematic-service
rustwork dev
```

Ou utilisez les logs préfixés pour filtrer :
```bash
rustwork dev | grep '\[problematic-service\]'
```

### Q18: Les changements sont-ils détectés automatiquement ?

Oui ! Chaque service utilise `cargo-watch` qui redémarre automatiquement lors de modifications dans `src/` ou `config/`.

### Q19: Puis-je utiliser ça en production ?

**Non !** `rustwork dev` est **uniquement pour le développement**. En production, utilisez `cargo build --release` et lancez les binaires compilés.

### Q20: Quelle est la performance avec beaucoup de services ?

Chaque service tourne dans son propre processus `cargo-watch`. Pour de nombreux services (10+), considérez :
- Lancer seulement les services sur lesquels vous travaillez
- Augmenter la RAM disponible
- Utiliser un mode de lancement sélectif

### Q21: Comment contribuer ou améliorer cette fonctionnalité ?

Consultez [CONTRIBUTING.md](CONTRIBUTING.md) et proposez une PR. Les améliorations futures pourraient inclure :
- Configuration `.rustwork/workspace.toml`
- Gestion des dépendances entre services
- Interface TUI pour contrôler individuellement
- Logs colorés par service

### Q22: Où trouver plus d'informations ?

- Guide complet : [docs/DEV_WORKSPACE.md](docs/DEV_WORKSPACE.md)
- Détails techniques : [docs/DEV_WORKSPACE_CHANGES.md](docs/DEV_WORKSPACE_CHANGES.md)
- Guide de test : [docs/DEV_WORKSPACE_TEST.md](docs/DEV_WORKSPACE_TEST.md)
- Résumé : [DEV_WORKSPACE_SUMMARY.md](DEV_WORKSPACE_SUMMARY.md)

### Q23: Y a-t-il des exemples de configuration ?

Oui ! Lancez le script de test pour voir un exemple complet :
```bash
./test_dev_workspace.sh
```

Ou consultez [docs/DEV_WORKSPACE_TEST.md](docs/DEV_WORKSPACE_TEST.md) pour des exemples détaillés.

---

## Aide Rapide

```bash
# Lancer tous les services détectés
rustwork dev

# Avec MCP activé
rustwork dev --mcp

# Un seul service
cd services/auth && rustwork dev

# Tester la détection sans lancer
cargo run --bin rustwork dev --help
```

## Dépannage Rapide

| Problème | Solution |
|----------|----------|
| "No services found" | Vérifiez `.rustwork/manifest.json`, `Cargo.toml`, `src/main.rs` |
| "cargo-watch not found" | `cargo install cargo-watch` |
| Ports en conflit | Configurez des ports différents dans `config/default.toml` |
| Logs trop verbeux | Lancez un service individuel ou filtrez avec `grep` |
| Service ne démarre pas | Vérifiez les logs préfixés avec `[service-name]` |

---

**Besoin d'aide ?** Consultez la [documentation complète](docs/DEV_WORKSPACE.md) ou ouvrez une issue sur GitHub.
