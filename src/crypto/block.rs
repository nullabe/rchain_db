use crypto::digest::Digest;
use crypto::sha2::Sha256;
use serde::Serialize;

use crate::crypto::Hasher;
use crate::model::block::Block;
use serde_json::value::Serializer;

impl Hasher for Block {
    fn hash(&mut self) -> Option<String> {
        let mut hasher = Sha256::new();

        hasher.input_str(&self.serialize(Serializer).unwrap().to_string());

        Some(hasher.result_str())
    }
}
