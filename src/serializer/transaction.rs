use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::ser::{SerializeStruct, SerializeMap};

use crate::model::transaction::Transaction;

const FIELDS_COUNT: usize = 3;

impl Serialize for Transaction {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        let mut transaction = serializer.serialize_struct("Transaction", self::FIELDS_COUNT)?;

        transaction.serialize_field("amount", &self.get_amount()).ok();
        transaction.serialize_field("receiver", &self.get_receiver()).ok();
        transaction.serialize_field("sender", &self.get_sender()).ok();

        transaction.end()
    }
}

impl Deserialize for Transaction {
    fn deserialize<'de, D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error> where
        D: Deserializer<'de> {
        unimplemented!()
    }
}
