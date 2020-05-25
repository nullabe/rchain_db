#[cfg(test)]
pub mod test_model_block {
    use std::borrow::Cow;

    use rchain_db::model::block::{Block, BlockHasher};
    use rchain_db::model::transaction::Transaction;

    #[test]
    fn test_get_hash() {
        let block = get_new_block();

        assert_eq!(
            "9a2b892c228648282c915af64b3eb85b34d40853ec1c11b07968e370b2f23bc3",
            block.hash().unwrap()
        );
    }

    #[test]
    fn test_get_index() {
        let block = get_new_block();

        assert_eq!(2, block.index());
    }

    #[test]
    fn test_get_transactions() {
        let block = get_new_block();
        let transactions = block.transactions();

        assert_eq!(1, transactions.len());
        assert_eq!("bla", transactions.first().unwrap().receiver());
        assert_eq!("bla", transactions.first().unwrap().sender());
        assert_eq!(1.0, transactions.first().unwrap().amount());
    }

    #[test]
    fn test_get_algorithm_proof() {
        let block = get_new_block();

        assert_eq!(5, block.algorithm_proof());
    }

    #[test]
    fn test_get_previous_block_hash() {
        let block = get_new_block();

        assert_eq!("hey", block.previous_block_hash().unwrap());
    }

    #[test]
    fn test_get_timestamp() {
        let block = get_new_block();

        assert_eq!(true, block.timestamp().is_sign_positive());
    }

    fn get_new_block() -> Block {
        Block::new::<BlockHasherMock>(
            2,
            vec![Transaction::new("bla", "bla", 1.0)],
            5,
            Some("hey".to_string()),
            Cow::Owned(BlockHasherMock),
        )
    }

    #[derive(Clone)]
    struct BlockHasherMock;

    impl BlockHasher for BlockHasherMock {
        fn hash(&self, _block: &Block) -> Option<String> {
            Some("9a2b892c228648282c915af64b3eb85b34d40853ec1c11b07968e370b2f23bc3".to_string())
        }
    }
}
