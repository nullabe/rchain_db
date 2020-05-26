use std::borrow::Cow;

use crate::error::blockchain::AddBlockToBlockchainError;
use crate::model::block::{Block, BlockHasher};
use crate::model::proof_of_work::{ProofOfWork, ProofValidator};
use crate::model::transaction::Transaction;
use crate::model::NeighbourhoodNodes;

pub const PROOF_OF_WORK_DIFFICULTY: &str = "000";
const SELF_SENDER: &str = "0";
const REWARD_AMOUNT: f64 = 1.0;

#[derive(Debug, Default)]
pub struct Blockchain<T, U> {
    blocks: Vec<Block>,
    transactions_to_process: Vec<Transaction>,
    registered_nodes: NeighbourhoodNodes,
    proof_of_work: ProofOfWork<T>,
    block_hasher: U,
}

impl<T, U> Blockchain<T, U>
where
    T: ProofValidator,
    U: BlockHasher + Clone,
{
    pub fn from(
        blocks: Vec<Block>,
        transactions_to_process: Vec<Transaction>,
        registered_nodes: NeighbourhoodNodes,
        proof_of_work: ProofOfWork<T>,
        block_hasher: U,
    ) -> Self {
        Self {
            blocks,
            transactions_to_process,
            registered_nodes,
            proof_of_work,
            block_hasher,
        }
    }

    pub fn new(proof_validator: T, block_hasher: U) -> Self {
        let blocks: Vec<Block> = Vec::new();

        let transactions_to_process: Vec<Transaction> = Vec::new();

        let registered_nodes: NeighbourhoodNodes = Vec::new();

        let proof_of_work = ProofOfWork::new(self::PROOF_OF_WORK_DIFFICULTY, proof_validator);

        Self {
            blocks,
            transactions_to_process,
            registered_nodes,
            proof_of_work,
            block_hasher,
        }
    }

    pub fn add_new_block(&mut self, node_uuid: &str) -> Result<(), AddBlockToBlockchainError> {
        let mut previous_block_hash: Option<String> = None;

        let mut algorithm_proof = 0;

        if let Some(last_block) = self.last_block() {
            previous_block_hash = last_block.hash().cloned();

            algorithm_proof = self.proof_of_work.generate(last_block.algorithm_proof());
        }

        let block = Block::new(
            self.blocks.len(),
            self.transactions_to_process.to_vec(),
            algorithm_proof,
            previous_block_hash,
            Cow::Borrowed(&self.block_hasher),
        );

        if block.hash().is_none() {
            return Err(AddBlockToBlockchainError::new(
                "Trying to add a block without a hash into blockchain",
            ));
        }

        self.blocks.push(block);

        self.transactions_to_process = Vec::new();

        self.transactions_to_process.push(Transaction::new(
            self::SELF_SENDER,
            node_uuid,
            self::REWARD_AMOUNT,
        ));

        Ok(())
    }

    pub fn add_new_transaction(&mut self, sender: &str, receiver: &str, amount: f64) -> usize {
        let transaction = Transaction::new(sender, receiver, amount);

        self.transactions_to_process.push(transaction);

        self.blocks.len()
    }

    pub fn blocks(&self) -> Vec<Block> {
        self.blocks.clone()
    }

    pub fn transactions_to_process(&self) -> Vec<Transaction> {
        self.transactions_to_process.clone()
    }

    pub fn registered_nodes(&self) -> &NeighbourhoodNodes {
        self.registered_nodes.as_ref()
    }

    pub fn last_block(&self) -> Option<&Block> {
        self.blocks.last()
    }
}
