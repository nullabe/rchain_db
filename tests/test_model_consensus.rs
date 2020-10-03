#[cfg(test)]
pub mod test_model_consensus {
    use rchain_db::model::block::{Block, BlockHasher};
    use rchain_db::model::blockchain::Blockchain;
    use rchain_db::model::consensus;
    use rchain_db::model::proof_of_work::ProofValidator;

    #[test]
    fn test_consensus() -> Result<(), String> {
        let mut blockchain = Blockchain::new(ProofValidatorMock, BlockHasherMock);

        match blockchain.add_block("test") {
            Err(err) => {
                return Err(err.message().clone());
            }

            _ => (),
        }

        assert_eq!(true, consensus::is_blockchain_authoritative(&blockchain, 0));

        Ok(())
    }

    struct ProofValidatorMock;

    impl ProofValidator for ProofValidatorMock {
        fn validate(&self, _to_validate: &str, _difficulty: &str) -> bool {
            true
        }
    }

    #[derive(Clone)]
    struct BlockHasherMock;

    impl BlockHasher for BlockHasherMock {
        fn hash(&self, _block: &Block) -> Option<String> {
            Some("9a2b892c228648282c915af64b3eb85b34d40853ec1c11b07968e370b2f23bc3".to_string())
        }
    }
}
