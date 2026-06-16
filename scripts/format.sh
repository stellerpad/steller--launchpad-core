#!/bin/bash
# Format all Rust code in the workspace

set -e

echo "🎨 Formatting Stellar Launchpad Core codebase..."

# Format all Rust code
echo "Running rustfmt..."
cargo fmt --all

# Check if there were any changes
if git diff --quiet; then
    echo "✅ Code is already properly formatted!"
else
    echo "📝 Code has been formatted. Review changes with: git diff"
    
    # Show summary of changed files
    echo "Modified files:"
    git diff --name-only | while read file; do
        echo "  - $file"
    done
fi

echo "🎯 Format complete!"