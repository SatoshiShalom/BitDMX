# Quick Start Guide

Get BitVMX-Z up and running in 5 minutes.

## Prerequisites Check

```bash
# Check versions
rust --version    # Should be ≥ 1.75
node --version    # Should be ≥ 20
docker --version  # Any recent version
```

## Option 1: Docker (Recommended)

**Fastest way to get started:**

```bash
# Clone repository
git clone https://github.com/fabohax/BitVMX-Z.git
cd BitVMX-Z

# Start everything
./scripts/start.sh
```

**Access the application:**
- Frontend: http://localhost:5173
- Backend API: http://localhost:3000
- Health Check: http://localhost:3000/health

**Stop when done:**
```bash
./scripts/stop.sh
```

## Option 2: Local Development

**For active development with hot reload:**

```bash
# Clone repository
git clone https://github.com/fabohax/BitVMX-Z.git
cd BitVMX-Z

# Start dev servers
./scripts/dev.sh
```

This starts both backend and frontend with auto-reload on code changes.

## Option 3: Manual Setup

**For granular control:**

### Backend

```bash
# Build Rust workspace
cargo build

# Run backend node
cargo run --bin bitvmx-z-node

# In another terminal, test the API
curl http://localhost:3000/health
```

### Frontend

```bash
# Navigate to frontend
cd frontend

# Install dependencies
npm install

# Start dev server
npm run dev
```

## Verify Installation

### Test Backend API

```bash
# Health check
curl http://localhost:3000/health

# Get rollup state
curl http://localhost:3000/state

# Submit a transaction
curl -X POST http://localhost:3000/submit \
  -H "Content-Type: application/json" \
  -d '{"from":"alice","to":"bob","amount":1000}'
```

### Test Frontend

Open http://localhost:5173 in your browser and verify:
- Dashboard loads
- Navigation works
- No console errors

## Next Steps

### Explore the Code

```
BitVMX-Z/
├── protocol/     # STARK proving system
├── backend/      # Rollup node and API
├── integration/  # Bitcoin/Taproot integration
└── frontend/     # React dashboard
```

### Run Tests

```bash
./scripts/test.sh
```

### Make Changes

1. Edit code in your favorite IDE
2. Tests run automatically (with dev mode)
3. See changes reflected immediately

### Submit a PR

1. Create a branch: `git checkout -b feature/my-feature`
2. Make changes
3. Run tests: `./scripts/test.sh`
4. Commit: `git commit -m "feat: add my feature"`
5. Push: `git push origin feature/my-feature`
6. Open PR on GitHub

## Common Issues

### Port Already in Use

```bash
# Change ports in docker-compose.yml or kill processes
lsof -ti:3000 | xargs kill -9  # Kill backend
lsof -ti:5173 | xargs kill -9  # Kill frontend
```

### Docker Build Fails

```bash
# Clean and rebuild
docker-compose down
docker system prune -a
./scripts/start.sh
```

### Rust Compilation Errors

```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build
```

### Frontend Not Loading

```bash
cd frontend
rm -rf node_modules package-lock.json
npm install
npm run dev
```

## Development Workflow

### Daily Development

```bash
# Pull latest changes
git pull origin main

# Start dev servers
./scripts/dev.sh

# Make changes...
# Tests run automatically

# Clean up when done
./scripts/clean.sh
```

### Before Submitting PR

```bash
# Run full test suite
./scripts/test.sh

# Check formatting
cargo fmt --all
cd frontend && npm run lint

# Build production
./scripts/build.sh
```

## Learning Resources

- **Architecture**: See root README.md diagram
- **API Docs**: Check backend/README.md
- **Protocol Details**: Read protocol/README.md
- **Docker Setup**: Read docs/DOCKER.md
- **Contributing**: Read CONTRIBUTING.md

## Getting Help

- 📖 Read the [full documentation](../README.md)
- 🐛 [Report bugs](https://github.com/fabohax/BitVMX-Z/issues/new?template=bug_report.yml)
- 💡 [Request features](https://github.com/fabohax/BitVMX-Z/issues/new?template=feature_request.yml)
- 💬 Open a [discussion](https://github.com/fabohax/BitVMX-Z/discussions)

## What's Next?

- [ ] Explore the dashboard at http://localhost:5173
- [ ] Try the API endpoints
- [ ] Read the [roadmap](ROADMAP.md)
- [ ] Check out [open issues](https://github.com/fabohax/BitVMX-Z/issues)
- [ ] Join the development!

---

**Welcome to BitVMX-Z! 🚀**
