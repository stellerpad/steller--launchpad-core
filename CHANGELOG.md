# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2026-06-16

### Added
- SAC-compatible token contract with minting, burning, and pausing capabilities
- Comprehensive vesting contract supporting linear, cliff, and hybrid strategies  
- Flexible airdrop contract with equal, weighted, and claimable distribution mechanisms
- Central launchpad registry for tracking all token launches
- Full-featured CLI tool with clap for all contract operations
- Automated deployment scripts for testnet and mainnet
- Comprehensive test suite with 45+ test cases covering all functionality
- Complete documentation for all contracts and usage examples
- Development guide and contribution guidelines

### Features
- **Token Contract**: SAC compatibility, admin controls, pausable functionality
- **Vesting Contract**: Multiple vesting strategies, revocable schedules, batch operations
- **Airdrop Contract**: Batch distribution, time-bound campaigns, individual claiming
- **Launchpad Registry**: Creator tracking, launch management, metadata storage
- **CLI Tool**: Complete command-line interface for all operations
- **Security**: Comprehensive authorization checks and input validation
- **Testing**: Extensive test coverage including happy path, error cases, and edge conditions

### Technical Details
- Built with Soroban SDK 21.0.0
- Rust workspace with proper dependency management
- Production-ready error handling and security measures
- Comprehensive logging and event emission
- Optimized for gas efficiency and performance

### Deployment
- Testnet deployment scripts and automation
- Mainnet deployment ready
- Contract verification and validation tools

### Documentation
- Complete API documentation for all contracts
- Usage examples and integration guides
- Development setup and contribution guidelines
- Comprehensive README with getting started guide

## [Unreleased]

### Planned
- Web dashboard integration
- Advanced analytics and reporting
- Multi-token launch batching
- Enhanced governance features
- Performance optimizations