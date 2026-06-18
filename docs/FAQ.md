# Frequently Asked Questions

## General Questions

### What is Stellar Launchpad Core?
Stellar Launchpad Core is a comprehensive Rust-based platform for launching tokens on the Stellar blockchain. It includes smart contracts for token management, vesting schedules, airdrops, and a central registry, plus a full CLI tool for easy interaction.

### What makes it different from other token launchers?
- **Native Stellar Integration**: Built specifically for Stellar using Soroban smart contracts
- **Comprehensive Testing**: 45+ test cases covering all functionality 
- **Production Ready**: Security-focused with proper authorization controls
- **CLI First**: Complete command-line interface for all operations
- **Modular Design**: Separate contracts that work together seamlessly

## Technical Questions

### What version of Stellar/Soroban does this support?
- Soroban SDK 21.0.0
- Stellar testnet and mainnet
- Rust 2021 edition

### Are the smart contracts audited?
The contracts follow security best practices and have comprehensive test coverage. For production use, we recommend professional security audits.

### Can I extend the contracts?
The contracts are designed with modularity in mind. While the contracts themselves are immutable once deployed, you can:
- Add new contracts that integrate with the existing ones
- Use the CLI as a foundation for custom tools
- Build web interfaces using the same contract bindings

## Usage Questions

### How do I launch my first token?
1. Follow the [Quick Start Guide](../QUICKSTART.md)
2. Run `stellar-launchpad launch --help` for all options
3. Start with testnet before moving to mainnet

### Can I create tokens without vesting or airdrops?
Yes! All features are optional. You can create a simple token without any additional features:

```bash
stellar-launchpad launch --name "SimpleToken" --symbol "SIM" --supply 1000000
```

### How do vesting schedules work?
We support three types:
- **Linear**: Tokens vest gradually over time
- **Cliff**: All tokens vest after a cliff period  
- **Hybrid**: Some tokens cliff, remainder vests linearly

### What airdrop types are supported?
- **Equal**: Same amount to all recipients
- **Weighted**: Different amounts per recipient
- **Claimable**: Recipients claim individually (gas efficient)

### How much does it cost to deploy?
Costs depend on Stellar network fees:
- Contract deployment: ~1-5 XLM per contract
- Token operations: ~0.0001 XLM per transaction
- See current fees at [stellar.org](https://stellar.org)

## Development Questions

### How do I contribute to the project?
1. Read the [Contributing Guide](../CONTRIBUTING.md)
2. Check [open issues](https://github.com/stellerpad/stellar-launchpad-core/issues)
3. Submit a pull request

### How do I report bugs?
1. Check existing [issues](https://github.com/stellerpad/stellar-launchpad-core/issues)
2. Create a new issue with:
   - Steps to reproduce
   - Expected vs actual behavior
   - Environment details (OS, Rust version, etc.)

### Can I use this in production?
The code is production-ready but we recommend:
1. Thorough testing on testnet
2. Security audit for high-value deployments  
3. Understanding the immutable nature of smart contracts

## Security Questions

### How secure are the smart contracts?
- All admin functions require cryptographic signatures
- Comprehensive input validation
- Extensive test coverage including security scenarios  
- Follow Stellar/Soroban security best practices

### What happens if I lose my admin keys?
Admin keys control important functions like minting and pausing. If lost:
- Basic token functions (transfer, approve) still work
- Admin functions become permanently inaccessible
- Always use secure key storage (hardware wallets)

### Can contracts be upgraded?
Soroban contracts are immutable once deployed. For upgrades:
- Deploy new contract versions
- Migrate data if needed
- Update your applications to use new addresses

## Integration Questions

### How do I integrate with my web app?
1. Use the generated client bindings
2. See examples in the `/examples` directory
3. Consider using our CLI as a reference implementation

### Can I build a web dashboard?
Yes! The contracts expose all necessary functions. You'll need:
- Contract bindings for your preferred language  
- Stellar SDK for wallet integration
- RPC endpoint for blockchain interaction

### How do I handle errors?
- Contracts use Rust's `panic!` for errors
- Check return values and handle errors gracefully
- See error handling patterns in our CLI code

## Troubleshooting

### "Contract not found" error
- Verify contract addresses are correct
- Ensure you're connected to the right network (testnet vs mainnet)
- Check that contracts are deployed

### "Unauthorized" error  
- Verify you're signing with the correct account
- Check that the account has the required permissions
- Ensure `require_auth()` is properly handled

### Gas/fee issues
- Check your account has sufficient XLM for fees
- Some operations require higher gas limits
- Consider batching operations to reduce costs

### Build errors
- Ensure you have the right Rust version (stable, latest)
- Add the wasm32 target: `rustup target add wasm32-unknown-unknown`
- Clear target directory: `cargo clean`

Still have questions? Join our [Discord](https://discord.gg/stellar-dev) or open an [issue](https://github.com/stellerpad/stellar-launchpad-core/issues)!