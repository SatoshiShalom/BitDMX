#!/bin/bash
set -e

echo "🏗️  Building BitVMX-Z locally..."

# Build Rust workspace
echo "Building Rust workspace..."
cargo build --release

# Build frontend
echo "Building frontend..."
cd frontend
if [ ! -d "node_modules" ]; then
    echo "Installing frontend dependencies..."
    npm install
fi
npm run build
cd ..

echo "✅ Build complete!"
echo ""
echo "Backend binary: ./target/release/bitvmx-z-node"
echo "Frontend dist: ./frontend/dist"
