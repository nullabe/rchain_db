use std::sync::MutexGuard;

use crate::crypto::Sha256Blockchain;

pub trait BlockchainStorage {
    fn retrieve(&self) -> Sha256Blockchain;

    fn persist(&self, blockchain: MutexGuard<Sha256Blockchain>);
}
