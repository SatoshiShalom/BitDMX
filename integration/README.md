# Integration Layer

Taproot bridge simulator and BitVMX-inspired challenge game logic.

## Components

- **taproot.rs**: Taproot commitment and script generation
- **bridge.rs**: Bitcoin Signet bridge simulator
- **challenge.rs**: Challenge/response game logic
- **bitvm.rs**: BitVMX verification principles and compatibility

## Challenge Game Flow

1. **Commitment**: Prover posts STARK proof commitment to Bitcoin via Taproot
2. **Challenge Window**: Anyone can challenge the commitment within the period
3. **Response**: Prover responds with proof segments
4. **Resolution**: Challenge is resolved on-chain or via timeout

## Usage

```rust
use bitvmx_z_integration::{BridgeSimulator, ChallengeGame};

// Create bridge simulator
let mut bridge = BridgeSimulator::new(bitcoin::Network::Signet);

// Post commitment
let tx = bridge.post_commitment(&proof_commitment).await?;

// Initialize challenge game
let mut game = ChallengeGame::new(3600); // 1 hour challenge period

// Initiate challenge
let challenge_id = game.initiate_challenge(
    batch_id,
    "challenger_address".to_string(),
    disputed_claim,
)?;
```

## Testing

```bash
cargo test
```
