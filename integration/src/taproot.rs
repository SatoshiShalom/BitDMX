/// Taproot commitment and script generation
use bitcoin::taproot::{TaprootBuilder, TaprootSpendInfo};
use bitcoin::{Address, Network, ScriptBuf, XOnlyPublicKey};
use anyhow::Result;
use sha2::{Sha256, Digest};

pub struct TaprootCommitment {
    pub commitment_hash: [u8; 32],
    pub spend_info: Option<TaprootSpendInfo>,
}

impl TaprootCommitment {
    pub fn new(proof_commitment: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(proof_commitment);
        let commitment_hash = hasher.finalize().into();

        Self {
            commitment_hash,
            spend_info: None,
        }
    }

    pub fn create_taproot_script(&self) -> Result<ScriptBuf> {
        // TODO: Create actual Taproot script with commitment
        Ok(ScriptBuf::new())
    }

    pub fn build_taproot_address(&mut self, internal_key: XOnlyPublicKey) -> Result<Address> {
        // TODO: Build proper Taproot address with commitment script
        let builder = TaprootBuilder::new();
        
        // Placeholder - needs actual implementation
        let spend_info = builder.finalize(&bitcoin::secp256k1::Secp256k1::new(), internal_key)
            .map_err(|e| anyhow::anyhow!("Failed to finalize taproot: {:?}", e))?;
        
        self.spend_info = Some(spend_info.clone());
        
        Ok(Address::p2tr_tweaked(
            spend_info.output_key(),
            Network::Signet,
        ))
    }

    pub fn verify_commitment(&self, proof_data: &[u8]) -> bool {
        let mut hasher = Sha256::new();
        hasher.update(proof_data);
        let computed: [u8; 32] = hasher.finalize().into();
        computed == self.commitment_hash
    }
}
