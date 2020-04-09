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
            block.serialize(),
            String::from("{\"hash\":\"417ae7df7e2a7a573867b70cf4f9a79c5eb275161a456ce4d80628568cd71f36\",\"index\":6,\"transactions\":[\"{\\\"sender\\\":\\\"s1\\\",\\\"receiver\\\":\\\"r1\\\",\\\"amount\\\":66.6}\",\"{\\\"sender\\\":\\\"s2\\\",\\\"receiver\\\":\\\"r2\\\",\\\"amount\\\":42.42}\"],\"algorithm_proof\":90838340972,\"previous_block_hash\":\"previous\"}")
        );
    }
}
