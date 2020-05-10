#[cfg(test)]
pub mod test_http_state {
    use std::sync::{Arc, Mutex};

    use rchain_db::crypto::block::Sha256BlockHasher;
    use rchain_db::crypto::proof_of_work::Sha256ProofValidator;
    use rchain_db::http::state::BlockchainState;
    use rchain_db::model::blockchain::Blockchain;

    #[test]
    fn test_blockchain() {
        let blockchain = Blockchain::new(Sha256ProofValidator, Sha256BlockHasher);
        let blockchain = Arc::new(Mutex::new(blockchain));

        let state = BlockchainState::from(blockchain, "uuid");

        assert_eq!(0, state.blockchain().lock().unwrap().blocks().len());
    }

    #[test]
    fn test_node_uuid() {
        let blockchain = Blockchain::new(Sha256ProofValidator, Sha256BlockHasher);
        let blockchain = Arc::new(Mutex::new(blockchain));

        let state = BlockchainState::from(blockchain, "uuid");

        assert_eq!("uuid", state.node_uuid());
    }
}
