#[cfg(test)]
pub mod test_serializer_blockchain {
    use serde::Serialize;
    use serde_json::value::Serializer;

    use rchain_db::crypto::block::Sha256BlockHasher;
    use rchain_db::crypto::proof_of_work::Sha256ProofValidator;
    use rchain_db::model::block::{Block, BlockHasher};
    use rchain_db::model::blockchain::Blockchain;
    use rchain_db::model::proof_of_work::ProofValidator;

    #[test]
    fn test_serialize() {
        let mut blockchain = Blockchain::new(ProofValidatorMock, BlockHasherMock);

        blockchain.add_new_transaction("s1", "r1", 66.6);
        blockchain.add_new_block("test").ok();

        assert_eq!(
            String::from("{\"blocks\":[{\"algorithm_proof\":0,\"hash\":\"9a2b892c228648282c915af64b3eb85b34d40853ec1c11b07968e370b2f23bc3\",\"index\":0,\"previous_block_hash\":\"\",\"transactions\":[{\"amount\":66.6,\"receiver\":\"r1\",\"sender\":\"s1\"}]}],\"transactions_to_process\":[{\"amount\":1.0,\"receiver\":\"test\",\"sender\":\"0\"}]}"),
            blockchain.serialize(Serializer).unwrap().to_string()
        );
    }

    #[test]
    fn test_deserialize() {
        let blockchain: Blockchain<Sha256ProofValidator, Sha256BlockHasher> = serde_json::from_str("{\"blocks\":[{\"algorithm_proof\":0,\"hash\":\"9a2b892c228648282c915af64b3eb85b34d40853ec1c11b07968e370b2f23bc3\",\"index\":0,\"previous_block_hash\":\"\",\"transactions\":[{\"amount\":66.6,\"receiver\":\"r1\",\"sender\":\"s1\"}]}],\"transactions_to_process\":[{\"amount\":66.6,\"receiver\":\"r1\",\"sender\":\"s1\"}]}").unwrap();

        assert_eq!(1, blockchain.blocks().len());
        assert_eq!(1, blockchain.transactions_to_process().len());
        assert_eq!(1, blockchain.last_block().unwrap().transactions().len());
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
