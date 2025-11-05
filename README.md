# 🌀 BitDMX Core  
**Next-generation Bitcoin zk-Rollup powered by STARKs and BitVMX principles**

---

## 🌍 Overview
BitDMX is a Bitcoin-native zero-knowledge rollup framework designed to extend Bitcoin’s scalability and programmability — **without relying on Ethereum**.  
It leverages **STARK proofs** for transparent, quantum-resistant verification and builds its bridge directly on Bitcoin using **Taproot scripts and BitVMX-style challenge games**.

This repository (`bitdmx-core`) contains the core runtime, proof engine, and integration layer for the BitDMX ecosystem.

---

## 🧩 Key Components

| Layer | Description |
|-------|--------------|
| **Protocol** | STARK proof engine, zkVM runtime, recursive proof compression |
| **Backend** | Rollup node, state tree management, proof aggregation daemon |
| **Integration** | Taproot bridge templates, UTXO tracking, and dispute protocol |
| **Frontend** | Web portal and explorer for proofs, transactions, and stats |
| **Design** | Brand identity, UX flows, and investor dashboard visualization |
| **Research** | Cryptographic analysis, performance benchmarking, and protocol review |

---

## 🚀 Architecture Overview

     ┌────────────────────────────┐
     │      Bitcoin Layer         │
     │  Taproot + BitVMX Scripts  │
     └───────────┬────────────────┘
                 │
       Commitments (STARK Roots)
                 │
     ┌───────────┴────────────────┐
     │       BitDMX Rollup        │
     │ - zkVM Runtime (Binius)    │
     │ - Recursive STARKs         │
     │ - Proof Aggregator         │
     └───────────┬────────────────┘
                 │
       REST + JSON-RPC APIs
                 │
     ┌───────────┴────────────────┐
     │  Web Portal + Explorer     │
     │  User, Dev, & Investor UI  │
     └────────────────────────────┘

---

## 📦 Repository Structure
bitdmx-core/
├── protocol/ # STARK engine and zkVM runtime
├── backend/ # Rollup node and aggregator
├── integration/ # Taproot bridge modules
├── frontend/ # React web portal
├── design/ # UI/UX, brand assets
├── research/ # Papers, benchmarks, metrics
└── .project/ # GitHub Project JSONs for Kanban

---

## 🗂️ Project Management

We use **GitHub Projects (Kanban)** for cross-team coordination.  
Each department maintains its own task board using JSON import files stored under `.project/`.

**Columns:**  
`Backlog → In Progress → Review → Done → Research`

**Departments:**  
`frontend`, `backend`, `integration`, `protocol`, `design`, `cto`, `research`

You can import the boards with:
```bash
gh project item-add --project "BitDMX Core" --body "$(cat .project/frontend.json)"

| Month     | Focus          | Deliverables                            |
| --------- | -------------- | --------------------------------------- |
| **1**     | Foundation     | Repo setup, AIR framework, branding     |
| **2**     | STARK Engine   | Prover prototype, zkVM spec             |
| **3**     | Bridge Alpha   | Taproot deposit contracts, UTXO tracker |
| **4–6**   | zkVM + Testnet | Recursive STARKs, frontend explorer     |
| **7–8**   | Optimization   | Performance tuning, audit planning      |
| **9–10**  | SDK + APIs     | Developer tools, verifier incentives    |
| **11–12** | Mainnet Launch | Public release, investor dashboard      |
🧠 Development Setup
Prerequisites

Rust ≥ 1.75

Node.js ≥ 20

Bitcoin Core (Signet)

Docker (optional for local orchestration)

**Build Instructions
**
# Clone repository
git clone https://github.com/satoshisbrideai/bitdmx-core.git
cd bitdmx-core

# Install dependencies
cargo build
npm install --prefix frontend

# Run local node
cargo run --bin bitdmx-node

🧪 Testnet Access

Network: DMX Testnet 0.1

Bridge: tb1qd... (Taproot Signet address)

Explorer: https://explorer.bitdmx.org

🧭 Contributing

Fork the repo

Create a feature branch

Submit a PR for review

Include test coverage and lint checks

Tag your issues with:

frontend, backend, protocol, integration, design, cto, or research

👁️‍🗨️ Governance & Security

Branching model: main (stable), dev (integration), feature/* (experimental)

Audits: Coordinated by CTO and external partners (Month 8 milestone)

Security reports: security@bitdmx.org

📰 Investor & Community Updates

Monthly investor newsletters summarize milestone progress:

Proof system maturity (protocol)

Rollup node reliability (backend)

Bridge audit readiness (integration)

Design and branding updates (UX)

Financial runway and contributors (management)

Subscribe at bitdmx.org/newsletter
.

📜 License

MIT License © 2025 Satoshi’s Bride AI Labs

⚡ BitDMX — Scaling Bitcoin with transparent zero-knowledge.
