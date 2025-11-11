# ⚡ BitVMX-Z

**Bitcoin-Native Zero-Knowledge Rollup using STARKs (Binius) and BitVMX Verification Principles**

---

## 🌍 Overview

**BitVMX-Z** is a Bitcoin-only zk-rollup prototype that integrates **Binius-accelerated STARK proofs** with **BitVMX-style challenge games**.
It demonstrates how scalable computation and state validity can be proven and verified **directly on Bitcoin**, without sidechains, forks, or EVM dependencies.

BitVMX-Z serves as a **research and hackathon prototype** for exploring trustless Bitcoin scalability through **deterministic proof verification** and **Taproot-based commitments**.

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
4. Enables off-chain **challenge/response verification** using **BitVMX** challenge logic.

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

---

## ⚙️ Development Setup

**Prerequisites**

* Rust ≥ 1.75
* Node.js ≥ 20
* Bitcoin Core (Signet)
* Docker (optional for local testnet)

**Build Instructions**

```bash
# Clone repository
git clone https://github.com/fabohax/bitvmx-z.git
cd bitvmx-z

# Install dependencies
cargo build
npm install --prefix frontend

# Run rollup node
cargo run --bin bitvmx-z-node
```

---

## 🧬 Research Focus

* Integrating **BitVMX dispute logic** with deterministic STARK proof roots.
* Measuring **Binius prover performance** vs standard STARKs.
* Evaluating Taproot commitment efficiency for rollup anchoring.

---

## 📜 License

MIT License © 2025 Satoshi’s Bride AI Labs & FABOHAX

---

> **BitVMX-Z — Scaling Bitcoin with deterministic zero-knowledge.**