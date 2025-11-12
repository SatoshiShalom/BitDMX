/// Proof structures and commitment handling
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proof {
    pub stark_proof: Vec<u8>,
    pub commitment: ProofCommitment,
    pub metadata: ProofMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofCommitment {
    pub root_hash: [u8; 32],
    pub merkle_path: Vec<Vec<u8>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofMetadata {
    pub batch_id: u64,
    pub timestamp: u64,
    pub transaction_count: u32,
}

impl Proof {
    pub fn new(stark_proof: Vec<u8>, batch_id: u64, tx_count: u32) -> Self {
        let commitment = Self::compute_commitment(&stark_proof);
        Self {
            stark_proof,
            commitment,
            metadata: ProofMetadata {
                batch_id,
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                transaction_count: tx_count,
            },
        }
    }

    fn compute_commitment(data: &[u8]) -> ProofCommitment {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let root_hash: [u8; 32] = hasher.finalize().into();
        
        ProofCommitment {
            root_hash,
            merkle_path: vec![],
        }
    }

    pub fn to_taproot_commitment(&self) -> Vec<u8> {
        // TODO: Convert proof commitment to Taproot script format
        self.commitment.root_hash.to_vec()
    }
}
