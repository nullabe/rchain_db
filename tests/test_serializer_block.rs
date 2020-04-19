#[cfg(test)]
pub mod test_serializer_block {
    use rchain_db::model::block::Block;
    use rchain_db::model::transaction::Transaction;
    use rchain_db::serializer::JsonSerializer;

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
            String::from("{\"hash\":\"166c335bbac8cb493c2326be0141e1f3c4dd7a0b3aa30459961a05148b57c396\",\"index\":6,\"transactions\":[{\"sender\":\"s1\",\"receiver\":\"r1\",\"amount\":66.6},{\"sender\":\"s2\",\"receiver\":\"r2\",\"amount\":42.42}],\"algorithm_proof\":90838340972,\"previous_block_hash\":\"previous\"}"),
            block.serialize()
        );
    }
}
