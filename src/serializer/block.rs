use json::JsonValue;

use crate::model::block::Block;
use crate::serializer::JsonSerializer;

impl JsonSerializer for Block {
    fn serialize(&self) -> String {
        let mut block = JsonValue::new_object();
        let mut hash = String::from("");
        let mut previous_block_hash = String::from("");

        if let Some(self_hash) = self.get_hash() {
            hash = self_hash.clone();
        }

        if let Some(self_previous_block_hash) = self.get_previous_block_hash() {
            previous_block_hash = self_previous_block_hash.clone();
        }

        block["hash"] = hash.into();
        block["index"] = self.get_index().into();
        block["transactions"] = JsonValue::new_array();
        block["algorithm_proof"] = self.get_algorithm_proof().into();
        block["previous_block_hash"] = previous_block_hash.into();

        for transaction in self.get_transactions() {
            block["transactions"].push(transaction.serialize()).ok();
        }

        block.to_string()
    }
}
