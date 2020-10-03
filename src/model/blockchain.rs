use std::borrow::Cow;

use crate::error::blockchain::{AddBlockToBlockchainError, RegisterNodeToBlockchainError};
use crate::model::block::{Block, BlockHasher};
use crate::model::node::{NeighbourServer, Node};
use crate::model::proof_of_work::{ProofOfWork, ProofValidator};
use crate::model::transaction::Transaction;
use crate::model::NeighbourhoodNodes;

pub const PROOF_OF_WORK_DIFFICULTY: &str = "00000";
const SELF_SENDER: &str = "0";
const REWARD_AMOUNT: f64 = 1.0;

#[derive(Debug, Default)]
pub struct Blockchain<T, U> {
    blocks: Vec<Block>,
    transactions_to_process: Vec<Transaction>,
    neighbour_nodes: NeighbourhoodNodes,
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
        neighbour_nodes: NeighbourhoodNodes,
        proof_of_work: ProofOfWork<T>,
        block_hasher: U,
    ) -> Self {
        Self {
            blocks,
            transactions_to_process,
            neighbour_nodes,
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
            neighbour_nodes: registered_nodes,
            proof_of_work,
            block_hasher,
        }
    }

    pub fn blocks(&self) -> Vec<Block> {
        self.blocks.clone()
    }

    pub fn add_block(&mut self, node_uuid: &str) -> Result<(), AddBlockToBlockchainError> {
        let mut previous_block_hash: Option<String> = None;

        let mut algorithm_proof = 0;

        if let Some(last_block) = self.last_block() {
            previous_block_hash = last_block.hash().cloned();

            algorithm_proof = self
                .proof_of_work
                .generate_algorithm_proof(last_block.algorithm_proof());
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

    pub fn replace_blocks(&mut self, blocks: Vec<Block>) -> &mut Self {
        self.blocks = blocks;

        self
    }

    pub fn last_block(&self) -> Option<&Block> {
        self.blocks.last()
    }

    pub fn transactions_to_process(&self) -> Vec<Transaction> {
        self.transactions_to_process.clone()
    }

    pub fn add_transactions_to_process(
        &mut self,
        sender: &str,
        receiver: &str,
        amount: f64,
    ) -> usize {
        let transaction = Transaction::new(sender, receiver, amount);

        self.transactions_to_process.push(transaction);

        self.blocks.len()
    }

    pub fn replace_transactions_to_process(
        &mut self,
        transactions_to_process: Vec<Transaction>,
    ) -> &mut Self {
        self.transactions_to_process = transactions_to_process;

        self
    }

    pub fn neighbour_nodes(&self) -> NeighbourhoodNodes {
        self.neighbour_nodes.clone()
    }

    pub fn add_neighbour_node(
        &mut self,
        uuid: &str,
        url: &str,
    ) -> Result<(), RegisterNodeToBlockchainError> {
        let mut node = Node::new(NeighbourServer);

        for node in &self.neighbour_nodes {
            if uuid != node.uuid().unwrap_or(&String::from(""))
                && url != node.url().unwrap_or(&String::from(""))
            {
                continue;
            }

            return Err(RegisterNodeToBlockchainError::new(
                "Node already registered",
            ));
        }

        node.set_uuid(uuid).set_url(url);

        self.neighbour_nodes.push(node);

        Ok(())
    }

    pub fn is_valid(&self) -> bool {
        let mut previous_block = self.blocks.first();

        if previous_block.is_none() {
            return true;
        }

        for block in &self.blocks {
            if 0 == block.index() {
                continue;
            }

            if block.previous_block_hash() != previous_block.unwrap().hash() {
                return false;
            }

            if block.algorithm_proof()
                != self
                    .proof_of_work
                    .generate_algorithm_proof(previous_block.unwrap().algorithm_proof())
            {
                return false;
            }

            previous_block = Some(block);
        }

        true
    }
}
