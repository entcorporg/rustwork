# Documentation: rustwork dev - Mode Workspace

## 📚 Guide de Navigation

### 🎯 Pour Commencer
- **[DEV_WORKSPACE_SUMMARY.md](../DEV_WORKSPACE_SUMMARY.md)** - Vue d'ensemble et résumé exécutif
- **[DEV_WORKSPACE.md](DEV_WORKSPACE.md)** - Guide utilisateur complet

### 🔧 Pour les Développeurs
- **[DEV_WORKSPACE_CHANGES.md](DEV_WORKSPACE_CHANGES.md)** - Détails techniques de l'implémentation
- **[DEV_WORKSPACE_TEST.md](DEV_WORKSPACE_TEST.md)** - Guide de test manuel

### ❓ Questions Fréquentes
- **[DEV_WORKSPACE_FAQ.md](DEV_WORKSPACE_FAQ.md)** - FAQ complète

### 🧪 Tests
- **[test_dev_workspace.sh](../test_dev_workspace.sh)** - Script de test automatisé

---

## 📖 Par Scénario d'Utilisation

### Je veux comprendre rapidement
→ Lisez [DEV_WORKSPACE_SUMMARY.md](../DEV_WORKSPACE_SUMMARY.md)

### Je veux utiliser la feature
→ Consultez [DEV_WORKSPACE.md](DEV_WORKSPACE.md)

### Je veux savoir comment ça marche
→ Parcourez [DEV_WORKSPACE_CHANGES.md](DEV_WORKSPACE_CHANGES.md)

### Je veux tester
→ Lancez [test_dev_workspace.sh](../test_dev_workspace.sh)  
→ Ou suivez [DEV_WORKSPACE_TEST.md](DEV_WORKSPACE_TEST.md)

### J'ai une question
→ Consultez [DEV_WORKSPACE_FAQ.md](DEV_WORKSPACE_FAQ.md)

### Je rencontre un problème
→ Section Troubleshooting dans [DEV_WORKSPACE.md](DEV_WORKSPACE.md#troubleshooting)  
→ Ou [DEV_WORKSPACE_FAQ.md](DEV_WORKSPACE_FAQ.md#d%C3%A9pannage-rapide)

---

## 🗺️ Architecture de la Documentation

```
rustwork/
├── DEV_WORKSPACE_SUMMARY.md       # 📋 Résumé exécutif
├── test_dev_workspace.sh          # 🧪 Script de test
└── docs/
    ├── DEV_WORKSPACE.md           # 📘 Guide utilisateur
    ├── DEV_WORKSPACE_CHANGES.md   # 🔧 Détails techniques
    ├── DEV_WORKSPACE_TEST.md      # 🧪 Guide de test manuel
    ├── DEV_WORKSPACE_FAQ.md       # ❓ Questions fréquentes
    └── DEV_WORKSPACE_INDEX.md     # 📚 Ce fichier
```

---

## 🎯 Checklist Rapide

Avant d'utiliser `rustwork dev` en mode workspace :

- [ ] J'ai lu le [résumé](../DEV_WORKSPACE_SUMMARY.md)
- [ ] J'ai compris les [critères de détection](DEV_WORKSPACE.md#détection-dun-service-rustwork)
- [ ] J'ai configuré des [ports différents](DEV_WORKSPACE.md#workflow-recommandé) pour mes services
- [ ] J'ai installé `cargo-watch` (`cargo install cargo-watch`)
- [ ] J'ai consulté les [exemples](DEV_WORKSPACE.md#exemples-darchitectures)

---

## 🚀 Démarrage Rapide

```bash
# 1. Créer un workspace de test
mkdir -p backend/services && cd backend/services

# 2. Créer des services
rustwork new auth
rustwork new user

# 3. Configurer les ports (3001, 3002)
# Éditer config/default.toml de chaque service

# 4. Lancer tout
cd .. && rustwork dev

# 5. Avec MCP
rustwork dev --mcp
```

---

## 📞 Support

- **Issues :** [GitHub Issues](https://github.com/entcorporg/rustwork/issues)
- **Documentation principale :** [README.md](../README.md)
- **Changelog :** [CHANGELOG.md](../CHANGELOG.md)

---

**Dernière mise à jour :** 11 janvier 2026
