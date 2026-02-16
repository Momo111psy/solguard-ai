#!/bin/bash
set -e

echo "🚀 SOLGuard AI - Devnet Deployment Script"
echo "=========================================="
echo ""

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Check if we're in the right directory
if [ ! -f "Anchor.toml" ]; then
    echo -e "${RED}❌ Error: Anchor.toml not found. Please run this script from the solguard-ai directory.${NC}"
    exit 1
fi

echo -e "${YELLOW}Step 1: Checking environment...${NC}"

# Check Solana CLI
if ! command -v solana &> /dev/null; then
    echo -e "${RED}❌ Solana CLI not found. Please install it first.${NC}"
    exit 1
fi

# Check Anchor
if ! command -v anchor &> /dev/null; then
    echo -e "${RED}❌ Anchor not found. Please install it first.${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Environment check passed${NC}"
echo ""

echo -e "${YELLOW}Step 2: Configuring for Devnet...${NC}"
solana config set --url devnet
echo -e "${GREEN}✅ Configured for Devnet${NC}"
echo ""

echo -e "${YELLOW}Step 3: Checking wallet balance...${NC}"
BALANCE=$(solana balance | awk '{print $1}')
echo "Current balance: $BALANCE SOL"

if (( $(echo "$BALANCE < 10" | bc -l) )); then
    echo -e "${YELLOW}⚠️  Low balance. You need ~10-15 SOL to deploy all 4 programs.${NC}"
    echo "Getting airdrop..."
    
    # Try to get airdrops
    for i in {1..5}; do
        solana airdrop 2 2>/dev/null || true
        sleep 2
    done
    
    BALANCE=$(solana balance | awk '{print $1}')
    echo "New balance: $BALANCE SOL"
    
    if (( $(echo "$BALANCE < 10" | bc -l) )); then
        echo -e "${RED}❌ Still insufficient balance. Please get Devnet SOL from:${NC}"
        echo "   - https://faucet.quicknode.com/solana/devnet"
        echo "   - https://faucet.solana.com/"
        echo ""
        echo "Your wallet address: $(solana address)"
        exit 1
    fi
fi

echo -e "${GREEN}✅ Sufficient balance for deployment${NC}"
echo ""

echo -e "${YELLOW}Step 4: Cleaning previous builds...${NC}"
anchor clean
rm -f Cargo.lock
echo -e "${GREEN}✅ Clean complete${NC}"
echo ""

echo -e "${YELLOW}Step 5: Building all programs...${NC}"
echo "This will take 5-10 minutes..."
echo ""

# Build with proper flags
if anchor build; then
    echo -e "${GREEN}✅ Build successful!${NC}"
else
    echo -e "${RED}❌ Build failed. Check errors above.${NC}"
    exit 1
fi

echo ""
echo -e "${YELLOW}Step 6: Verifying build artifacts...${NC}"

PROGRAMS=("security_oracle" "validator_registry" "governance_module" "transparency_vault" "solguard_token")
ALL_BUILT=true

for program in "${PROGRAMS[@]}"; do
    if [ -f "target/deploy/${program}.so" ]; then
        SIZE=$(ls -lh "target/deploy/${program}.so" | awk '{print $5}')
        echo -e "${GREEN}✅ ${program}.so (${SIZE})${NC}"
    else
        echo -e "${RED}❌ ${program}.so not found${NC}"
        ALL_BUILT=false
    fi
done

if [ "$ALL_BUILT" = false ]; then
    echo -e "${RED}❌ Some programs failed to build${NC}"
    exit 1
fi

echo ""
echo -e "${YELLOW}Step 7: Deploying to Devnet...${NC}"
echo "This will deploy all 4 programs..."
echo ""

# Deploy using anchor
if anchor deploy; then
    echo ""
    echo -e "${GREEN}✅ Deployment successful!${NC}"
else
    echo -e "${RED}❌ Deployment failed${NC}"
    exit 1
fi

echo ""
echo -e "${GREEN}=========================================="
echo "🎉 SOLGuard AI Deployed Successfully!"
echo "==========================================${NC}"
echo ""

# Extract and display program IDs
echo "📋 Program IDs:"
echo ""

if [ -f "target/deploy/security_oracle-keypair.json" ]; then
    SECURITY_ORACLE_ID=$(solana-keygen pubkey target/deploy/security_oracle-keypair.json)
    echo "Security Oracle:      $SECURITY_ORACLE_ID"
fi

if [ -f "target/deploy/validator_registry-keypair.json" ]; then
    VALIDATOR_REGISTRY_ID=$(solana-keygen pubkey target/deploy/validator_registry-keypair.json)
    echo "Validator Registry:   $VALIDATOR_REGISTRY_ID"
fi

if [ -f "target/deploy/governance_module-keypair.json" ]; then
    GOVERNANCE_ID=$(solana-keygen pubkey target/deploy/governance_module-keypair.json)
    echo "Governance:           $GOVERNANCE_ID"
fi

if [ -f "target/deploy/transparency_vault-keypair.json" ]; then
    TRANSPARENCY_VAULT_ID=$(solana-keygen pubkey target/deploy/transparency_vault-keypair.json)
    echo "Transparency Vault:   $TRANSPARENCY_VAULT_ID"
fi

if [ -f "target/deploy/solguard_token-keypair.json" ]; then
    SOLGUARD_TOKEN_ID=$(solana-keygen pubkey target/deploy/solguard_token-keypair.json)
    echo "SolGuard Token:       $SOLGUARD_TOKEN_ID"
fi

echo ""
echo "🔗 View on Solana Explorer:"
echo "https://explorer.solana.com/address/$SECURITY_ORACLE_ID?cluster=devnet"
echo ""

FINAL_BALANCE=$(solana balance | awk '{print $1}')
echo "💰 Remaining balance: $FINAL_BALANCE SOL"
echo ""

echo -e "${GREEN}✅ All done! SOLGuard AI is now live on Devnet!${NC}"
echo ""
echo "Next steps:"
echo "1. Run tests on live network: anchor test"
echo "2. Update README with program IDs"
echo "3. Push changes to GitHub"
echo ""
