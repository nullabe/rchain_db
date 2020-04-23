#[cfg(test)]
pub mod test_serializer_transaction {
    use serde::Serialize;
    use serde_json::value::Serializer;

    use rchain_db::model::transaction::Transaction;

    #[test]
    fn test_serialize() {
        let transaction = Transaction::new(String::from("s1"), String::from("r1"), 66.6);

        assert_eq!(
            String::from("{\"amount\":66.6,\"receiver\":\"r1\",\"sender\":\"s1\"}"),
            transaction.serialize(Serializer).unwrap().to_string()
        );
    }
}
