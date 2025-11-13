#!/usr/bin/env bash
set -e

echo "🛑 Stopping BitVMX-Z..."

docker-compose down

echo "✅ Services stopped!"
