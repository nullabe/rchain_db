#[cfg(test)]
pub mod test_serializer_transaction {
    use serde::Serialize;
    use serde_json::value::Serializer;

    use rchain_db::model::transaction::Transaction;

    #[test]
    fn test_serialize() {
        let transaction = Transaction::new("s1", "r1", 66.6);

        assert_eq!(
            String::from("{\"amount\":66.6,\"receiver\":\"r1\",\"sender\":\"s1\"}"),
            transaction.serialize(Serializer).unwrap().to_string()
        );
    }

    #[test]
    fn test_deserialize() {
        let transaction: Transaction =
            serde_json::from_str("{\"amount\":66.6,\"receiver\":\"r1\",\"sender\":\"s1\"}")
                .unwrap();
        let transaction_expected = Transaction::new("s1", "r1", 66.6);

        assert_eq!(transaction_expected.amount(), transaction.amount());

        assert_eq!(transaction_expected.receiver(), transaction.receiver());

        assert_eq!(transaction_expected.sender(), transaction.sender());
    }
}
