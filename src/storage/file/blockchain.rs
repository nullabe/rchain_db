use crate::crypto::block::Sha256BlockHasher;
use crate::crypto::proof_of_work::Sha256ProofValidator;
use crate::crypto::Sha256Blockchain;
use crate::model::blockchain::Blockchain;
use crate::storage::BlockchainStorage;
use std::sync::{Arc, Mutex};

pub struct BlockchainFileStorage;

impl BlockchainStorage for BlockchainFileStorage {
    fn retrieve(&self) -> Sha256Blockchain {
        Blockchain::new(Sha256ProofValidator, Sha256BlockHasher)
    }

    fn persist(&self, _blockchain: &Arc<Mutex<Sha256Blockchain>>) {}
}
