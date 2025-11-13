/// BitVMX-inspired challenge game logic
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChallengeStatus {
    Open,
    Responded,
    Resolved,
    Timeout,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Challenge {
    pub id: String,
    pub batch_id: u64,
    pub challenger: String,
    pub disputed_claim: Vec<u8>,
    pub status: ChallengeStatus,
    pub created_at: u64,
    pub response_deadline: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChallengeResponse {
    pub challenge_id: String,
    pub proof_segment: Vec<u8>,
    pub witness_data: Vec<u8>,
}

pub struct ChallengeGame {
    challenges: HashMap<String, Challenge>,
    challenge_period_secs: u64,
}

impl ChallengeGame {
    pub fn new(challenge_period_secs: u64) -> Self {
        Self {
            challenges: HashMap::new(),
            challenge_period_secs,
        }
    }

    pub fn initiate_challenge(
        &mut self,
        batch_id: u64,
        challenger: String,
        disputed_claim: Vec<u8>,
    ) -> Result<String> {
        let challenge_id = uuid::Uuid::new_v4().to_string();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();

        let challenge = Challenge {
            id: challenge_id.clone(),
            batch_id,
            challenger,
            disputed_claim,
            status: ChallengeStatus::Open,
            created_at: now,
            response_deadline: now + self.challenge_period_secs,
        };

        self.challenges.insert(challenge_id.clone(), challenge);
        Ok(challenge_id)
    }

    pub fn respond_to_challenge(&mut self, response: ChallengeResponse) -> Result<()> {
        let challenge = self.challenges
            .get_mut(&response.challenge_id)
            .ok_or_else(|| anyhow::anyhow!("Challenge not found"))?;

        if challenge.status != ChallengeStatus::Open {
            return Err(anyhow::anyhow!("Challenge is not open"));
        }

        // TODO: Verify proof segment and witness data
        challenge.status = ChallengeStatus::Responded;
        Ok(())
    }

    pub fn resolve_challenge(&mut self, challenge_id: &str, valid: bool) -> Result<()> {
        let challenge = self.challenges
            .get_mut(challenge_id)
            .ok_or_else(|| anyhow::anyhow!("Challenge not found"))?;

        challenge.status = if valid {
            ChallengeStatus::Resolved
        } else {
            ChallengeStatus::Timeout
        };

        Ok(())
    }

    pub fn get_challenge(&self, challenge_id: &str) -> Option<&Challenge> {
        self.challenges.get(challenge_id)
    }

    pub fn active_challenges(&self) -> Vec<&Challenge> {
        self.challenges
            .values()
            .filter(|c| c.status == ChallengeStatus::Open)
            .collect()
    }

    pub fn check_timeouts(&mut self) -> Result<Vec<String>> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();

        let mut timed_out = vec![];

        for (id, challenge) in self.challenges.iter_mut() {
            if challenge.status == ChallengeStatus::Open && now > challenge.response_deadline {
                challenge.status = ChallengeStatus::Timeout;
                timed_out.push(id.clone());
            }
        }

        Ok(timed_out)
    }
}
