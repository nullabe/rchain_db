use std::ops::Add;

use crate::crypto::Validator;

#[derive(Debug, Default)]
pub struct ProofOfWork {
    difficulty: String,
}

impl ProofOfWork {
    pub fn new(difficulty: String) -> Self {
        ProofOfWork { difficulty }
    }

    pub fn generate(&self, last_algorithm_proof: i64) -> i64 {
        let mut proposed_algorithm_proof: i64 = 0;

        while !self.validate(
            &self.to_validate(
                &last_algorithm_proof.to_string(),
                &proposed_algorithm_proof.to_string(),
            ),
            &self.difficulty,
        ) {
            proposed_algorithm_proof += 1;
        }

        proposed_algorithm_proof
    }

    fn to_validate(&self, last_algorithm_proof: &str, proposed_algorithm_proof: &str) -> String {
        let mut to_validate = String::new();

        to_validate = to_validate.add(last_algorithm_proof);
        to_validate = to_validate.add(proposed_algorithm_proof);

        to_validate
    }
}
