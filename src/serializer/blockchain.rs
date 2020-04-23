use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::ser::SerializeStruct;

use crate::model::blockchain::Blockchain;

const FIELDS_COUNT: usize = 2;

impl Serialize for Blockchain {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        let mut blockchain = serializer.serialize_struct("Blockchain", self::FIELDS_COUNT)?;

        blockchain.serialize_field("count", &self.get_blocks().len()).ok();
        blockchain.serialize_field("blocks", &self.get_blocks()).ok();

        blockchain.end()
    }
}
