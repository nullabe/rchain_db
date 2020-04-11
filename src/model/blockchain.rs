use crate::error::blockchain::AddBlockToBlockchainError;
use crate::model::block::Block;
use crate::model::proof_of_work::ProofOfWork;
use crate::model::transaction::Transaction;

#[derive(Debug, Default)]
pub struct Blockchain {
    blocks: Vec<Block>,
    transactions_to_process: Vec<Transaction>,
    proof_of_work: ProofOfWork,
}

impl Blockchain {
    pub fn new() -> Self {
        let blocks: Vec<Block> = Vec::new();
        let transactions_to_process: Vec<Transaction> = Vec::new();
        let proof_of_work = ProofOfWork::new(String::from("0000"));

        Self {
            blocks,
            transactions_to_process,
            proof_of_work,
        }
    }

    pub fn add_new_block(&mut self) -> Result<(), AddBlockToBlockchainError> {
        let mut previous_block_hash: Option<String> = None;
        let mut algorithm_proof = 0;

        if let Some(last_block) = self.last_block() {
            previous_block_hash = last_block.get_hash().cloned();

            algorithm_proof = self
                .proof_of_work
                .generate(last_block.get_algorithm_proof());
        }

        let block = Block::new(
            self.next_block_index(),
            self.transactions_to_process.to_vec(),
            algorithm_proof,
            previous_block_hash,
        );

        if block.get_hash().is_none() {
            return Err(AddBlockToBlockchainError::new(String::from(
                "Trying to add a block without a hash into blockchain",
            )));
        }

        self.blocks.push(block);

        self.transactions_to_process = Vec::new();

        Ok(())
    }

    pub fn add_new_transaction(&mut self, sender: String, receiver: String, amount: f64) -> usize {
        let transaction = Transaction::new(sender, receiver, amount);

        self.transactions_to_process.push(transaction);

        self.next_block_index()
    }

    pub fn last_block(&self) -> Option<&Block> {
        self.blocks.last()
    }

    fn next_block_index(&self) -> usize {
        self.blocks.len()
    }
}
