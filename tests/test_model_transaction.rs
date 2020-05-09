#[cfg(test)]
pub mod test_model_transaction {
    use rchain_db::model::transaction::Transaction;

    #[test]
    fn test_get_sender() {
        let transaction = get_transaction();

        assert_eq!("toto", transaction.sender());
    }

    #[test]
    fn test_get_receiver() {
        let transaction = get_transaction();

        assert_eq!("tata", transaction.receiver());
    }

    #[test]
    fn test_get_amount() {
        let transaction = get_transaction();

        assert_eq!(66.6, transaction.amount());
    }

    fn get_transaction() -> Transaction {
        Transaction::new("toto", "tata", 66.6)
    }
}
