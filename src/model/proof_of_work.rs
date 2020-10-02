#[derive(Debug, Default)]
pub struct ProofOfWork<T> {
    difficulty: String,
    proof_validator: T,
}

impl<T> ProofOfWork<T>
where
    T: ProofValidator,
{
    pub fn new(difficulty: &str, proof_validator: T) -> Self {
        ProofOfWork {
            difficulty: difficulty.to_string(),
            proof_validator,
        }
    }

    pub fn generate_algorithm_proof(&self, last_algorithm_proof: i64) -> i64 {
        let mut proposed_algorithm_proof: i64 = 0;

        while !self.proof_validator.validate(
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
        last_algorithm_proof.to_string() + proposed_algorithm_proof
    }
}

pub trait ProofValidator {
    fn validate(&self, to_validate: &str, difficulty: &str) -> bool;
}
