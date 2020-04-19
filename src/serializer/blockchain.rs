use crate::model::blockchain::Blockchain;
use crate::serializer::JsonSerializer;
use json::JsonValue;

impl JsonSerializer for Blockchain {
    fn serialize(&self) -> String {
        self.to_json_value().to_string()
    }

    fn to_json_value(&self) -> JsonValue {
        let mut blockchain = JsonValue::new_object();
        let blocks = self.get_blocks();

        blockchain["count"] = blocks.len().into();
        blockchain["blocks"] = JsonValue::new_array();

        for block in blocks {
            blockchain["blocks"].push(block.to_json_value()).ok();
        }

        blockchain
    }
}
