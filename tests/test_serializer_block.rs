#[cfg(test)]
pub mod test_serializer_block {
    use std::borrow::Cow;

    use serde::Serialize;
    use serde_json::value::Serializer;

    use rchain_db::model::block::{Block, BlockHasher};
    use rchain_db::model::transaction::Transaction;

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

        let serialized_block = block.serialize(Serializer).unwrap().to_owned();

        assert_eq!(
            90838340972,
            serialized_block
                .get("algorithm_proof")
                .unwrap()
                .as_i64()
                .unwrap()
        );
        assert_eq!(
            "9a2b892c228648282c915af64b3eb85b34d40853ec1c11b07968e370b2f23bc3",
            serialized_block.get("hash").unwrap().as_str().unwrap()
        );
        assert_eq!(6, serialized_block.get("index").unwrap().as_i64().unwrap());
        assert_eq!(
            "previous",
            serialized_block
                .get("previous_block_hash")
                .unwrap()
                .as_str()
                .unwrap()
        );
        assert_eq!(
            true,
            serialized_block
                .get("timestamp")
                .unwrap()
                .as_f64()
                .unwrap()
                .is_sign_positive()
        );
        assert_eq!(
            2,
            serialized_block
                .get("transactions")
                .unwrap()
                .as_array()
                .unwrap()
                .len()
        );
    }

    #[test]
    fn test_deserialize() {
        let block: Block =
            serde_json::from_str("{\"timestamp\":55.534,\"algorithm_proof\":90838340972,\"hash\":\"9a2b892c228648282c915af64b3eb85b34d40853ec1c11b07968e370b2f23bc3\",\"index\":6,\"previous_block_hash\":\"previous\",\"transactions\":[{\"amount\":66.6,\"receiver\":\"r1\",\"sender\":\"s1\"},{\"amount\":42.42,\"receiver\":\"r2\",\"sender\":\"s2\"}]}")
                .unwrap();

        assert_eq!(2, block.transactions().len());
        assert_eq!(6, block.index());
        assert_eq!(90838340972, block.algorithm_proof());
        assert_eq!("previous", block.previous_block_hash().unwrap());
        assert_eq!(
            "9a2b892c228648282c915af64b3eb85b34d40853ec1c11b07968e370b2f23bc3",
            block.hash().unwrap()
        );
        assert_eq!(55.534, block.timestamp());
    }

    #[derive(Clone)]
    struct BlockHasherMock;

    impl BlockHasher for BlockHasherMock {
        fn hash(&self, _block: &Block) -> Option<String> {
            Some("9a2b892c228648282c915af64b3eb85b34d40853ec1c11b07968e370b2f23bc3".to_string())
        }
    }
}
