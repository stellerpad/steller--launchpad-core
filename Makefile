# Stellar Launchpad Core Makefile
# Provides convenient shortcuts for common development tasks

.PHONY: help build test clean format lint deploy setup install

# Default target
help:
	@echo "Stellar Launchpad Core - Available Commands:"
	@echo "  setup     - Set up development environment"
	@echo "  build     - Build all contracts and CLI"
	@echo "  test      - Run comprehensive test suite"
	@echo "  format    - Format all Rust code"
	@echo "  lint      - Run clippy linting"
	@echo "  clean     - Clean all build artifacts"
	@echo "  deploy    - Deploy contracts to testnet"
	@echo "  install   - Install CLI globally"
	@echo "  docs      - Generate documentation"

# Development setup
setup:
	@echo "🚀 Setting up development environment..."
	@./scripts/setup-dev.sh

# Build everything
build:
	@echo "🔨 Building all contracts and CLI..."
	@cargo build --all
	@cd contracts/token && stellar contract build
	@cd contracts/vesting && stellar contract build
	@cd contracts/airdrop && stellar contract build
	@cd contracts/launchpad && stellar contract build

# Run tests
test:
	@echo "🧪 Running test suite..."
	@./scripts/run-tests.sh

# Format code
format:
	@echo "🎨 Formatting code..."
	@./scripts/format.sh

# Lint code
lint:
	@echo "🔍 Running linter..."
	@cargo clippy --all-targets --all-features -- -D warnings

# Clean artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	@./scripts/clean.sh

# Deploy to testnet
deploy:
	@echo "🚀 Deploying to testnet..."
	@./scripts/deploy.sh

# Install CLI globally
install:
	@echo "📦 Installing CLI..."
	@cd crates/cli && cargo install --path .

# Generate documentation
docs:
	@echo "📚 Generating documentation..."
	@cargo doc --all --no-deps --open

# Development workflow shortcuts
dev-setup: setup build test
dev-check: format lint test
release: clean build test lint