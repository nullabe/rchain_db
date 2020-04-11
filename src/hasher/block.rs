use crypto::digest::Digest;
use crypto::sha2::Sha256;

use crate::hasher::ModelHasher;
use crate::model::block::Block;
use crate::serializer::JsonSerializer;

impl ModelHasher for Block {
    fn hash(&mut self) -> Option<String> {
        let mut hasher = Sha256::new();

        hasher.input_str(self.serialize().as_str());

        Some(hasher.result_str())
    }
}
