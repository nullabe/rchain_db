use std::fmt;

use serde::de::{self, MapAccess, Visitor};
use serde::export::Formatter;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::model::block::Block;

const FIELDS_COUNT: usize = 5;
const FIELDS: [&str; FIELDS_COUNT] = [
    "hash",
    "index",
    "transactions",
    "algorithm_proof",
    "previous_block_hash",
];

enum BlockField {
    Hash,
    Index,
    Transactions,
    AlgorithmProof,
    PreviousBlockHash,
    Timestamp,
}

struct BlockFieldVisitor;

struct BlockVisitor;

impl<'de> Visitor<'de> for BlockFieldVisitor {
    type Value = BlockField;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter
            .write_str("'hash', 'index', 'transactions', 'algorithm_proof', 'previous_block_hash', 'timestamp'")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match value {
            "hash" => Ok(BlockField::Hash),
            "index" => Ok(BlockField::Index),
            "transactions" => Ok(BlockField::Transactions),
            "algorithm_proof" => Ok(BlockField::AlgorithmProof),
            "previous_block_hash" => Ok(BlockField::PreviousBlockHash),
            "timestamp" => Ok(BlockField::Timestamp),

            _ => Err(de::Error::unknown_field(value, &self::FIELDS)),
        }
    }
}

impl<'de> Deserialize<'de> for BlockField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_identifier(BlockFieldVisitor)
    }
}

impl<'de> Visitor<'de> for BlockVisitor {
    type Value = Block;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("struct Block")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut hash = None;
        let mut index = None;
        let mut transactions = None;
        let mut algorithm_proof = None;
        let mut previous_block_hash = None;
        let mut timestamp = None;

        while let Some(key) = map.next_key()? {
            match key {
                BlockField::Hash => {
                    if hash.is_some() {
                        return Err(de::Error::duplicate_field("hash"));
                    }

                    hash = Some(map.next_value()?);
                }
                BlockField::Index => {
                    if index.is_some() {
                        return Err(de::Error::duplicate_field("index"));
                    }

                    index = Some(map.next_value()?);
                }
                BlockField::Transactions => {
                    if transactions.is_some() {
                        return Err(de::Error::duplicate_field("transactions"));
                    }

                    transactions = Some(map.next_value()?);
                }
                BlockField::AlgorithmProof => {
                    if algorithm_proof.is_some() {
                        return Err(de::Error::duplicate_field("algorithm_proof"));
                    }

                    algorithm_proof = Some(map.next_value()?);
                }
                BlockField::PreviousBlockHash => {
                    if previous_block_hash.is_some() {
                        return Err(de::Error::duplicate_field("previous_block_hash"));
                    }

                    previous_block_hash = Some(map.next_value()?);
                }
                BlockField::Timestamp => {
                    if timestamp.is_some() {
                        return Err(de::Error::duplicate_field("timestamp"));
                    }

                    timestamp = Some(map.next_value()?);
                }
            }
        }

        let hash = hash.ok_or_else(|| de::Error::missing_field("hash"))?;

        let index = index.ok_or_else(|| de::Error::missing_field("index"))?;

        let transactions = transactions.ok_or_else(|| de::Error::missing_field("transactions"))?;

        let algorithm_proof =
            algorithm_proof.ok_or_else(|| de::Error::missing_field("algorithm_proof"))?;

        let previous_block_hash =
            previous_block_hash.ok_or_else(|| de::Error::missing_field("previous_block_hash"))?;

        let timestamp = timestamp.ok_or_else(|| de::Error::missing_field("timestamp"))?;

        Ok(Block::from(
            hash,
            index,
            transactions,
            algorithm_proof,
            previous_block_hash,
            timestamp,
        ))
    }
}

impl<'de> Deserialize<'de> for Block {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct("Block", &self::FIELDS, BlockVisitor)
    }
}

impl Serialize for Block {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut block = serializer.serialize_struct("Block", self::FIELDS_COUNT)?;

        let hash = match self.hash() {
            Some(hash) => hash,
            None => "",
        };

        let previous_block_hash = match self.previous_block_hash() {
            Some(previous_block_hash) => previous_block_hash,
            None => "",
        };

        block.serialize_field("index", &self.index()).ok();

        block
            .serialize_field("algorithm_proof", &self.algorithm_proof())
            .ok();

        block
            .serialize_field("transactions", &self.transactions())
            .ok();

        block.serialize_field("hash", &hash).ok();

        block
            .serialize_field("previous_block_hash", &previous_block_hash)
            .ok();

        block.serialize_field("timestamp", &self.timestamp()).ok();

        block.end()
    }
}
