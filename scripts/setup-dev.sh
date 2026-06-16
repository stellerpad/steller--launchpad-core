#!/bin/bash
# Development environment setup script

set -e

echo "🚀 Setting up Stellar Launchpad development environment..."

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo "❌ Rust is not installed. Please install Rust first:"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# Add wasm target if not present
echo "📦 Adding wasm32-unknown-unknown target..."
rustup target add wasm32-unknown-unknown

# Check if Stellar CLI is installed
if ! command -v stellar &> /dev/null; then
    echo "⬇️ Installing Stellar CLI..."
    curl -sL https://stellar.github.io/stellar-cli/install.sh | bash
    echo "✅ Stellar CLI installed. Please add ~/.local/bin to your PATH"
    echo "   export PATH=\"\$HOME/.local/bin:\$PATH\""
else
    echo "✅ Stellar CLI already installed"
fi

# Build the project
echo "🔨 Building the project..."
cargo build

echo "🧪 Running tests..."
cargo test

echo "✨ Development environment setup complete!"
echo ""
echo "Next steps:"
echo "1. Generate a new keypair: stellar keys generate test-account"
echo "2. Fund your account: stellar keys fund test-account --network testnet"
echo "3. Deploy contracts: ./scripts/deploy.sh"
echo "4. Start building! 🎉"