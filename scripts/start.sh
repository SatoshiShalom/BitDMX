#!/usr/bin/env bash
set -e

echo "🚀 Starting BitVMX-Z with Docker..."

# Build and start services
docker-compose up --build -d

echo "✅ Services started!"
echo ""
echo "Backend API: http://localhost:3000"
echo "Frontend: http://localhost:5173"
echo ""
echo "To view logs:"
echo "  docker-compose logs -f"
echo ""
echo "To stop:"
echo "  docker-compose down"
