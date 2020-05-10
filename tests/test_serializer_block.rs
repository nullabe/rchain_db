#[cfg(test)]
pub mod test_serializer_block {
    use serde::Serialize;
    use serde_json::value::Serializer;

    use rchain_db::model::block::{Block, BlockHasher};
    use rchain_db::model::transaction::Transaction;
    use std::borrow::Cow;

    #[test]
    fn test_serialize() {
        let mut transactions = Vec::new();

        transactions.push(Transaction::new("s1", "r1", 66.6));
        transactions.push(Transaction::new("s2", "r2", 42.42));

        let block = Block::new::<BlockHasherMock>(
            6,
            transactions,
            90838340972,
            Some(String::from("previous")),
            Cow::Owned(BlockHasherMock),
        );

        assert_eq!(
            String::from("{\"algorithm_proof\":90838340972,\"hash\":\"9a2b892c228648282c915af64b3eb85b34d40853ec1c11b07968e370b2f23bc3\",\"index\":6,\"previous_block_hash\":\"previous\",\"transactions\":[{\"amount\":66.6,\"receiver\":\"r1\",\"sender\":\"s1\"},{\"amount\":42.42,\"receiver\":\"r2\",\"sender\":\"s2\"}]}"),
            block.serialize(Serializer).unwrap().to_string()
        );
    }

    #[test]
    fn test_deserialize() {
        let block: Block =
            serde_json::from_str("{\"algorithm_proof\":90838340972,\"hash\":\"9a2b892c228648282c915af64b3eb85b34d40853ec1c11b07968e370b2f23bc3\",\"index\":6,\"previous_block_hash\":\"previous\",\"transactions\":[{\"amount\":66.6,\"receiver\":\"r1\",\"sender\":\"s1\"},{\"amount\":42.42,\"receiver\":\"r2\",\"sender\":\"s2\"}]}")
                .unwrap();

        assert_eq!(2, block.transactions().len());
        assert_eq!(6, block.index());
        assert_eq!(90838340972, block.algorithm_proof());
        assert_eq!("previous", block.previous_block_hash().unwrap());
        assert_eq!(
            "9a2b892c228648282c915af64b3eb85b34d40853ec1c11b07968e370b2f23bc3",
            block.hash().unwrap()
        );
    }

    #[derive(Clone)]
    struct BlockHasherMock;

    impl BlockHasher for BlockHasherMock {
        fn hash(&self, _block: &Block) -> Option<String> {
            Some("9a2b892c228648282c915af64b3eb85b34d40853ec1c11b07968e370b2f23bc3".to_string())
        }
    }
}
