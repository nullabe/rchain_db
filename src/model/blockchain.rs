use crate::model::block::Block;
use crate::model::transaction::Transaction;

#[derive(Debug)]
pub struct Blockchain<'blockchain> {
    blocks: Vec<Block<'blockchain>>,
    unprocessed_transactions: Vec<Transaction<'blockchain>>,
}

impl<'blockchain> Blockchain<'blockchain> {
    pub fn new() -> Self {
        let blocks: Vec<Block> = Vec::new();
        let unprocessed_transactions: Vec<Transaction> = Vec::new();

        Blockchain {
            blocks,
            unprocessed_transactions,
        }
    }

    pub fn add_new_block(&mut self) {}

    pub fn add_new_transaction(&mut self, sender: String, receiver: String, amount: f64) -> usize {
        let transaction = Transaction::new(sender, receiver, amount);

        self.unprocessed_transactions.push(transaction);

        // Currently return next block index
        self.blocks.len()
    }

    pub fn last_block(&self) -> Option<&Block> {
        self.blocks.last()
    }
}

impl<'blockchain> Default for Blockchain<'blockchain> {
    fn default() -> Self {
        Blockchain::new()
    }
}
