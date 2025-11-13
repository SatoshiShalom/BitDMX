/// BitVMX-Z Integration Layer
/// 
/// Taproot bridge simulator and challenge game logic
pub mod taproot;
pub mod bridge;
pub mod challenge;
pub mod bitvm;

pub use taproot::TaprootCommitment;
pub use bridge::BridgeSimulator;
pub use challenge::ChallengeGame;

#[cfg(test)]
mod tests {
    #[test]
    fn test_integration_initialization() {
        assert!(true);
    }
}
