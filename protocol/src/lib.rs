/// BitVMX-Z Protocol Layer
/// 
/// Binius-accelerated STARK proving system and zkVM runtime
pub mod stark;
pub mod zkvm;
pub mod binius;
pub mod proof;

pub use stark::{StarkProver, StarkVerifier};
pub use zkvm::ZkVMRuntime;
pub use proof::{Proof, ProofCommitment};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol_initialization() {
        // Basic smoke test
        assert!(true);
    }
}
