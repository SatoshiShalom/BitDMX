/// Rollup Node Simulator
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollupState {
    pub block_height: u64,
    pub state_root: Vec<u8>,
    pub pending_transactions: Vec<Transaction>,
}

pub struct RollupNode {
    state: Arc<RwLock<RollupState>>,
}

impl RollupNode {
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(RollupState {
                block_height: 0,
                state_root: vec![],
                pending_transactions: vec![],
            })),
        }
    }

    pub async fn submit_transaction(&self, tx: Transaction) -> Result<()> {
        let mut state = self.state.write().await;
        state.pending_transactions.push(tx);
        Ok(())
    }

    pub async fn get_state(&self) -> RollupState {
        self.state.read().await.clone()
    }

    pub async fn process_batch(&self) -> Result<Vec<Transaction>> {
        let mut state = self.state.write().await;
        let batch = state.pending_transactions.drain(..).collect();
        state.block_height += 1;
        Ok(batch)
    }
}

impl Default for RollupNode {
    fn default() -> Self {
        Self::new()
    }
}
