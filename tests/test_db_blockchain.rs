#[cfg(test)]
pub mod test_db_blockchain {
    use rchain_db::db::blockchain::Blockchain;

    #[test]
    fn test_new_with_empty_last_block() -> Result<(), String> {
        let blockchain = Blockchain::new();

        match blockchain.last_block() {
            Some(_) => {
                Err(String::from("Last block should be empty"))
            }

            None => {
                Ok(())
            }
        }
    }
}
