#[cfg(test)]
pub mod test_db_blockchain {
    use rchain_db::model::block::Block;
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

        let next_block_index =
            blockchain.add_new_transaction(String::from("sender"), String::from("receiver"), 66.6);

        assert_eq!(0, next_block_index);
    }

    #[test]
    fn test_add_new_block() -> Result<(), String> {
        let mut blockchain = Blockchain::new();

        match blockchain.add_new_block() {
            Err(err) => {
                return Err(err.message().clone());
            }

            _ => (),
        }

        match blockchain.last_block() {
            Some(_) => Ok(()),

            None => Err(String::from("There should be a block")),
        }
    }

    #[test]
    fn test_add_two_new_block() -> Result<(), String> {
        let mut blockchain = Blockchain::new();

        match blockchain.add_new_block() {
            Err(err) => {
                return Err(err.message().clone());
            }

            _ => (),
        }

        match blockchain.last_block() {
            None => {
                return Err(String::from("There should be a block"));
            }

            Some(_) => (),
        }

        match blockchain.add_new_block() {
            Err(err) => {
                return Err(err.message().clone());
            }

            _ => (),
        }

        let last_block: &Block;

        match blockchain.last_block() {
            None => {
                return Err(String::from("There should be a block"));
            }

            Some(block) => {
                last_block = block;
            }
        }

        match last_block.get_algorithm_proof() {
            0 => Err(String::from(
                "Only first block can have an algorithm_proof eq to 0",
            )),

            _ => Ok(()),
        }
    }
}
