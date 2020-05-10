#[cfg(test)]
pub mod test_crypto_proof_of_work {
    use rchain_db::crypto::proof_of_work::Sha256ProofValidator;
    use rchain_db::model::proof_of_work::ProofValidator;

    #[test]
    fn test_validate() {
        let proof_validator = Sha256ProofValidator;

        assert_eq!(false, proof_validator.validate("toto", "c impossible que ça soit la fin"));
    }
}
