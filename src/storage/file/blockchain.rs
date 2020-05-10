use std::sync::MutexGuard;

use crate::crypto::Sha256Blockchain;
use crate::storage::blockchain::BlockchainStorage;

pub struct BlockchainFileStorage;

impl BlockchainStorage for BlockchainFileStorage {
    fn retrieve(&self) -> Sha256Blockchain {
        unimplemented!()
    }

    fn persist(&self, blockchain: MutexGuard<Sha256Blockchain>) {
        unimplemented!()
    }
}
