#[cfg(test)]
pub mod test_model_blockchain {
    use rchain_db::model::block::{Block, BlockHasher};
    use rchain_db::model::blockchain::Blockchain;
    use rchain_db::model::proof_of_work::ProofValidator;

    #[test]
    fn test_new_with_empty_last_block() -> Result<(), String> {
        let blockchain = Blockchain::new(ProofValidatorMock, BlockHasherMock);

        match blockchain.last_block() {
            Some(_) => Err(String::from("Last block should be empty")),

            None => Ok(()),
        }
    }

    #[test]
    fn test_add_new_transaction_return_0() {
        let mut blockchain = Blockchain::new(ProofValidatorMock, BlockHasherMock);

        let next_block_index = blockchain.add_transactions_to_process("sender", "receiver", 66.6);

        assert_eq!(0, next_block_index);
    }

    #[test]
    fn test_add_new_block() -> Result<(), String> {
        let mut blockchain = Blockchain::new(ProofValidatorMock, BlockHasherMock);

        match blockchain.add_block("test") {
            Err(err) => {
                return Err(err.message().clone());
            }

            _ => (),
        }

        match blockchain.last_block() {
            Some(_) => (),

            None => return Err(String::from("There should be a block")),
        };

        let next_block_index = blockchain.add_transactions_to_process("sender", "receiver", 66.6);

        assert_eq!(1, next_block_index);

        Ok(())
    }

    #[test]
    fn test_add_two_new_block() -> Result<(), String> {
        let mut blockchain = Blockchain::new(ProofValidatorMock, BlockHasherMock);

        match blockchain.add_block("test") {
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

        match blockchain.add_block("test") {
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

        match last_block.algorithm_proof() {
            0 => Ok(()),

            _ => Err(String::from("Algorithm_proof should be eq to 0")),
        }
    }

    #[test]
    fn test_add_new_block_reward_transaction() -> Result<(), String> {
        let mut blockchain = Blockchain::new(ProofValidatorMock, BlockHasherMock);

        match blockchain.add_block("test") {
            Err(err) => {
                return Err(err.message().clone());
            }

            _ => (),
        }

        let transactions = blockchain.transactions_to_process();
        let reward_transaction = transactions.first().unwrap();

        assert_eq!(1.0, reward_transaction.amount());
        assert_eq!("0", reward_transaction.sender());
        assert_eq!("test", reward_transaction.receiver());

        Ok(())
    }

    #[test]
    fn test_register_node() -> Result<(), String> {
        let mut blockchain = Blockchain::new(ProofValidatorMock, BlockHasherMock);

        match blockchain.add_registered_node("uuuuiiiiiddddd", "127.0.0.1:8080") {
            Err(err) => {
                return Err(err.message().clone());
            }

            Ok(_some) => (),
        }

        match blockchain.registered_nodes().first() {
            Some(_node) => (),

            None => return Err(String::from("No node found")),
        }

        match blockchain.add_registered_node("uuuuiiiiiddddd", "127.0.0.1:8080") {
            Err(_err) => Ok(()),

            Ok(_some) => Err(String::from("Registered node must be idempotent")),
        }
    }

    struct ProofValidatorMock;

    impl ProofValidator for ProofValidatorMock {
        fn validate(&self, _to_validate: &str, _difficulty: &str) -> bool {
            true
        }
    }

    #[derive(Clone)]
    struct BlockHasherMock;

    impl BlockHasher for BlockHasherMock {
        fn hash(&self, _block: &Block) -> Option<String> {
            Some("9a2b892c228648282c915af64b3eb85b34d40853ec1c11b07968e370b2f23bc3".to_string())
        }
    }
}
