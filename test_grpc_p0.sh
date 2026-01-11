#!/bin/bash
# Test automatique du support gRPC P0

set -e

echo "============================================"
echo "Test gRPC Rustwork - Niveau P0"
echo "============================================"
echo ""

# Couleurs
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color

RUSTWORK_BIN="/home/linux/rustwork/target/release/rustwork"

# Vérifier que rustwork est compilé
if [ ! -f "$RUSTWORK_BIN" ]; then
    echo "❌ rustwork CLI non trouvé. Compilez d'abord :"
    echo "   cd /home/linux/rustwork && cargo build --release --bin rustwork"
    exit 1
fi

echo "✅ rustwork CLI trouvé"
echo ""

# Test 1: Garde-fou monolithe
echo "📋 Test 1: Garde-fou monolithe"
echo "------------------------------"

cd /tmp
rm -rf test-grpc-mono
$RUSTWORK_BIN new test-grpc-mono --layout monolith > /dev/null 2>&1
cd test-grpc-mono

if $RUSTWORK_BIN grpc build 2>&1 | grep -q "only supported in micro-service layout"; then
    echo -e "${GREEN}✅ PASS${NC}: Erreur claire si monolithe détecté"
else
    echo -e "${RED}❌ FAIL${NC}: Le garde-fou monolithe ne fonctionne pas"
    exit 1
fi
echo ""

# Test 2: Génération micro-services
echo "📋 Test 2: Génération micro-services"
echo "------------------------------------"

cd /tmp
rm -rf test-grpc-micro
$RUSTWORK_BIN new test-grpc-micro --layout micro --services user > /dev/null 2>&1
cd test-grpc-micro

# Créer un fichier .rwk minimal
mkdir -p services/user/grpc
cat > services/user/grpc/user.rwk << 'EOF'
service User

rpc GetUser(GetUserRequest) returns (UserResponse)

message GetUserRequest {
    id: int
}

message UserResponse {
    id: int
    name: string
}
EOF

if $RUSTWORK_BIN grpc build 2>&1 | grep -q "Construction gRPC terminée avec succès"; then
    echo -e "${GREEN}✅ PASS${NC}: Génération gRPC réussie"
else
    echo -e "${RED}❌ FAIL${NC}: La génération gRPC a échoué"
    exit 1
fi

# Vérifier que les fichiers sont générés
if [ -f "services/user/build.rs" ] && \
   [ -f "services/user/target/rustwork/grpc/user_service.proto" ] && \
   [ -f "services/user/src/grpc/user.rs" ]; then
    echo -e "${GREEN}✅ PASS${NC}: Fichiers générés au bon emplacement"
else
    echo -e "${RED}❌ FAIL${NC}: Fichiers manquants"
    exit 1
fi

# Vérifier que build.rs utilise OUT_DIR
if grep -q "std::env::var(\"OUT_DIR\")" services/user/build.rs; then
    echo -e "${GREEN}✅ PASS${NC}: build.rs utilise OUT_DIR"
else
    echo -e "${RED}❌ FAIL${NC}: build.rs n'utilise pas OUT_DIR"
    exit 1
fi
echo ""

# Test 3: Vérification fichiers générés
echo "📋 Test 3: Vérification fichiers générés"
echo "----------------------------------------"

# Vérifier le contenu du code Rust généré
if grep -q "include_proto!" services/user/src/grpc/user.rs && \
   grep -q "UserHandler" services/user/src/grpc/user.rs && \
   grep -q "grpc_service" services/user/src/grpc/user.rs; then
    echo -e "${GREEN}✅ PASS${NC}: Code Rust généré correct (traits, serveur, client)"
else
    echo -e "${RED}❌ FAIL${NC}: Code Rust généré incomplet"
    exit 1
fi

# Vérifier le proto généré
if grep -q "service User" services/user/target/rustwork/grpc/user_service.proto && \
   grep -q "package user_service" services/user/target/rustwork/grpc/user_service.proto; then
    echo -e "${GREEN}✅ PASS${NC}: Proto généré correct"
else
    echo -e "${RED}❌ FAIL${NC}: Proto généré incorrect"
    exit 1
fi
echo ""

# Test 4: Exemple complet
echo "📋 Test 4: Exemple complet"
echo "-------------------------"

cd /home/linux/rustwork/examples/grpc_micro_minimal/grpc-example

if [ -d "services/user" ] && [ -d "services/auth" ]; then
    echo -e "${GREEN}✅ PASS${NC}: Structure exemple présente"
else
    echo -e "${RED}❌ FAIL${NC}: Exemple manquant"
    exit 1
fi

if cargo build --workspace --quiet 2>&1; then
    echo -e "${GREEN}✅ PASS${NC}: Exemple compile"
else
    echo -e "${RED}❌ FAIL${NC}: Erreur compilation exemple"
    exit 1
fi
echo ""

# Résumé
echo "============================================"
echo -e "${GREEN}✅ TOUS LES TESTS P0 PASSENT${NC}"
echo "============================================"
echo ""
echo "Support gRPC Rustwork validé au niveau P0 :"
echo "  ✅ Garde-fou monolithe"
echo "  ✅ Génération micro-services"
echo "  ✅ Fichiers au bon emplacement"
echo "  ✅ OUT_DIR respecté"
echo "  ✅ Compilation réussie"
echo "  ✅ Exemple complet fonctionnel"
echo ""
