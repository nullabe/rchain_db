pub mod file;

use std::sync::{Arc, Mutex};

use crate::crypto::Sha256Blockchain;

pub trait BlockchainStorage {
    fn retrieve(&self) -> Sha256Blockchain;

    fn persist(&self, blockchain: &Arc<Mutex<Sha256Blockchain>>);
}
