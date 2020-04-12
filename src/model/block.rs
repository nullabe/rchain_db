use crate::crypto::Hasher;
use crate::model::transaction::Transaction;

#[derive(Debug, Clone)]
pub struct Block {
    hash: Option<String>,
    index: usize,
    transactions: Vec<Transaction>,
    algorithm_proof: i64,
    previous_block_hash: Option<String>,
}

impl Block {
    pub fn new(
        index: usize,
        transactions: Vec<Transaction>,
        algorithm_proof: i64,
        previous_block_hash: Option<String>,
    ) -> Self {
        let mut block = Self {
            hash: None,
            index,
            transactions,
            algorithm_proof,
            previous_block_hash,
        };

        block.hash = block.hash();

        block
    }

    pub fn get_hash(&self) -> Option<&String> {
        self.hash.as_ref()
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_transactions(&self) -> Vec<Transaction> {
        self.transactions.to_vec()
    }

    pub fn get_algorithm_proof(&self) -> i64 {
        self.algorithm_proof
    }

    pub fn get_previous_block_hash(&self) -> Option<&String> {
        self.previous_block_hash.as_ref()
    }
}
