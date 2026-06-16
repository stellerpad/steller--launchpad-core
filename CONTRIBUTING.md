# Contributing to Stellar Launchpad Core

Welcome to Stellar Launchpad Core! This guide will help you set up your development environment and contribute to the project.

## Table of Contents

- [Development Setup](#development-setup)
- [Building the Project](#building-the-project)
- [Testing](#testing)
- [Deployment](#deployment)
- [Adding New Features](#adding-new-features)
- [Code Style Guidelines](#code-style-guidelines)
- [Submitting Changes](#submitting-changes)

## Development Setup

### Prerequisites

1. **Rust** (latest stable version)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup target add wasm32-unknown-unknown
   ```

2. **Stellar CLI** 
   ```bash
   curl -sL https://stellar.github.io/stellar-cli/install.sh | bash
   ```

3. **Node.js** (for additional tooling)
   ```bash
   # Install via your package manager or from nodejs.org
   ```

### Project Structure

```
stellar-launchpad-core/
├── contracts/           # Soroban smart contracts
│   ├── token/          # SAC-compatible token contract
│   ├── vesting/        # Token vesting contract
│   ├── airdrop/        # Airdrop distribution contract
│   └── launchpad/      # Main launchpad registry
├── crates/             # Rust workspace members
│   ├── cli/            # Command-line interface
│   └── client/         # Contract client helpers
├── docs/               # Contract documentation
├── scripts/            # Deployment and utility scripts
└── target/             # Build artifacts
```

## Building the Project

### Build All Contracts

```bash
# Build the entire workspace
cargo build

# Build individual contracts
cd contracts/token && cargo build
cd contracts/vesting && cargo build
cd contracts/airdrop && cargo build
cd contracts/launchpad && cargo build
```

### Build CLI Tool

```bash
cd crates/cli
cargo build --release
```

### Build for WASM (Smart Contracts)

```bash
# Build all contracts for deployment
cd contracts/token && stellar contract build
cd contracts/vesting && stellar contract build
cd contracts/airdrop && stellar contract build
cd contracts/launchpad && stellar contract build
```

## Testing

### Run All Tests

```bash
# Test the entire workspace
cargo test

# Test individual components
cargo test -p stellar-launchpad-token
cargo test -p stellar-launchpad-vesting
cargo test -p stellar-launchpad-airdrop
cargo test -p stellar-launchpad-registry
cargo test -p stellar-launchpad
```

### Contract-Specific Testing

```bash
# Test individual contracts
cd contracts/token && cargo test
cd contracts/vesting && cargo test
cd contracts/airdrop && cargo test
cd contracts/launchpad && cargo test
```

### CLI Testing

```bash
cd crates/cli
cargo test

# Manual CLI testing
cargo run -- --help
cargo run -- launch --name "TestToken" --symbol "TST" --supply 1000000
```

## Deployment

### Local Development

1. **Start Stellar Testnet**
   ```bash
   # Use Stellar CLI to configure testnet
   stellar network add --global testnet \
     --rpc-url https://soroban-testnet.stellar.org:443 \
     --network-passphrase "Test SDF Network ; September 2015"
   ```

2. **Create Test Account**
   ```bash
   stellar keys generate test-account
   stellar keys fund test-account --network testnet
   ```

3. **Deploy Contracts**
   ```bash
   # Set your source account
   export SOURCE_ACCOUNT=$(stellar keys address test-account)
   
   # Run deployment script
   ./scripts/deploy.sh
   ```

### Manual Deployment

```bash
# Build contracts
stellar contract build

# Deploy individual contracts
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/stellar_launchpad_token.wasm \
  --source-account $SOURCE_ACCOUNT \
  --network testnet
```

## Adding New Features

### Adding a New Contract Function

1. **Define the Function**
   ```rust
   // In contracts/*/src/lib.rs
   pub fn new_function(env: Env, param: Type) -> ReturnType {
       // Require authentication if needed
       param.require_auth();
       
       // Implement function logic
       // ...
       
       // Log important events
       log!(&env, "Function executed with param: {}", param);
   }
   ```

2. **Add Tests**
   ```rust
   #[cfg(test)]
   mod test {
       use super::*;
       
       #[test]
       fn test_new_function() {
           let env = Env::default();
           env.mock_all_auths();
           
           // Test implementation
           // ...
       }
   }
   ```

3. **Update Documentation**
   - Add function description to relevant docs/*.md file
   - Include usage examples
   - Document parameters and return values

### Adding a New CLI Command

1. **Define Command Structure**
   ```rust
   // In crates/cli/src/main.rs
   #[derive(Subcommand)]
   enum Commands {
       // ... existing commands
       NewCommand {
           #[arg(long)]
           param: String,
       },
   }
   ```

2. **Implement Command Logic**
   ```rust
   Commands::NewCommand { param } => {
       println!("Executing new command with: {}", param);
       // Implementation
   }
   ```

3. **Add Tests**
   ```rust
   #[test]
   fn test_new_command() {
       // Test command logic
   }
   ```

### Adding a New CLI Command

Follow the existing pattern in `crates/cli/src/main.rs`:

1. Add to the `Commands` enum
2. Implement the command handler in the match statement
3. Add appropriate tests
4. Update the help documentation

## Code Style Guidelines

### Rust Code Style

- Follow `rustfmt` formatting (run `cargo fmt`)
- Use `clippy` for linting (run `cargo clippy`)
- Write descriptive variable and function names
- Include comprehensive documentation comments
- Use `Result<T, E>` for error handling, panic! for contract errors

### Contract Development Best Practices

1. **Security First**
   - Always use `require_auth()` for sensitive functions
   - Validate all inputs thoroughly  
   - Handle edge cases explicitly

2. **Error Handling**
   ```rust
   // Use panic! for contract errors (they become ContractError)
   if amount <= 0 {
       panic!("Amount must be positive");
   }
   
   // Use proper error types for internal logic
   ```

3. **Testing Requirements**
   - Minimum 8 tests per contract
   - Test happy paths, error conditions, and edge cases
   - Use `env.mock_all_auths()` for testing
   - Test authorization failures with `#[should_panic]`

4. **Documentation**
   - Document all public functions
   - Include usage examples
   - Explain complex business logic
   - Document security considerations

### CLI Development Best Practices

1. **User Experience**
   - Provide helpful error messages
   - Use consistent command structure
   - Include progress indicators for long operations
   - Validate inputs before processing

2. **Configuration Management**
   - Support environment variables
   - Provide sane defaults
   - Allow configuration overrides

## Submitting Changes

### Pull Request Process

1. **Fork and Clone**
   ```bash
   git clone https://github.com/your-username/stellar-launchpad-core.git
   cd stellar-launchpad-core
   ```

2. **Create Feature Branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

3. **Make Changes**
   - Follow coding standards
   - Add/update tests
   - Update documentation
   - Test thoroughly

4. **Commit Changes**
   ```bash
   git add .
   git commit -m "feat: add new feature description
   
   - Detailed change description
   - Why the change is needed
   - How it was implemented"
   ```

5. **Push and Create PR**
   ```bash
   git push origin feature/your-feature-name
   # Create pull request on GitHub
   ```

### Commit Message Format

Use conventional commits format:

```
type(scope): description

- Detailed explanation
- Why the change was made
- How it affects users
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `test`: Test additions/modifications
- `refactor`: Code refactoring
- `chore`: Maintenance tasks

### Code Review Checklist

Before submitting a PR, ensure:

- [ ] All tests pass (`cargo test`)
- [ ] Code is properly formatted (`cargo fmt`)
- [ ] No linting warnings (`cargo clippy`)
- [ ] Documentation is updated
- [ ] Security considerations are addressed
- [ ] Breaking changes are documented
- [ ] Performance impact is considered

## Development Workflow

### Typical Development Cycle

1. **Setup Development Environment**
   ```bash
   # Clone repo and setup
   git clone <repo-url>
   cd stellar-launchpad-core
   cargo build
   ```

2. **Make Changes**
   ```bash
   # Create feature branch
   git checkout -b feature/new-feature
   
   # Edit code
   # Add tests
   # Update docs
   ```

3. **Test Locally**
   ```bash
   # Run all tests
   cargo test
   
   # Test CLI manually
   cd crates/cli
   cargo run -- launch --name "Test" --symbol "TST" --supply 1000
   ```

4. **Deploy and Test on Testnet**
   ```bash
   # Deploy contracts
   ./scripts/deploy.sh
   
   # Test with real contracts
   stellar-launchpad status --launch-id 1 --network testnet
   ```

5. **Submit Changes**
   ```bash
   git add .
   git commit -m "feat: implement new feature"
   git push origin feature/new-feature
   # Create PR on GitHub
   ```

## Getting Help

### Resources

- [Stellar Documentation](https://developers.stellar.org/)
- [Soroban Documentation](https://soroban.stellar.org/)
- [Rust Documentation](https://doc.rust-lang.org/)

### Community

- **GitHub Issues**: Report bugs and request features
- **Discussions**: Ask questions and share ideas
- **Discord**: Join the Stellar developer community

### Sister Repositories

This project is part of a larger ecosystem:

- **Web Dashboard**: [stellar-launchpad-web](https://github.com/<org>/stellar-launchpad-web)
- **Documentation Site**: [stellar-launchpad-docs](https://github.com/<org>/stellar-launchpad-docs)

## Release Process

1. **Version Bump**: Update version in all Cargo.toml files
2. **Changelog**: Update CHANGELOG.md with new features and fixes
3. **Testing**: Comprehensive testing on testnet
4. **Documentation**: Update all relevant documentation
5. **Deployment**: Deploy to testnet, then mainnet
6. **Release**: Create GitHub release with binaries

## Security

- Report security vulnerabilities privately to the maintainers
- Follow secure coding practices
- Regular security audits of contract code
- Keep dependencies up to date

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

Thank you for contributing to Stellar Launchpad Core! 🚀