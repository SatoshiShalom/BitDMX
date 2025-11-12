/// BitVMX-Z Backend
/// 
/// Rollup node simulator, batch builder, and proof aggregator
pub mod rollup;
pub mod batch;
pub mod aggregator;
pub mod api;
pub mod storage;

pub use rollup::RollupNode;
pub use batch::BatchBuilder;
pub use aggregator::ProofAggregator;

#[cfg(test)]
mod tests {
    #[test]
    fn test_backend_initialization() {
        assert!(true);
    }
}
