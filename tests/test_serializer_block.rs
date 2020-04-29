#[cfg(test)]
pub mod test_serializer_block {
    use serde::Serialize;
    use serde_json::value::Serializer;

    use rchain_db::model::block::Block;
    use rchain_db::model::transaction::Transaction;

    #[test]
    fn test_serialize() {
        let mut transactions = Vec::new();

        transactions.push(Transaction::new(
            String::from("s1"),
            String::from("r1"),
            66.6,
        ));
        transactions.push(Transaction::new(
            String::from("s2"),
            String::from("r2"),
            42.42,
        ));

        let block = Block::new(6, transactions, 90838340972, Some(String::from("previous")));

        assert_eq!(
            String::from("{\"algorithm_proof\":90838340972,\"hash\":\"53e0938e2a8194d3dd7cd94d76ded2bf8c55bd3e9669af20434cb9183bd6eb08\",\"index\":6,\"previous_block_hash\":\"previous\",\"transactions\":[{\"amount\":66.6,\"receiver\":\"r1\",\"sender\":\"s1\"},{\"amount\":42.42,\"receiver\":\"r2\",\"sender\":\"s2\"}]}"),
            block.serialize(Serializer).unwrap().to_string()
        );
    }
}
