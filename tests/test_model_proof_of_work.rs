#[cfg(test)]
pub mod test_model_proof_of_work {
    use rchain_db::model::proof_of_work::{ProofOfWork, ProofValidator};

    #[test]
    fn test_generate() {
        let proof_of_work = ProofOfWork::new("000", ProofValidatorMock);

        assert_eq!(0, proof_of_work.generate(1))
    }

    pub struct ProofValidatorMock;

    impl ProofValidator for ProofValidatorMock {
        fn validate(&self, _to_validate: &str, _difficulty: &str) -> bool {
            true
        }
    }
}
