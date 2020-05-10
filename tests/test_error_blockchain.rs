#[cfg(test)]
pub mod test_error_blockchain {
    use rchain_db::error::blockchain::AddBlockToBlockchainError;

    #[test]
    fn test_message() {
        let error = AddBlockToBlockchainError::new("coucou");

        assert_eq!("coucou", error.message());
    }
}
