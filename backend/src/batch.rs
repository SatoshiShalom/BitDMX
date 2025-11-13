/// Batch Builder for transaction batching
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Batch {
    pub id: u64,
    pub transactions: Vec<crate::rollup::Transaction>,
    pub state_root: [u8; 32],
    pub timestamp: u64,
}

pub struct BatchBuilder {
    max_batch_size: usize,
    batch_timeout_secs: u64,
}

impl BatchBuilder {
    pub fn new(max_batch_size: usize, batch_timeout_secs: u64) -> Self {
        Self {
            max_batch_size,
            batch_timeout_secs,
        }
    }

    pub fn build_batch(&self, transactions: Vec<crate::rollup::Transaction>, batch_id: u64) -> Result<Batch> {
        let state_root = self.compute_state_root(&transactions);
        
        Ok(Batch {
            id: batch_id,
            transactions,
            state_root,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs(),
        })
    }

    fn compute_state_root(&self, transactions: &[crate::rollup::Transaction]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        for tx in transactions {
            hasher.update(tx.id.as_bytes());
        }
        hasher.finalize().into()
    }

    pub fn should_create_batch(&self, pending_count: usize, last_batch_time: u64) -> bool {
        pending_count >= self.max_batch_size || 
        (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - last_batch_time) >= self.batch_timeout_secs
    }
}
