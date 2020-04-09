#[cfg(test)]
pub mod test_serializer_transaction {
    use rchain_db::model::transaction::Transaction;
    use rchain_db::serializer::JsonSerializer;

    #[test]
    fn test_serialize() {
        let transaction = Transaction::new(String::from("s1"), String::from("r1"), 66.6);

        assert_eq!(
            transaction.serialize(),
            String::from("{\"sender\":\"s1\",\"receiver\":\"r1\",\"amount\":66.6}")
        );
    }
}
