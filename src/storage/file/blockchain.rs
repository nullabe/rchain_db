use std::fs::{remove_file, File};
use std::io::{Read, Write};
use std::path::Path;

use crate::crypto::block::Sha256BlockHasher;
use crate::crypto::proof_of_work::Sha256ProofValidator;
use crate::crypto::Sha256Blockchain;
use crate::model::blockchain::Blockchain;
use crate::storage::BlockchainStorage;

pub struct BlockchainFileStorage;

impl BlockchainStorage for BlockchainFileStorage {
    const STORAGE_FILE_PATH: &'static str = "bsave_state.txt";

    fn retrieve(&self) -> Sha256Blockchain {
        if !self.storage_file_exist(Self::STORAGE_FILE_PATH) {
            return Blockchain::new(Sha256ProofValidator, Sha256BlockHasher);
        }

        let mut storage_file = File::open(Self::STORAGE_FILE_PATH).unwrap();

        let mut storage_file_content = String::new();

        match storage_file.read_to_string(&mut storage_file_content) {
            Err(_err) => Blockchain::new(Sha256ProofValidator, Sha256BlockHasher),

            Ok(_) => {
                let blockchain: serde_json::Result<Sha256Blockchain> =
                    serde_json::from_str(&storage_file_content);

                if let Err(_err) = blockchain {
                    return Blockchain::new(Sha256ProofValidator, Sha256BlockHasher);
                }

                blockchain.unwrap()
            }
        }
    }

    fn persist(&self, blockchain: &Sha256Blockchain) {
        if self.storage_file_exist(Self::STORAGE_FILE_PATH) {
            remove_file(Self::STORAGE_FILE_PATH).unwrap();
        }

        let path = Path::new(Self::STORAGE_FILE_PATH);

        let mut file = File::create(&path).unwrap();

        file.write_all(serde_json::to_string(blockchain).unwrap().as_bytes())
            .unwrap();
    }
}

impl BlockchainFileStorage {
    fn storage_file_exist(&self, path: &str) -> bool {
        let path = Path::new(path);

        match File::open(path) {
            Err(_err) => false,

            Ok(_file) => true,
        }
    }
}
