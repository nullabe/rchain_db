use std::sync::{Arc, Mutex};

use crate::model::blockchain::Blockchain;

pub struct BlockchainState {
    blockchain: Arc<Mutex<Blockchain>>,
}

impl BlockchainState {
    pub fn from(blockchain: Arc<Mutex<Blockchain>>) -> Self {
        BlockchainState { blockchain }
    }

    pub fn get_blockchain(&self) -> &Arc<Mutex<Blockchain>> {
        &self.blockchain
    }
}
