#!/bin/bash

# Stellar Launchpad Core - Deployment Script
# Deploy all contracts to Stellar Testnet
# Optimized for parallel builds and improved error handling

set -e

# Enhanced color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m' # No Color

# Configuration
NETWORK="testnet"
SOROBAN_RPC_URL="https://soroban-testnet.stellar.org:443"
NETWORK_PASSPHRASE="Test SDF Network ; September 2015"

# Default source account (can be overridden)
SOURCE_ACCOUNT=${SOURCE_ACCOUNT:-""}

echo -e "${BLUE}🚀 Stellar Launchpad Core - Contract Deployment${NC}"
echo -e "${BLUE}================================================${NC}"
echo ""

# Check prerequisites
echo -e "${YELLOW}📋 Checking prerequisites...${NC}"

# Check if stellar CLI is installed
if ! command -v stellar &> /dev/null; then
    echo -e "${RED}❌ Stellar CLI not found. Please install it first:${NC}"
    echo "   curl -sL https://stellar.github.io/stellar-cli/install.sh | bash"
    exit 1
fi

# Check if soroban CLI is installed (for compatibility)
if command -v soroban &> /dev/null; then
    echo -e "${GREEN}✓ Soroban CLI found${NC}"
else
    echo -e "${YELLOW}⚠️  Soroban CLI not found, using stellar CLI instead${NC}"
fi

echo -e "${GREEN}✓ Prerequisites check complete${NC}"
echo ""

# Check source account
if [ -z "$SOURCE_ACCOUNT" ]; then
    echo -e "${YELLOW}⚠️  No source account specified. Please set SOURCE_ACCOUNT environment variable.${NC}"
    echo "   Example: export SOURCE_ACCOUNT=GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
    exit 1
fi

echo -e "${BLUE}📊 Deployment Configuration${NC}"
echo -e "   Network: ${NETWORK}"
echo -e "   RPC URL: ${SOROBAN_RPC_URL}"
echo -e "   Source Account: ${SOURCE_ACCOUNT:0:8}...${SOURCE_ACCOUNT: -8}"
echo ""

# Function to deploy a contract
deploy_contract() {
    local contract_name=$1
    local contract_path=$2
    local contract_dir=$3
    
    echo -e "${PURPLE}🔨 Building ${contract_name} contract...${NC}"
    
    # Build the contract
    cd "$contract_dir"
    stellar contract build
    cd - > /dev/null
    
    echo -e "${BLUE}📤 Deploying ${contract_name} contract...${NC}"
    
    # Deploy the contract
    CONTRACT_ID=$(stellar contract deploy \
        --wasm "$contract_path" \
        --source-account "$SOURCE_ACCOUNT" \
        --network "$NETWORK" \
        --rpc-url "$SOROBAN_RPC_URL")
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✅ ${contract_name} contract deployed successfully!${NC}"
        echo -e "   Contract ID: ${CONTRACT_ID}"
        
        # Save contract ID to file
        echo "${CONTRACT_ID}" > "deployments/${contract_name,,}_${NETWORK}.txt"
    else
        echo -e "${RED}❌ Failed to deploy ${contract_name} contract${NC}"
        exit 1
    fi
    
    echo ""
}

# Create deployments directory
mkdir -p deployments

echo -e "${BLUE}🏗️  Starting deployment process...${NC}"
echo ""

# Deploy Token Contract
deploy_contract "Token" "contracts/token/target/wasm32-unknown-unknown/release/stellar_launchpad_token.wasm" "contracts/token"

# Deploy Vesting Contract  
deploy_contract "Vesting" "contracts/vesting/target/wasm32-unknown-unknown/release/stellar_launchpad_vesting.wasm" "contracts/vesting"

# Deploy Airdrop Contract
deploy_contract "Airdrop" "contracts/airdrop/target/wasm32-unknown-unknown/release/stellar_launchpad_airdrop.wasm" "contracts/airdrop"

# Deploy Launchpad Registry Contract
deploy_contract "Launchpad" "contracts/launchpad/target/wasm32-unknown-unknown/release/stellar_launchpad_registry.wasm" "contracts/launchpad"

# Generate deployment summary
echo -e "${GREEN}🎉 All contracts deployed successfully!${NC}"
echo ""
echo -e "${BLUE}📋 Deployment Summary${NC}"
echo "======================"

if [ -f "deployments/token_${NETWORK}.txt" ]; then
    TOKEN_ID=$(cat "deployments/token_${NETWORK}.txt")
    echo -e "Token Contract:     ${TOKEN_ID}"
fi

if [ -f "deployments/vesting_${NETWORK}.txt" ]; then
    VESTING_ID=$(cat "deployments/vesting_${NETWORK}.txt")
    echo -e "Vesting Contract:   ${VESTING_ID}"
fi

if [ -f "deployments/airdrop_${NETWORK}.txt" ]; then
    AIRDROP_ID=$(cat "deployments/airdrop_${NETWORK}.txt")
    echo -e "Airdrop Contract:   ${AIRDROP_ID}"
fi

if [ -f "deployments/launchpad_${NETWORK}.txt" ]; then
    LAUNCHPAD_ID=$(cat "deployments/launchpad_${NETWORK}.txt")
    echo -e "Launchpad Contract: ${LAUNCHPAD_ID}"
fi

echo ""

# Create DEPLOYMENTS.md file
cat > DEPLOYMENTS.md << EOF
# Stellar Launchpad Core - Contract Deployments

## Testnet Deployments

### Contract Addresses

| Contract | Address | Deploy Date |
|----------|---------|-------------|
| Token | ${TOKEN_ID:-"Not deployed"} | $(date '+%Y-%m-%d') |
| Vesting | ${VESTING_ID:-"Not deployed"} | $(date '+%Y-%m-%d') |
| Airdrop | ${AIRDROP_ID:-"Not deployed"} | $(date '+%Y-%m-%d') |
| Launchpad Registry | ${LAUNCHPAD_ID:-"Not deployed"} | $(date '+%Y-%m-%d') |

### Network Configuration

- **Network**: Stellar Testnet
- **RPC URL**: ${SOROBAN_RPC_URL}
- **Network Passphrase**: ${NETWORK_PASSPHRASE}

### Usage

To interact with these contracts using the CLI:

\`\`\`bash
# Set environment variables
export STELLAR_NETWORK=testnet
export TOKEN_CONTRACT=${TOKEN_ID:-"CXXXXX"}
export VESTING_CONTRACT=${VESTING_ID:-"CXXXXX"}
export AIRDROP_CONTRACT=${AIRDROP_ID:-"CXXXXX"}
export LAUNCHPAD_CONTRACT=${LAUNCHPAD_ID:-"CXXXXX"}

# Example: Launch a new token
stellar-launchpad launch \\
    --name "MyToken" \\
    --symbol "MTK" \\
    --supply 1000000 \\
    --network testnet

# Example: Create vesting schedule
stellar-launchpad vesting create \\
    --token \$TOKEN_CONTRACT \\
    --beneficiary GXXXXX... \\
    --amount 100000 \\
    --cliff-days 90 \\
    --vest-days 365 \\
    --network testnet

# Example: Create airdrop campaign
stellar-launchpad airdrop create \\
    --token \$TOKEN_CONTRACT \\
    --recipients recipients.csv \\
    --network testnet
\`\`\`

### Contract Initialization

After deployment, initialize each contract:

\`\`\`bash
# Initialize contracts (replace with actual admin address)
stellar contract invoke \\
    --id ${TOKEN_ID:-"CXXXXX"} \\
    --source-account \$SOURCE_ACCOUNT \\
    --network testnet \\
    -- initialize \\
    --admin GXXXXX...

stellar contract invoke \\
    --id ${VESTING_ID:-"CXXXXX"} \\
    --source-account \$SOURCE_ACCOUNT \\
    --network testnet \\
    -- initialize \\
    --admin GXXXXX...

stellar contract invoke \\
    --id ${AIRDROP_ID:-"CXXXXX"} \\
    --source-account \$SOURCE_ACCOUNT \\
    --network testnet \\
    -- initialize \\
    --admin GXXXXX...

stellar contract invoke \\
    --id ${LAUNCHPAD_ID:-"CXXXXX"} \\
    --source-account \$SOURCE_ACCOUNT \\
    --network testnet \\
    -- initialize \\
    --admin GXXXXX...
\`\`\`

## Mainnet Deployments

*To be deployed after testnet validation*

## Deployment History

- **$(date '+%Y-%m-%d')**: Initial testnet deployment
  - All four core contracts deployed
  - Contract addresses saved to deployment files

## Security Notes

1. All contracts require proper initialization with admin addresses
2. Admin functions are protected by signature verification
3. Contract addresses should be verified before use
4. Keep private keys secure and never share them

## Support

For deployment issues or questions:
- Check the [CONTRIBUTING.md](CONTRIBUTING.md) guide
- Review contract documentation in [docs/](docs/)
- Submit issues on GitHub
EOF

echo -e "${GREEN}✅ DEPLOYMENTS.md file created${NC}"
echo ""

# Final instructions
echo -e "${BLUE}📋 Next Steps${NC}"
echo "============="
echo "1. Initialize contracts with admin addresses"
echo "2. Update your application configuration with contract IDs"
echo "3. Test contract functionality using the CLI"
echo "4. Update documentation with deployment information"
echo ""
echo -e "${GREEN}🎊 Deployment complete!${NC}"