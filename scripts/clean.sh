#!/bin/bash
set -e

echo "🧹 Cleaning build artifacts..."

# Clean Rust
cargo clean

# Clean frontend
if [ -d "frontend/node_modules" ]; then
    rm -rf frontend/node_modules
fi

if [ -d "frontend/dist" ]; then
    rm -rf frontend/dist
fi

# Clean data
if [ -d "data" ]; then
    rm -rf data
fi

echo "✅ Clean complete!"
