use json::JsonValue;

use crate::model::transaction::Transaction;
use crate::serializer::JsonSerializer;

impl JsonSerializer for Transaction {
    fn serialize(&self) -> String {
        let mut transaction = JsonValue::new_object();

        transaction["sender"] = self.get_sender().into();
        transaction["receiver"] = self.get_receiver().into();
        transaction["amount"] = self.get_amount().into();

        transaction.to_string()
    }
}
