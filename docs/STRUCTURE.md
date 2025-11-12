# Project Structure

Complete overview of the BitVMX-Z repository structure.

## Directory Tree

```
BitVMX-Z/
├── .github/                      # GitHub configuration
│   ├── ISSUE_TEMPLATE/          # Issue templates
│   │   ├── bug_report.yml
│   │   └── feature_request.yml
│   ├── workflows/               # CI/CD workflows
│   │   ├── docker.yml           # Docker build & test
│   │   ├── frontend.yml         # Frontend CI
│   │   └── rust.yml             # Rust CI
│   └── pull_request_template.md
│
├── .project/                     # Project management
│   └── README.md
│
├── backend/                      # Rollup Node (Rust)
│   ├── src/
│   │   ├── aggregator.rs        # Proof aggregation
│   │   ├── api.rs               # REST API endpoints
│   │   ├── batch.rs             # Transaction batching
│   │   ├── lib.rs               # Library root
│   │   ├── rollup.rs            # Rollup state management
│   │   ├── storage.rs           # SQLite persistence
│   │   └── bin/
│   │       └── node.rs          # Main binary
│   ├── Cargo.toml
│   └── README.md
│
├── design/                       # UX and branding
│   ├── assets/                  # Logos, icons, mockups
│   └── README.md
│
├── docs/                         # Documentation
│   ├── CI.md                    # CI/CD guide
│   ├── DOCKER.md                # Docker setup guide
│   ├── QUICKSTART.md            # Quick start guide
│   ├── ROADMAP.md               # Project roadmap
│   └── SCRIPTS.md               # Scripts reference
│
├── frontend/                     # Dashboard (React + Vite)
│   ├── src/
│   │   ├── components/
│   │   │   └── Layout.tsx       # Main layout
│   │   ├── pages/
│   │   │   ├── Batches.tsx      # Batch explorer
│   │   │   ├── Challenges.tsx   # Challenge tracker
│   │   │   ├── Dashboard.tsx    # Main dashboard
│   │   │   └── Proofs.tsx       # Proof explorer
│   │   ├── App.tsx              # Root component
│   │   ├── index.css            # Global styles
│   │   └── main.tsx             # Entry point
│   ├── index.html
│   ├── nginx.conf               # Nginx config for Docker
│   ├── package.json
│   ├── postcss.config.js
│   ├── tailwind.config.js
│   ├── tsconfig.json
│   ├── tsconfig.node.json
│   ├── vite.config.ts
│   └── README.md
│
├── integration/                  # Bitcoin Integration (Rust)
│   ├── src/
│   │   ├── bitvm.rs             # BitVMX verification
│   │   ├── bridge.rs            # Bitcoin bridge simulator
│   │   ├── challenge.rs         # Challenge game logic
│   │   ├── lib.rs               # Library root
│   │   └── taproot.rs           # Taproot commitments
│   ├── Cargo.toml
│   └── README.md
│
├── protocol/                     # Protocol Layer (Rust)
│   ├── src/
│   │   ├── binius.rs            # Binary field arithmetic
│   │   ├── lib.rs               # Library root
│   │   ├── proof.rs             # Proof structures
│   │   ├── stark.rs             # STARK prover/verifier
│   │   └── zkvm.rs              # zkVM runtime
│   ├── Cargo.toml
│   └── README.md
│
├── research/                     # Research papers
│   ├── papers/                  # Academic papers
│   └── README.md
│
├── scripts/                      # Helper scripts
│   ├── build.sh                 # Build all components
│   ├── clean.sh                 # Clean artifacts
│   ├── dev.sh                   # Development mode
│   ├── start.sh                 # Start with Docker
│   ├── stop.sh                  # Stop services
│   └── test.sh                  # Run all tests
│
├── .gitignore                    # Git ignore rules
├── Cargo.toml                    # Rust workspace config
├── CONTRIBUTING.md               # Contribution guide
├── docker-compose.yml            # Docker orchestration
├── Dockerfile.backend            # Backend container
├── Dockerfile.frontend           # Frontend container
├── LICENSE                       # MIT license
├── README.md                     # Main documentation
└── SECURITY.md                   # Security policy
```

## Core Components

### Protocol Layer (`protocol/`)
**Language**: Rust  
**Purpose**: STARK proving system and zkVM runtime

- `binius.rs` - Binary field arithmetic for efficient STARKs
- `stark.rs` - STARK prover and verifier implementation
- `zkvm.rs` - Zero-knowledge virtual machine runtime
- `proof.rs` - Proof structures and commitments

### Backend Layer (`backend/`)
**Language**: Rust  
**Purpose**: Rollup node with REST API

- `rollup.rs` - State management and transaction processing
- `batch.rs` - Transaction batching logic
- `aggregator.rs` - Recursive proof aggregation
- `api.rs` - REST API endpoints (Axum)
- `storage.rs` - SQLite persistence
- `bin/node.rs` - Main executable

### Integration Layer (`integration/`)
**Language**: Rust  
**Purpose**: Bitcoin Signet integration and challenge games

- `taproot.rs` - Taproot commitment generation
- `bridge.rs` - Bitcoin bridge simulator
- `challenge.rs` - Challenge/response game logic
- `bitvm.rs` - BitVMX verification principles

### Frontend Layer (`frontend/`)
**Language**: TypeScript + React  
**Purpose**: Proof explorer dashboard

- `components/Layout.tsx` - Navigation and layout
- `pages/Dashboard.tsx` - Overview and statistics
- `pages/Batches.tsx` - Transaction batch explorer
- `pages/Proofs.tsx` - STARK proof viewer
- `pages/Challenges.tsx` - Challenge tracker

## Configuration Files

### Rust Workspace
- `Cargo.toml` (root) - Workspace configuration
- `*/Cargo.toml` - Crate-specific dependencies

### Frontend
- `package.json` - npm dependencies
- `vite.config.ts` - Vite build config
- `tsconfig.json` - TypeScript config
- `tailwind.config.js` - Tailwind CSS config

### Docker
- `Dockerfile.backend` - Backend container build
- `Dockerfile.frontend` - Frontend container build
- `docker-compose.yml` - Multi-container orchestration

### CI/CD
- `.github/workflows/rust.yml` - Rust testing and building
- `.github/workflows/frontend.yml` - Frontend testing and building
- `.github/workflows/docker.yml` - Docker image publishing

## Scripts

All scripts are in `scripts/` and are executable:

| Script | Purpose |
|--------|---------|
| `start.sh` | Start all services with Docker |
| `stop.sh` | Stop all services |
| `dev.sh` | Run in development mode with hot reload |
| `build.sh` | Build all components locally |
| `test.sh` | Run all tests |
| `clean.sh` | Remove build artifacts |

## Documentation

All documentation is in Markdown format:

| File | Description |
|------|-------------|
| `README.md` | Main project documentation |
| `docs/QUICKSTART.md` | Quick start guide |
| `docs/DOCKER.md` | Docker setup guide |
| `docs/SCRIPTS.md` | Scripts reference |
| `docs/CI.md` | CI/CD documentation |
| `docs/ROADMAP.md` | Project roadmap |
| `CONTRIBUTING.md` | Contribution guidelines |
| `SECURITY.md` | Security policy |

## Key Technologies

### Backend
- **Rust** 1.75+ - Systems programming language
- **Axum** - Web framework
- **Tokio** - Async runtime
- **SQLx** - Database access
- **Serde** - Serialization

### Frontend
- **React** 18 - UI framework
- **TypeScript** - Type-safe JavaScript
- **Vite** - Build tool
- **Tailwind CSS** - Styling
- **Axios** - HTTP client

### Infrastructure
- **Docker** - Containerization
- **GitHub Actions** - CI/CD
- **Nginx** - Web server

## File Counts

```bash
# Count by type
Rust files:      ~15
TypeScript files: ~8
Config files:    ~12
Scripts:         6
Docs:           ~12
```

## Lines of Code (Estimated)

```
Protocol:      ~500 LOC
Backend:       ~800 LOC
Integration:   ~600 LOC
Frontend:      ~400 LOC
Total:        ~2300 LOC (excluding tests and docs)
```

## Getting Started

1. **Read**: [QUICKSTART.md](QUICKSTART.md)
2. **Explore**: Start with `README.md` in each directory
3. **Develop**: Use `./scripts/dev.sh`
4. **Contribute**: Read [CONTRIBUTING.md](../CONTRIBUTING.md)

## Navigation Tips

- Each component has its own `README.md`
- Scripts are self-documenting (check source for details)
- CI workflows have inline comments
- Docker configs include build optimization

---

*This structure enables modular development while maintaining clear separation of concerns.*
