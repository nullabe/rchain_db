use std::borrow::Cow;

use crate::model::transaction::Transaction;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct Block {
    hash: Option<String>,
    index: usize,
    transactions: Vec<Transaction>,
    algorithm_proof: i64,
    previous_block_hash: Option<String>,
    timestamp: f64,
}

impl Block {
    pub fn from(
        hash: Option<String>,
        index: usize,
        transactions: Vec<Transaction>,
        algorithm_proof: i64,
        previous_block_hash: Option<String>,
        timestamp: f64,
    ) -> Self {
        Block {
            hash,
            index,
            transactions,
            algorithm_proof,
            previous_block_hash,
            timestamp,
        }
    }

    pub fn new<T: BlockHasher + Clone>(
        index: usize,
        transactions: Vec<Transaction>,
        algorithm_proof: i64,
        previous_block_hash: Option<String>,
        block_hasher: Cow<T>,
    ) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();

        let mut block = Self {
            hash: None,
            index,
            transactions,
            algorithm_proof,
            previous_block_hash,
            timestamp: timestamp.as_secs_f64(),
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

    pub fn timestamp(&self) -> f64 {
        self.timestamp
    }
}

pub trait BlockHasher {
    fn hash(&self, block: &Block) -> Option<String>;
}
