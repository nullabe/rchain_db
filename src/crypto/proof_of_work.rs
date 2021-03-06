use crypto::digest::Digest;
use crypto::sha2::Sha256;

use crate::model::proof_of_work::ProofValidator;

pub struct Sha256ProofValidator;

impl ProofValidator for Sha256ProofValidator {
    fn validate(&self, to_validate: &str, difficulty: &str) -> bool {
        let mut hasher = Sha256::new();

        hasher.input_str(&to_validate);

        let hash = hasher.result_str();

        hash.ends_with(difficulty)
    }
}
