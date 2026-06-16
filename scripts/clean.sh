#!/bin/bash
# Clean build artifacts and temporary files

set -e

echo "🧹 Cleaning Stellar Launchpad Core..."

# Clean Cargo build artifacts
echo "Cleaning Cargo build artifacts..."
cargo clean

# Clean any generated WASM files
echo "Cleaning WASM artifacts..."
find . -name "*.wasm" -type f -delete

# Clean deployment artifacts
echo "Cleaning deployment artifacts..."
if [ -f "DEPLOYMENTS.md" ]; then
    rm DEPLOYMENTS.md
fi

# Clean temporary files
echo "Cleaning temporary files..."
find . -name "*.tmp" -type f -delete
find . -name "*.temp" -type f -delete
find . -name "*~" -type f -delete

# Clean IDE files
echo "Cleaning IDE artifacts..."
rm -rf .vscode/settings.json 2>/dev/null || true
rm -rf .idea/ 2>/dev/null || true

echo "✨ Clean complete!"