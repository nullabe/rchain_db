use json::JsonValue;

use crate::model::transaction::Transaction;
use crate::serializer::JsonSerializer;

impl JsonSerializer for Transaction {
    fn serialize(&self) -> String {
        self.to_json_value().to_string()
    }

    fn to_json_value(&self) -> JsonValue {
        let mut transaction = JsonValue::new_object();

        transaction["sender"] = self.get_sender().into();
        transaction["receiver"] = self.get_receiver().into();
        transaction["amount"] = self.get_amount().into();

        transaction
    }
}
