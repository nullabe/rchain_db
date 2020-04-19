use crate::model::blockchain::Blockchain;

pub struct BlockchainState {
    blockchain: Blockchain,
}

impl BlockchainState {
    pub fn from(blockchain: Blockchain) -> Self {
        BlockchainState { blockchain }
    }

    pub fn get_blockchain(&self) -> &Blockchain {
        &self.blockchain
    }
}
