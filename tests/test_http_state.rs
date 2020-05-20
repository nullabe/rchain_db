#[cfg(test)]
pub mod test_http_state {
    use rchain_db::http::state::BlockchainState;
    use rchain_db::storage::file::blockchain::BlockchainFileStorage;

    #[test]
    fn test_blockchain() {
        let state = BlockchainState::from(BlockchainFileStorage, "uuid");

        assert_eq!(0, state.blockchain().lock().unwrap().blocks().len());
    }

    #[test]
    fn test_node_uuid() {
        let state = BlockchainState::from(BlockchainFileStorage, "uuid");

        assert_eq!("uuid", state.node_uuid());
    }
}
