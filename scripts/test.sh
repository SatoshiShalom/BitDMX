#!/bin/bash
set -e

echo "🧪 Running tests..."

# Test Rust workspace
echo "Testing Rust crates..."
cargo test --workspace

# Test frontend (if node_modules exist)
if [ -d "frontend/node_modules" ]; then
    echo "Testing frontend..."
    cd frontend
    npm test
    cd ..
else
    echo "⚠️  Skipping frontend tests (run 'cd frontend && npm install' first)"
fi

echo "✅ All tests passed!"
