use std::sync::{Arc, Mutex};

use crate::model::blockchain::Blockchain;

pub struct BlockchainState {
    blockchain: Arc<Mutex<Blockchain>>,
    node_uuid: String,
}

impl BlockchainState {
    pub fn from(blockchain: Arc<Mutex<Blockchain>>, node_uuid: &str) -> Self {
        BlockchainState {
            blockchain,
            node_uuid: node_uuid.to_string(),
        }
    }

    pub fn get_blockchain(&self) -> &Arc<Mutex<Blockchain>> {
        &self.blockchain
    }

    pub fn get_node_uuid(&self) -> &str {
        &self.node_uuid
    }
}
