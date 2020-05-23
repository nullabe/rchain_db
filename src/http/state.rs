use std::sync::{Arc, Mutex};

use crate::crypto::Sha256Blockchain;
use crate::storage::file::blockchain::BlockchainFileStorage;
use crate::storage::BlockchainStorage;

pub struct BlockchainState {
    blockchain: Arc<Mutex<Sha256Blockchain>>,
    blockchain_storage: BlockchainFileStorage,
    node_uuid: String,
}

impl BlockchainState {
    pub fn from(blockchain_storage: BlockchainFileStorage, node_uuid: &str) -> Self {
        let blockchain = blockchain_storage.retrieve();
        let blockchain = Arc::new(Mutex::new(blockchain));

        BlockchainState {
            blockchain,
            blockchain_storage,
            node_uuid: node_uuid.to_string(),
        }
    }

    pub fn blockchain(&self) -> &Arc<Mutex<Sha256Blockchain>> {
        &self.blockchain
    }

    pub fn node_uuid(&self) -> &str {
        &self.node_uuid
    }

    pub fn persist_state(&self, blockchain: &Sha256Blockchain) {
        self.blockchain_storage.persist(&blockchain);
    }
}
