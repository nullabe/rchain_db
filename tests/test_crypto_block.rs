#[cfg(test)]
pub mod test_crypto_block {
    use std::borrow::Cow;

    use rchain_db::model::block::{Block, BlockHasher};
    use rchain_db::crypto::block::Sha256BlockHasher;
    use rchain_db::model::transaction::Transaction;

    #[test]
    fn test_hash() {
        let block_hasher = Sha256BlockHasher;

        let block = Block::new::<Sha256BlockHasher>(
            2,
            vec![Transaction::new("bla", "bla", 1.0)],
            5,
            Some("hey".to_string()),
            Cow::Owned(Sha256BlockHasher),
        );

        assert_eq!("06400a15cf462af497c718ab7daf7b4ae1cfee1903246d8be4d302c475a3b9ea", block_hasher.hash(&block).unwrap())
    }
}
