# ⚡ BitVMX-Z

**Bitcoin-Native Zero-Knowledge Rollup using STARKs (Binius) and BitVMX Verification Principles**

[![Rust CI](https://github.com/fabohax/BitVMX-Z/actions/workflows/rust.yml/badge.svg)](https://github.com/fabohax/BitVMX-Z/actions/workflows/rust.yml)
[![Frontend CI](https://github.com/fabohax/BitVMX-Z/actions/workflows/frontend.yml/badge.svg)](https://github.com/fabohax/BitVMX-Z/actions/workflows/frontend.yml)
[![Docker CI](https://github.com/fabohax/BitVMX-Z/actions/workflows/docker.yml/badge.svg)](https://github.com/fabohax/BitVMX-Z/actions/workflows/docker.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

---

## 🌍 Overview

**BitVMX-Z** is a Bitcoin-only zk-rollup prototype that integrates **Binius-accelerated STARK proofs** with **BitVMX-style challenge games**.
It demonstrates how scalable computation and state validity can be proven and verified **directly on Bitcoin**.

BitVMX-Z serves as a **research and hackathon prototype** for exploring trustless Bitcoin scalability through **deterministic proof verification** and **Taproot-based commitments**.

**[📖 Quick Start Guide](docs/QUICKSTART.md)** | **[🗺️ Roadmap](docs/ROADMAP.md)** | **[🤝 Contributing](CONTRIBUTING.md)**

---

## 🧩 Key Components

| Layer           | Description                                                     |
| --------------- | --------------------------------------------------------------- |
| **Protocol**    | STARK prover and verifier (Binius), recursive proof compression |
| **Backend**     | Rollup node simulator, batch builder, and proof aggregator      |
| **Integration** | Taproot bridge simulator and dispute challenge logic            |
| **Frontend**    | Proof explorer and challenge dashboard (React + Vite)           |
| **Design**      | Clean UX for visualizing proofs and disputes                    |
| **Research**    | BitVMX compatibility study and performance benchmarks           |

---

## 🚀 Architecture Overview

```
     ┌────────────────────────────┐
     │       Bitcoin Layer        │
     │  Taproot + BitVMX Scripts  │
     └───────────┬────────────────┘
                 │
      STARK Root Commitments
                 │
     ┌───────────┴────────────────┐
     │       BitVMX-Z Node        │
     │ - Binius STARK Engine      │
     │ - Proof Aggregator         │
     │ - zkVM Runtime Simulator   │
     └───────────┬────────────────┘
                 │
        REST + CLI + WebSocket APIs
                 │
     ┌───────────┴────────────────┐
     │   Proof Explorer (Web)     │
     │   - Rollup Batches         │
     │   - STARK Proofs           │
     │   - Challenges             │
     └────────────────────────────┘
```

---

## 🧠 Prototype Concept

What BitVMX-Z does:

1. Simulates batched Bitcoin transactions in a rollup node.
2. Generates **STARK proofs** of correct state transitions using **Binius**.
3. Posts proof commitments (hashes) to **Bitcoin Signet via Taproot scripts**.
4. Enables off-chain **challenge/response verification** inspired by **BitVMX**.

The prototype validates the idea of **deterministic verification via cryptographic proofs** on Bitcoin.

---

## 📦 Repository Structure

```
bitvmx-z/
├── protocol/        # Binius STARK engine + zkVM runtime
├── backend/         # Rollup simulator + aggregator
├── integration/     # Taproot bridge simulator + challenge game
├── frontend/        # Proof explorer dashboard (React + Vite)
├── design/          # Branding, UX, dashboard layout
├── research/        # Papers, benchmarks, BitVMX analysis
└── .project/        # GitHub Project JSONs for Kanban
```

## ⚙️ Development Setup

**Prerequisites**

* Rust ≥ 1.75
* Node.js ≥ 20
* Docker & Docker Compose
* Bitcoin Core (Signet) - optional for testing

### 🐳 Quick Start with Docker

```bash
# Clone repository
git clone https://github.com/fabohax/bitvmx-z.git
cd bitvmx-z

# Start all services
./scripts/start.sh

# Access:
# Backend API: http://localhost:3000
# Frontend: http://localhost:5173

# Stop services
./scripts/stop.sh
```

### 💻 Local Development

```bash
# Build everything
./scripts/build.sh

# Run in development mode (auto-reload)
./scripts/dev.sh

# Run tests
./scripts/test.sh

# Clean build artifacts
./scripts/clean.sh
```

### Manual Setup

```bash
# Build Rust workspace
cargo build --release

# Install frontend dependencies
cd frontend
npm install
npm run dev

# In another terminal, run backend
cargo run --bin bitvmx-z-node
```

---

## 🧪 Testnet Access

**Network:** Z-Testnet 0.1
**Bridge (Taproot):** tb1qd... (Signet)
**Explorer:** [https://bitvmx-explorer.com](https://bitvmx-explorer.com)

---

## 📚 Documentation

- [Quick Start](docs/QUICKSTART.md) - Get running in 5 minutes
- [Docker Setup](docs/DOCKER.md) - Complete Docker guide
- [Scripts Reference](docs/SCRIPTS.md) - Helper script documentation
- [CI/CD](docs/CI.md) - GitHub Actions workflows
- [Roadmap](docs/ROADMAP.md) - Project timeline and milestones
- [Contributing](CONTRIBUTING.md) - Contribution guidelines
- [Security](SECURITY.md) - Security policy and reporting

---
explorer.bitvmx.org
## 🧭 Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

Quick start:
1. Fork the repo
2. Create a feature branch
3. Submit a PR with tests and lint checks
4. Tag issues by domain: `protocol`, `backend`, `integration`, `frontend`, `design`, or `research`

---

## 🔒 Governance & Security

* Branches: `main` (stable), `dev` (integration), `feature/*` (experimental)
* Audits: scheduled for Month 8 milestone
* Security contact: **[security@bitvmx.org](mailto:security@bitvmx.org)**

---

## 🧬 Research Focus

* Integrating **BitVMX dispute logic** with deterministic STARK proof roots.
* Measuring **Binius prover performance** vs standard STARKs.
* Evaluating Taproot commitment efficiency for rollup anchoring.

---

## 📜 License

MIT License © 2025 Satoshi’s Bride AI Labs

---

> **BitVMX-Z — Scaling Bitcoin with deterministic zero-knowledge.**
