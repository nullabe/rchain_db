pub mod file;

use crate::crypto::Sha256Blockchain;
use std::sync::{Arc, Mutex};

pub trait BlockchainStorage {
    fn retrieve(&self) -> Sha256Blockchain;

    fn persist(&self, blockchain: &Arc<Mutex<Sha256Blockchain>>);
}
