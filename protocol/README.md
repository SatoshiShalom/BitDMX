# Protocol Layer

Binius-accelerated STARK proving system and zkVM runtime for BitVMX-Z.

## Components

- **stark.rs**: STARK prover and verifier implementation
- **zkvm.rs**: zkVM runtime simulator for execution traces
- **binius.rs**: Binary field arithmetic for efficient STARK computations
- **proof.rs**: Proof structures and commitment handling

## Usage

```rust
use bitvmx_z_protocol::{StarkProver, ZkVMRuntime};

// Create zkVM instance
let mut vm = ZkVMRuntime::new();

// Execute program and generate trace
let trace = vm.execute(program)?;

// Generate STARK proof
let prover = StarkProver::new(128);
let proof = prover.prove(&trace)?;
```

## Development

```bash
cargo build
cargo test
cargo bench
```
