use std::borrow::Cow;

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
    pub fn from_values(
        hash: Option<String>,
        index: usize,
        transactions: Vec<Transaction>,
        algorithm_proof: i64,
        previous_block_hash: Option<String>,
    ) -> Self {
        Block {
            hash,
            index,
            transactions,
            algorithm_proof,
            previous_block_hash,
        }
    }

    pub fn new<T: BlockHasher + Clone>(
        index: usize,
        transactions: Vec<Transaction>,
        algorithm_proof: i64,
        previous_block_hash: Option<String>,
        block_hasher: Cow<T>,
    ) -> Self {
        let mut block = Self {
            hash: None,
            index,
            transactions,
            algorithm_proof,
            previous_block_hash,
        };

        block.hash = block_hasher.hash(&block);

        block
    }

    pub fn hash(&self) -> Option<&String> {
        self.hash.as_ref()
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn transactions(&self) -> Vec<Transaction> {
        self.transactions.to_vec()
    }

    pub fn algorithm_proof(&self) -> i64 {
        self.algorithm_proof
    }

    pub fn previous_block_hash(&self) -> Option<&String> {
        self.previous_block_hash.as_ref()
    }
}

pub trait BlockHasher {
    fn hash(&self, block: &Block) -> Option<String>;
}
