# 🌀 BitVMX-Z

**Zero-Knowledge Extension of BitVMX — STARK-powered Rollup Prototype for Bitcoin and Rootstock**

---

## 🌍 Overview

**BitVMX-Z** is a prototype demonstrating how **STARK proofs** and **BitVMX-style verification games** can power scalable Bitcoin applications.
It explores a hybrid model where **Rootstock (RSK)** acts as an **execution layer**, while **Bitcoin** remains the **settlement and dispute layer**.

The system simulates a **zk-Rollup bridge** between Bitcoin and Rootstock using:

* **STARK-style proofs** for validity
* **Taproot-based commitments** for finality
* **BitVMX challenge logic** for trustless verification

This repository (`bitvmx-z`) implements a **Bridge Simulator** that visualizes transactions, proofs, and challenge-response flows between the two chains.

---

## 🧩 Prototype Components

| Layer           | Description                                                          |
| --------------- | -------------------------------------------------------------------- |
| **Protocol**    | Mock STARK proof engine (Binius-accelerated) and zkVM runtime        |
| **Backend**     | Rollup node simulator and challenge handler                          |
| **Integration** | Bitcoin–Rootstock bridge logic (Taproot commitments + RSK contracts) |
| **Frontend**    | React dashboard showing deposits, proofs, and challenges             |
| **Design**      | Visual branding, diagrams, and investor-facing UI                    |
| **Research**    | Notes on integrating Rootstock VM and BitVMX verification            |

---

## 🚀 Architecture Overview

```
 ┌────────────────────────────┐
 │       Bitcoin Layer        │
 │ Taproot + BitVMX Scripts   │
 └───────────┬────────────────┘
             │
   Commitments (STARK Roots)
             │
 ┌───────────┴────────────────┐
 │       BitVMX-Z Rollup      │
 │ - zkVM Runtime (Binius)    │
 │ - STARK Proof Simulator    │
 │ - Challenge Engine         │
 └───────────┬────────────────┘
             │
   REST + JSON-RPC APIs
             │
 ┌───────────┴────────────────┐
 │ Rootstock Execution Layer  │
 │ RSK Smart Contracts + VM   │
 └───────────┬────────────────┘
             │
 ┌───────────┴────────────────┐
 │   Web Portal & Explorer    │
 │   Proofs / Disputes / UX   │
 └────────────────────────────┘
```

---

## 🧪 Rootstock Prototype

The **BitVMX-Z Bridge Simulator** connects to a local **Rootstock test node** to emulate asset transfers:

1. Users deposit BTC → represented as wrapped tokens on RSK.
2. Transactions are rolled up and “proven” using mock STARK data.
3. Commitments (STARK roots) are published to Bitcoin via Taproot.
4. Disputes are simulated using BitVMX interactive proofs.

This prototype demonstrates **how Bitcoin can host zk-verified rollups** while Rootstock executes complex smart-contract logic.

---

## 📦 Repository Structure

```
bitvmx-z/
├── protocol/      # STARK & zkVM simulator
├── backend/       # Rollup node and challenge logic
├── integration/   # Rootstock bridge and Taproot commitment templates
├── frontend/      # React dashboard and explorer
├── design/        # Branding and UX assets
├── research/      # Rootstock + BitVMX integration notes
└── .project/      # Kanban JSONs for GitHub Projects
```

---

## 🧠 Development Setup

**Prerequisites**

* Rust ≥ 1.75
* Node.js ≥ 20
* Bitcoin Core (Signet)
* Rootstock node or Ganache fork
* Docker (optional for orchestration)

**Run locally**

```bash
git clone https://github.com/satoshisbrideai/bitvmx-z.git
cd bitvmx-z
cargo build
npm install --prefix frontend
cargo run --bin bitvmx-z-node
```

---

## 🧭 Contributing

1. Fork the repo
2. Create a feature branch
3. Submit PR with coverage
4. Tag issues with `frontend`, `backend`, `integration`, `protocol`, or `research`

---

## 🧱 Governance & Security

* Branches: `main`, `dev`, `feature/*`
* Audits: community-led before hackathon submission
* Reports: `security@bitvmx.org`

---

## 📜 License

MIT License © 2025 Satoshi’s Bride AI Labs & FABOHAX

⚡ **BitVMX-Z — Bringing STARKs and Rootstock to Bitcoin’s Base Layer**
