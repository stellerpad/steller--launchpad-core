# Troubleshooting Guide

This guide helps you resolve common issues when working with Stellar Launchpad Core.

## Installation Issues

### Rust Installation Problems
**Error**: `rustc: command not found`
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Add WASM target
rustup target add wasm32-unknown-unknown
```

### Stellar CLI Installation Issues
**Error**: `stellar: command not found`
```bash
# Install Stellar CLI
curl -sL https://stellar.github.io/stellar-cli/install.sh | bash

# Add to PATH
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

## Build Issues

### Compilation Errors
**Error**: "package not found" or dependency issues
```bash
# Clean and rebuild
cargo clean
cargo build

# Update dependencies
cargo update
```

**Error**: WASM compilation fails
```bash
# Ensure WASM target is installed
rustup target add wasm32-unknown-unknown

# Build specific contract
cd contracts/token
stellar contract build
```

### Test Failures
**Error**: Tests fail with "auth required"
```bash
# In your test, ensure you mock authentication
env.mock_all_auths();
```

**Error**: "Contract not deployed" in tests
```bash
# Register the contract in your test
let contract_id = env.register_contract(None, YourContract);
```

## Deployment Issues

### Network Connection Problems
**Error**: "connection refused" or RPC errors
```bash
# Verify network configuration
stellar network ls

# Test connection
stellar keys fund test-account --network testnet
```

**Error**: Account not found
```bash
# Generate new account
stellar keys generate my-account

# Fund account on testnet
stellar keys fund my-account --network testnet
```

### Contract Deployment Failures
**Error**: "insufficient balance"
```bash
# Check account balance
stellar account get-details <ADDRESS> --network testnet

# Fund account if needed
stellar keys fund <ACCOUNT> --network testnet
```

**Error**: "contract already deployed"
```bash
# Use existing deployment or deploy with different salt
stellar contract deploy --wasm contract.wasm --source-account <ACCOUNT> --network testnet
```

## Runtime Issues

### CLI Errors
**Error**: "command not recognized"
```bash
# Build CLI first
cd crates/cli
cargo build --release

# Use full path or install globally
./target/release/stellar-launchpad --help
```

**Error**: "invalid network specified"
```bash
# Check available networks
stellar network ls

# Use correct network name
stellar-launchpad launch --network testnet
```

### Contract Interaction Errors
**Error**: "unauthorized access"
- Ensure you're signing with the correct account
- Check that the account has required permissions
- Verify the contract admin settings

**Error**: "invalid parameters"
- Check parameter types and ranges
- Ensure addresses are valid Stellar addresses
- Verify numeric values are within bounds

## Performance Issues

### Slow Builds
```bash
# Use release mode for faster execution
cargo build --release

# Parallel compilation
export CARGO_BUILD_JOBS=4
cargo build
```

### High Gas Costs
- Batch operations when possible
- Optimize contract logic
- Use efficient data structures
- Minimize cross-contract calls

## Common Error Patterns

### "Contract Trap" Errors
These indicate panic! in contract code:
1. Check input parameters
2. Verify account has required permissions  
3. Ensure contract is in valid state

### Network Timeout Issues
1. Check internet connection
2. Try different RPC endpoint
3. Increase timeout values
4. Verify network is operational

### Memory/Storage Issues
1. Clear target directory: `cargo clean`
2. Free up disk space
3. Restart development environment

## Debugging Tips

### Enable Detailed Logging
```bash
# Set log level for more details
export RUST_LOG=debug
cargo test

# For Stellar CLI
export STELLAR_RPC_LOG_LEVEL=debug
```

### Contract State Inspection
```bash
# Check contract storage
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source-account <ACCOUNT> \
  --network testnet \
  -- get_admin
```

### Transaction Analysis
```bash
# Inspect failed transactions
stellar transaction inspect <TRANSACTION_ID> --network testnet
```

## Getting Help

### Check Common Issues
1. Review [FAQ](FAQ.md)
2. Check [GitHub Issues](https://github.com/your-org/stellar-launchpad-core/issues)
3. Search [Stellar Documentation](https://developers.stellar.org/)

### Report New Issues
Include in your issue:
- Operating system and version
- Rust version (`rustc --version`)
- Stellar CLI version (`stellar --version`)
- Complete error message
- Steps to reproduce
- Expected vs actual behavior

### Community Support
- [Stellar Discord](https://discord.gg/stellar-dev)
- [Stellar Stack Overflow](https://stackoverflow.com/questions/tagged/stellar)
- [GitHub Discussions](https://github.com/your-org/stellar-launchpad-core/discussions)

## Emergency Procedures

### Contract Emergency Stop
If you need to pause operations:
```bash
# Pause token (if pausable)
stellar-launchpad token pause --token <TOKEN_ID> --network <NETWORK>

# Cancel airdrop campaigns
stellar-launchpad airdrop cancel --campaign-id <ID> --network <NETWORK>
```

### Key Compromise
If admin keys are compromised:
1. Immediately pause all pausable contracts
2. Revoke any active vesting schedules (if revocable)
3. Cancel ongoing airdrop campaigns
4. Deploy new contracts with new admin keys
5. Migrate assets if possible

### Network Issues
During Stellar network outages:
1. Operations will queue until network recovers
2. Monitor [Stellar Status](https://status.stellar.org/)
3. Consider using alternative RPC endpoints
4. Be patient - transactions will eventually process

Remember: Smart contracts are immutable, so always test thoroughly on testnet before mainnet deployment!