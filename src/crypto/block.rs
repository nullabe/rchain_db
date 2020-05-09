use crypto::digest::Digest;
use crypto::sha2::Sha256;
use serde::Serialize;

use crate::model::block::{Block, BlockHasher};
use serde_json::value::Serializer;

#[derive(Clone)]
pub struct Sha256BlockHasher;

impl BlockHasher for Sha256BlockHasher {
    fn hash(&self, block: &Block) -> Option<String> {
        let mut hasher = Sha256::new();

        hasher.input_str(&block.serialize(Serializer).unwrap().to_string());

        Some(hasher.result_str())
    }
}
