# Quick Start Guide

Get up and running with Stellar Launchpad Core in 5 minutes!

## Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [Stellar CLI](https://stellar.github.io/stellar-cli/install.sh)

## 1. Clone and Setup

```bash
git clone https://github.com/stellerpad/stellar-launchpad-core.git
cd stellar-launchpad-core

# Run automated setup
./scripts/setup-dev.sh
```

## 2. Generate Keys and Fund Account

```bash
# Generate a new keypair
stellar keys generate myaccount

# Fund your account on testnet
stellar keys fund myaccount --network testnet

# Set environment variable
export SOURCE_ACCOUNT=$(stellar keys address myaccount)
```

## 3. Deploy Contracts

```bash
# Deploy all contracts to testnet
./scripts/deploy.sh
```

## 4. Launch Your First Token

```bash
# Build the CLI
cd crates/cli
cargo build --release

# Launch a token
./target/release/stellar-launchpad launch \
    --name "My Awesome Token" \
    --symbol "MAT" \
    --supply 1000000 \
    --network testnet
```

## 5. Create a Vesting Schedule

```bash
# Create vesting for team members
./target/release/stellar-launchpad vesting create \
    --token <TOKEN_CONTRACT_ID> \
    --beneficiary <TEAM_MEMBER_ADDRESS> \
    --amount 100000 \
    --vesting-type linear \
    --vest-days 365 \
    --network testnet
```

## 6. Set up an Airdrop

```bash
# Create airdrop campaign
./target/release/stellar-launchpad airdrop create \
    --token <TOKEN_CONTRACT_ID> \
    --airdrop-type equal \
    --amount 50000 \
    --duration-days 30 \
    --network testnet
```

## Next Steps

- Read the [full documentation](docs/)
- Check out [example configurations](examples/)
- Join our [Discord community](https://discord.gg/stellar-dev)

Need help? Open an [issue](https://github.com/stellerpad/stellar-launchpad-core/issues) or check our [troubleshooting guide](docs/TROUBLESHOOTING.md).