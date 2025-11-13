/// Proof Aggregator for recursive proof compression
use anyhow::Result;
use bitvmx_z_protocol::{Proof, StarkProver};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedProof {
    pub batches: Vec<u64>,
    pub proof: Vec<u8>,
    pub commitment: [u8; 32],
}

pub struct ProofAggregator {
    prover: StarkProver,
    pending_proofs: Vec<Proof>,
}

impl ProofAggregator {
    pub fn new() -> Self {
        Self {
            prover: StarkProver::new(128),
            pending_proofs: vec![],
        }
    }

    pub fn add_proof(&mut self, proof: Proof) {
        self.pending_proofs.push(proof);
    }

    pub async fn aggregate(&mut self) -> Result<AggregatedProof> {
        // TODO: Implement recursive proof aggregation
        let batches: Vec<u64> = self.pending_proofs
            .iter()
            .map(|p| p.metadata.batch_id)
            .collect();

        let aggregated = AggregatedProof {
            batches,
            proof: vec![],
            commitment: [0u8; 32],
        };

        self.pending_proofs.clear();
        Ok(aggregated)
    }

    pub fn pending_count(&self) -> usize {
        self.pending_proofs.len()
    }
}

impl Default for ProofAggregator {
    fn default() -> Self {
        Self::new()
    }
}
