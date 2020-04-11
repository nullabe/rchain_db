#[cfg(test)]
pub mod test_db_blockchain {
    use rchain_db::model::blockchain::Blockchain;

    #[test]
    fn test_new_with_empty_last_block() -> Result<(), String> {
        let blockchain = Blockchain::new();

        match blockchain.last_block() {
            Some(_) => Err(String::from("Last block should be empty")),

            None => Ok(()),
        }
    }

    #[test]
    fn test_add_new_transaction_return_0() {
        let mut blockchain = Blockchain::new();
        let amount: f64 = 66.6;

        let next_block_index = blockchain.add_new_transaction(
            String::from("sender"),
            String::from("receiver"),
            amount,
        );

        assert_eq!(0, next_block_index);
    }
}
