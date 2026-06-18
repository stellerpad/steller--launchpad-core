# Stellar Launchpad Core

A comprehensive Rust CLI and Soroban smart contract backend for launching tokens on Stellar with built-in vesting schedules, airdrop distribution, and contributor allocation tools.

## 🚀 Features

- **Complete Token Launch Platform**: End-to-end solution for token launches on Stellar
- **SAC-Compatible Tokens**: Stellar Asset Contract compatible fungible tokens
- **Flexible Vesting**: Linear, cliff, and hybrid vesting strategies
- **Efficient Airdrops**: Equal, weighted, and claimable distribution mechanisms
- **Central Registry**: Track all launches with comprehensive metadata
- **CLI Interface**: Full command-line tool for all operations
- **Comprehensive Testing**: 45+ tests covering all functionality
- **Production Ready**: Security-focused with proper authorization controls

## 📋 Architecture

This repository contains 4 Soroban smart contracts and a comprehensive CLI tool:

### Smart Contracts

#### 🪙 Token Contract
- SAC-compatible fungible token implementation
- Configurable minting, burning, and pausing capabilities
- Admin controls with proper authorization
- **Tests**: 10 comprehensive test cases

#### 📅 Vesting Contract  
- **Linear Vesting**: Gradual token release over time
- **Cliff Vesting**: All tokens released after cliff period
- **Hybrid Vesting**: Combination of cliff amount + linear remainder
- Revocable and non-revocable schedules
- **Tests**: 12 comprehensive test cases

#### 🎁 Airdrop Contract
- **Equal Distribution**: Same amount to all recipients
- **Weighted Distribution**: Different amounts per recipient  
- **Claimable Campaigns**: Recipients claim individually
- Batch operations and time-bound campaigns
- **Tests**: 11 comprehensive test cases

#### 🏗️ Launchpad Registry Contract
- Central registry for all token launches
- Creator-based launch tracking
- Integration with vesting and airdrop contracts
- Admin oversight and launch management
- **Tests**: 10 comprehensive test cases

### 🔧 CLI Tool

The `stellar-launchpad` CLI provides comprehensive commands for:

```bash
# Launch Management
stellar-launchpad launch --name "MyToken" --symbol "MTK" --supply 1000000
stellar-launchpad status --launch-id 1 --network testnet
stellar-launchpad list --creator GXXXXX --active-only

# Vesting Operations
stellar-launchpad vesting create --token CXXXXX --beneficiary GXXXXX --amount 100000
stellar-launchpad vesting release --schedule-id 1
stellar-launchpad vesting check --schedule-id 1

# Airdrop Management  
stellar-launchpad airdrop create --token CXXXXX --recipients recipients.csv
stellar-launchpad airdrop distribute --campaign-id 1
stellar-launchpad airdrop claim --campaign-id 1

# Contract Deployment
stellar-launchpad deploy --contract token --network testnet
```

## 🚀 Quick Start

### Prerequisites

1. **Rust** (latest stable version):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup target add wasm32-unknown-unknown
   ```

2. **Stellar CLI**:
   ```bash
   curl -sL https://stellar.github.io/stellar-cli/install.sh | bash
   ```

### Installation & Setup

```bash
# Clone the repository
git clone https://github.com/stellerpad/stellar-launchpad-core.git
cd stellar-launchpad-core

# Build all contracts and CLI
cargo build

# Run comprehensive test suite
cargo test

# Build CLI for release
cd crates/cli
cargo build --release
```

### Deployment

```bash
# Set up your Stellar account (testnet)
stellar keys generate my-account
stellar keys fund my-account --network testnet

# Deploy all contracts to testnet
export SOURCE_ACCOUNT=$(stellar keys address my-account)
./scripts/deploy.sh

# Check deployment status
cat DEPLOYMENTS.md
```

### Usage Examples

#### Complete Token Launch Workflow

```bash
# 1. Launch a new token
stellar-launchpad launch \
    --name "MyAwesomeToken" \
    --symbol "MAT" \
    --supply 1000000000 \
    --decimals 7 \
    --description "Revolutionary DeFi token" \
    --website "https://mytoken.com" \
    --mintable \
    --network testnet

# 2. Create vesting schedules for team members
stellar-launchpad vesting create \
    --token $TOKEN_CONTRACT \
    --beneficiary $TEAM_MEMBER_ADDRESS \
    --amount 50000000 \
    --cliff-days 365 \
    --vest-days 1460 \
    --vesting-type hybrid \
    --cliff-amount 12500000 \
    --revocable \
    --network testnet

# 3. Set up community airdrop
stellar-launchpad airdrop create \
    --token $TOKEN_CONTRACT \
    --recipients community_members.csv \
    --airdrop-type claimable \
    --amount 100000000 \
    --duration-days 90 \
    --network testnet

# 4. Monitor launch status
stellar-launchpad status --launch-id 1 --network testnet
```

## 📁 Project Structure

```
stellar-launchpad-core/
├── crates/                   # Rust workspace members
│   ├── cli/                  # CLI binary: stellar-launchpad
│   │   ├── src/main.rs      # CLI implementation with clap
│   │   └── Cargo.toml       # CLI dependencies
│   └── client/              # Contract client helpers
├── contracts/               # Soroban smart contracts
│   ├── token/               # SAC-compatible token contract
│   │   ├── src/lib.rs      # Token implementation
│   │   └── test_snapshots/ # Test result snapshots
│   ├── vesting/             # Token vesting contract
│   │   ├── src/lib.rs      # Vesting strategies implementation
│   │   └── test_snapshots/
│   ├── airdrop/             # Airdrop distribution contract
│   │   ├── src/lib.rs      # Airdrop mechanisms implementation
│   │   └── test_snapshots/
│   └── launchpad/          # Launchpad registry contract
│       ├── src/lib.rs      # Registry implementation
│       └── test_snapshots/
├── docs/                    # Comprehensive documentation
│   ├── token.md            # Token contract documentation
│   ├── vesting.md          # Vesting contract documentation
│   ├── airdrop.md          # Airdrop contract documentation
│   └── launchpad.md        # Launchpad contract documentation
├── scripts/
│   └── deploy.sh           # Automated deployment script
├── DEPLOYMENTS.md          # Contract addresses and deployment info
├── CONTRIBUTING.md         # Development guide
└── Cargo.toml              # Workspace configuration
```

## 🧪 Testing

The project includes comprehensive test coverage:

```bash
# Run all tests (45+ test cases)
cargo test

# Run tests for specific contracts
cargo test -p stellar-launchpad-token      # 10 tests
cargo test -p stellar-launchpad-vesting    # 12 tests  
cargo test -p stellar-launchpad-airdrop    # 11 tests
cargo test -p stellar-launchpad-registry   # 10 tests
cargo test -p stellar-launchpad            # 2 CLI tests

# Test individual contracts
cd contracts/token && cargo test
cd contracts/vesting && cargo test
cd contracts/airdrop && cargo test
cd contracts/launchpad && cargo test
```

### Test Coverage

- **Happy Path Tests**: All core functionality working correctly
- **Authorization Tests**: Proper access control enforcement
- **Edge Case Tests**: Boundary conditions and error scenarios
- **Integration Tests**: Cross-contract interactions
- **CLI Tests**: Command-line interface functionality

## 🔒 Security

- **Authentication Required**: All admin functions protected by signature verification
- **Input Validation**: Comprehensive validation of all parameters
- **Access Controls**: Proper authorization checks throughout
- **Error Handling**: Robust error handling with clear messages
- **No todo!() Macros**: All functionality fully implemented
- **Comprehensive Testing**: Extensive test coverage including security scenarios

## 🌐 Stellar Integration

This project integrates deeply with the Stellar network and its ecosystem:

- **Soroban Smart Contracts**: All contracts are written for Stellar's Soroban VM and compiled to WASM
- **Stellar Asset Contract (SAC)**: Tokens are SAC-compatible, enabling native interoperability with the Stellar DEX, Horizon, and wallets
- **Stellar SDK**: Uses `soroban-sdk` for contract development and `stellar-sdk` in the CLI for transaction building and submission
- **Stellar CLI**: Contract deployment and invocation via the official `stellar` CLI tool
- **Horizon API**: Launch status and account queries are routed through the Horizon REST API
- **Friendbot / Testnet**: Funded test accounts via Stellar's Friendbot for fast local development
- **Testnet**: Fully deployed and tested
- **Mainnet**: Ready for production deployment

### Stellar Network Architecture

```
┌─────────────────────────────────────────────┐
│              stellar-launchpad CLI           │
│         (Transaction builder & signer)       │
└──────────┬──────────────────────┬────────────┘
           │                      │
    ┌──────▼──────┐        ┌──────▼──────┐
    │ Horizon API │        │ Soroban RPC │
    │  (queries)  │        │  (invokes)  │
    └──────┬──────┘        └──────┬──────┘
           │                      │
           └──────────┬───────────┘
                      │
           ┌──────────▼───────────┐
           │    Stellar Network   │
           │  ┌────────────────┐  │
           │  │ Token Contract │  │
           │  │ Vesting        │  │
           │  │ Airdrop        │  │
           │  │ Launchpad Reg. │  │
           │  └────────────────┘  │
           └──────────────────────┘
```

## 📚 Documentation

Detailed documentation for each contract:

- **[Token Contract](docs/token.md)**: SAC-compatible token implementation
- **[Vesting Contract](docs/vesting.md)**: Flexible vesting strategies  
- **[Airdrop Contract](docs/airdrop.md)**: Efficient distribution mechanisms
- **[Launchpad Contract](docs/launchpad.md)**: Central launch registry
- **[Contributing Guide](CONTRIBUTING.md)**: Development setup and guidelines

## 🤝 Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on:

- Development environment setup
- Building and testing procedures
- Code style guidelines
- Pull request process
- Adding new features

## 🔗 Ecosystem

This project is part of a larger ecosystem:

- **[Web Dashboard](https://github.com/stellerpad/stellar-launchpad-web)**: React-based user interface
- **[Documentation Site](https://github.com/stellerpad/stellar-launchpad-docs)**: Comprehensive user guides

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🎯 Roadmap

- [x] Core smart contracts implementation
- [x] Comprehensive CLI tool
- [x] Automated deployment scripts
- [x] Extensive test coverage
- [x] Documentation and guides
- [ ] Web dashboard integration
- [ ] Mainnet deployment
- [ ] Advanced analytics and reporting
- [ ] Multi-token launch batching
- [ ] Enhanced governance features

## 💡 Support

- **Issues**: [GitHub Issues](https://github.com/stellerpad/stellar-launchpad-core/issues)
- **Discussions**: [GitHub Discussions](https://github.com/stellerpad/stellar-launchpad-core/discussions)
- **Discord**: Join the Stellar developer community

---

Built with ❤️ for the Stellar ecosystem