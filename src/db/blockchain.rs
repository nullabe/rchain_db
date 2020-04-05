use crate::db::block::Block;
use crate::db::transaction::Transaction;

#[derive(Debug, Default)]
pub struct Blockchain {
    blocks: Vec<Block>,
    current_transactions: Vec<Transaction>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let blocks: Vec<Block> = Vec::new();
        let current_transactions: Vec<Transaction> = Vec::new();

        Blockchain {
            blocks,
            current_transactions,
        }
    }

    pub fn add_new_block(&mut self) {}

    pub fn last_block(&self) -> Option<&Block> {
        self.blocks.last()
    }
}
