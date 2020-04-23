use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::ser::SerializeStruct;

use crate::model::block::Block;

const FIELDS_COUNT: usize = 5;

impl Serialize for Block {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        let mut block = serializer.serialize_struct("Block", self::FIELDS_COUNT)?;

        block.serialize_field("algorithm_proof", &self.get_algorithm_proof()).ok();
        block.serialize_field("hash", "").ok();
        block.serialize_field("index", &self.get_index()).ok();
        block.serialize_field("previous_block_hash", "").ok();
        block.serialize_field("transactions", &self.get_transactions()).ok();

        if let Some(hash) = self.get_hash() {
            block.serialize_field("hash", &hash).ok();
        }

        if let Some(previous_block_hash) = self.get_previous_block_hash() {
            block.serialize_field("previous_block_hash", &previous_block_hash).ok();
        }

        block.end()
    }
}
