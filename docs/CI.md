# Continuous Integration

GitHub Actions workflows for automated testing, building, and deployment.

## Workflows

### Rust CI (`rust.yml`)

**Triggers**: Push/PR to `main` or `dev` branches

**Jobs**:
- **test**: Runs formatting, linting, build, and tests
  - `cargo fmt --check`
  - `cargo clippy`
  - `cargo build --workspace`
  - `cargo test --workspace`
  
- **build-release**: Creates release binaries (main branch only)
  - Uploads `bitvmx-z-node` binary as artifact

**Caching**: Cargo registry, index, and build artifacts

### Frontend CI (`frontend.yml`)

**Triggers**: Push/PR to `main` or `dev` branches

**Jobs**:
- **test**: Lints and builds frontend
  - `npm ci`
  - `npm run lint`
  - `npm run build`
  - Uploads `dist/` as artifact (main branch only)

**Caching**: npm dependencies

### Docker CI (`docker.yml`)

**Triggers**: 
- Push to `main`
- Tags matching `v*`
- PRs to `main`

**Jobs**:
- **build-backend**: Builds and pushes backend Docker image
- **build-frontend**: Builds and pushes frontend Docker image
- **docker-compose-test**: Tests full stack with docker-compose

**Image Registry**: GitHub Container Registry (ghcr.io)

**Tags**:
- Branch name (e.g., `main`)
- PR number (e.g., `pr-123`)
- Semantic version (e.g., `v1.0.0`, `1.0`)
- Git SHA

## Status Badges

Add these to your README:

```markdown
[![Rust CI](https://github.com/fabohax/BitVMX-Z/actions/workflows/rust.yml/badge.svg)](https://github.com/fabohax/BitVMX-Z/actions/workflows/rust.yml)
[![Frontend CI](https://github.com/fabohax/BitVMX-Z/actions/workflows/frontend.yml/badge.svg)](https://github.com/fabohax/BitVMX-Z/actions/workflows/frontend.yml)
[![Docker CI](https://github.com/fabohax/BitVMX-Z/actions/workflows/docker.yml/badge.svg)](https://github.com/fabohax/BitVMX-Z/actions/workflows/docker.yml)
```

## Using Docker Images

Pull published images:

```bash
# Backend
docker pull ghcr.io/fabohax/bitvmx-z/backend:main

# Frontend
docker pull ghcr.io/fabohax/bitvmx-z/frontend:main

# Specific version
docker pull ghcr.io/fabohax/bitvmx-z/backend:v1.0.0
```

Update `docker-compose.yml` to use pre-built images:

```yaml
services:
  backend:
    image: ghcr.io/fabohax/bitvmx-z/backend:main
    # Remove 'build' section
```

## Running Locally

All workflows can be tested locally using [act](https://github.com/nektos/act):

```bash
# Install act
brew install act  # macOS
# or
curl https://raw.githubusercontent.com/nektos/act/master/install.sh | sudo bash

# Run Rust workflow
act -W .github/workflows/rust.yml

# Run all workflows
act push
```

## Secrets

Required secrets for full CI/CD:

- `GITHUB_TOKEN` - Automatically provided by GitHub Actions
- No additional secrets needed for current setup

Future additions might include:
- `DOCKER_HUB_TOKEN` - For Docker Hub
- `DEPLOY_KEY` - For automated deployments

## Troubleshooting

### Workflow Fails on Clippy

Fix locally:
```bash
cargo clippy --fix --all-targets --all-features
```

### Frontend Lint Errors

Fix locally:
```bash
cd frontend
npm run lint -- --fix
```

### Docker Build Fails

Test locally:
```bash
docker build -f Dockerfile.backend -t test-backend .
docker build -f Dockerfile.frontend -t test-frontend .
```

### Cache Issues

GitHub Actions caches can be cleared:
1. Go to repository Settings → Actions → Caches
2. Delete relevant caches
3. Re-run workflow

## Performance

Typical workflow times:
- **Rust CI**: 5-8 minutes (with cache)
- **Frontend CI**: 2-3 minutes (with cache)
- **Docker CI**: 8-12 minutes (with cache)

First run (no cache): 15-25 minutes per workflow

## Branch Protection

Recommended branch protection rules for `main`:

- ✅ Require pull request before merging
- ✅ Require status checks to pass:
  - `Test` (Rust CI)
  - `Test & Build` (Frontend CI)
  - `Test Docker Compose` (Docker CI)
- ✅ Require branches to be up to date
- ✅ Require linear history
