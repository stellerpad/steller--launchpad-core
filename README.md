# Stellar Launchpad Core

A Rust CLI and Soroban smart contract backend for launching tokens on Stellar with built-in vesting schedules, airdrop distribution, and contributor allocation tools.

## Architecture

This repo contains 4 Soroban smart contracts and a CLI tool:

### Smart Contracts
- **Token Contract**: SAC-compatible fungible token with launch configuration
- **Vesting Contract**: Token release scheduling with linear/cliff/hybrid vesting
- **Airdrop Contract**: Efficient token distribution to multiple recipients  
- **Launchpad Registry**: Central registry tracking all token launches

### CLI Tool
The `stellar-launchpad` CLI provides commands for:
- Launching new tokens
- Creating vesting schedules
- Managing airdrop campaigns
- Querying launch status

## Quick Start

```bash
# Build all contracts and CLI
cargo build

# Run tests
cargo test

# Deploy to testnet (requires Stellar CLI)
./scripts/deploy.sh testnet

# Launch a token
stellar-launchpad launch --name "MyToken" --symbol "MTK" --supply 1000000 --decimals 7 --network testnet
```

## Project Structure

```
stellar-launchpad-core/
├── crates/
│   ├── cli/                  # Binary: stellar-launchpad
│   └── client/               # Soroban contract client helpers
├── contracts/
│   ├── token/                # SAC-compatible token contract
│   ├── vesting/              # Token vesting contract
│   ├── airdrop/              # Airdrop distribution contract
│   └── launchpad/            # Main launchpad registry contract
├── docs/                     # Contract documentation
├── scripts/
│   └── deploy.sh             # Stellar CLI deployment script
├── DEPLOYMENTS.md            # Testnet contract addresses
└── CONTRIBUTING.md
```

## Related Projects

- Web Dashboard: https://github.com/<your-org>/stellar-launchpad-web
- Documentation Site: https://github.com/<your-org>/stellar-launchpad-docs

## License

MIT