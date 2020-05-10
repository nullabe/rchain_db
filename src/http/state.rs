use std::sync::{Arc, Mutex};

use crate::crypto::Sha256Blockchain;

pub struct BlockchainState {
    blockchain: Arc<Mutex<Sha256Blockchain>>,
    node_uuid: String,
}

impl BlockchainState {
    pub fn from(blockchain: Arc<Mutex<Sha256Blockchain>>, node_uuid: &str) -> Self {
        BlockchainState {
            blockchain,
            node_uuid: node_uuid.to_string(),
        }
    }

    pub fn blockchain(&self) -> &Arc<Mutex<Sha256Blockchain>> {
        &self.blockchain
    }

    pub fn node_uuid(&self) -> &str {
        &self.node_uuid
    }
}
