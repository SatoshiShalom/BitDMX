#!/usr/bin/env bash
set -e

echo "💻 Starting BitVMX-Z in development mode..."

# Start backend in background
echo "Starting backend..."
cargo run --bin bitvmx-z-node &
BACKEND_PID=$!

# Wait for backend to be ready
sleep 3

# Start frontend
echo "Starting frontend..."
cd frontend
npm install 2>/dev/null || true
npm run dev &
FRONTEND_PID=$!

cd ..

echo "✅ Development servers started!"
echo ""
echo "Backend API: http://localhost:3000"
echo "Frontend: http://localhost:5173"
echo ""
echo "Press Ctrl+C to stop both servers"

# Trap Ctrl+C to kill both processes
trap "kill $BACKEND_PID $FRONTEND_PID 2>/dev/null" EXIT

# Wait
wait
