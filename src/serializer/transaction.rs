use std::fmt;

use serde::de::{self, MapAccess, Visitor};
use serde::export::Formatter;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::model::transaction::Transaction;

const FIELDS_COUNT: usize = 3;
const FIELDS: [&str; self::FIELDS_COUNT] = ["amount", "receiver", "sender"];

enum TransactionField {
    Amount,
    Receiver,
    Sender,
}

struct TransactionVisitor;

struct TransactionFieldVisitor;

impl<'de> Visitor<'de> for TransactionFieldVisitor {
    type Value = TransactionField;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("'amount', 'receiver' and 'sender'")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match value {
            "amount" => Ok(TransactionField::Amount),
            "receiver" => Ok(TransactionField::Receiver),
            "sender" => Ok(TransactionField::Sender),

            _ => Err(de::Error::unknown_field(value, &self::FIELDS)),
        }
    }
}

impl<'de> Deserialize<'de> for TransactionField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_identifier(TransactionFieldVisitor)
    }
}

impl<'de> Visitor<'de> for TransactionVisitor {
    type Value = Transaction;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("struct Transaction")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut amount = None;
        let mut receiver = None;
        let mut sender = None;

        while let Some(key) = map.next_key()? {
            match key {
                TransactionField::Amount => {
                    if amount.is_some() {
                        return Err(de::Error::duplicate_field("amount"));
                    }

                    amount = Some(map.next_value()?);
                }
                TransactionField::Receiver => {
                    if receiver.is_some() {
                        return Err(de::Error::duplicate_field("receiver"));
                    }

                    receiver = Some(map.next_value()?);
                }
                TransactionField::Sender => {
                    if sender.is_some() {
                        return Err(de::Error::duplicate_field("sender"));
                    }

                    sender = Some(map.next_value()?);
                }
            }
        }

        let amount = amount.ok_or_else(|| de::Error::missing_field("amont"))?;
        let receiver = receiver.ok_or_else(|| de::Error::missing_field("receiver"))?;
        let sender = sender.ok_or_else(|| de::Error::missing_field("sender"))?;

        Ok(Transaction::new(sender, receiver, amount))
    }
}

impl<'de> Deserialize<'de> for Transaction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct("Transaction", &self::FIELDS, TransactionVisitor)
    }
}

impl Serialize for Transaction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut transaction = serializer.serialize_struct("Transaction", self::FIELDS_COUNT)?;

        transaction
            .serialize_field("amount", &self.get_amount())
            .ok();
        transaction
            .serialize_field("receiver", &self.get_receiver())
            .ok();
        transaction
            .serialize_field("sender", &self.get_sender())
            .ok();

        transaction.end()
    }
}
