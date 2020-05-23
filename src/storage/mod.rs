pub mod file;

use crate::crypto::Sha256Blockchain;

pub trait BlockchainStorage {
    const STORAGE_FILE_PATH: &'static str;

    fn retrieve(&self) -> Sha256Blockchain;

    fn persist(&self, blockchain: &Sha256Blockchain);
}
