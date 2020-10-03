use std::fmt;

use serde::de::{self, MapAccess, Visitor};
use serde::export::Formatter;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::crypto::block::Sha256BlockHasher;
use crate::crypto::proof_of_work::Sha256ProofValidator;
use crate::crypto::Sha256Blockchain;
use crate::model::block::BlockHasher;
use crate::model::blockchain::{Blockchain, PROOF_OF_WORK_DIFFICULTY};
use crate::model::proof_of_work::{ProofOfWork, ProofValidator};

const FIELDS_COUNT: usize = 2;
const FIELDS: [&str; FIELDS_COUNT] = ["blocks", "transactions_to_process"];

enum BlockchainField {
    Blocks,
    TransactionsToProcess,
    RegisteredNodes,
}

struct BlockchainFieldVisitor;

struct BlockchainVisitor;

impl<'de> Visitor<'de> for BlockchainFieldVisitor {
    type Value = BlockchainField;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("'blocks', 'transactions_to_process', 'registered_nodes'")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match value {
            "blocks" => Ok(BlockchainField::Blocks),
            "transactions_to_process" => Ok(BlockchainField::TransactionsToProcess),
            "registered_nodes" => Ok(BlockchainField::RegisteredNodes),

            _ => Err(de::Error::unknown_field(value, &self::FIELDS)),
        }
    }
}

impl<'de> Deserialize<'de> for BlockchainField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_identifier(BlockchainFieldVisitor)
    }
}

impl<'de> Visitor<'de> for BlockchainVisitor {
    type Value = Sha256Blockchain;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("struct Sha256Blockchain")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut blocks = None;
        let mut transactions_to_process = None;
        let mut registered_nodes = None;

        while let Some(key) = map.next_key()? {
            match key {
                BlockchainField::Blocks => {
                    if blocks.is_some() {
                        return Err(de::Error::duplicate_field("blocks"));
                    }

                    blocks = Some(map.next_value()?);
                }
                BlockchainField::TransactionsToProcess => {
                    if transactions_to_process.is_some() {
                        return Err(de::Error::duplicate_field("transactions_to_process"));
                    }

                    transactions_to_process = Some(map.next_value()?);
                }
                BlockchainField::RegisteredNodes => {
                    if registered_nodes.is_some() {
                        return Err(de::Error::duplicate_field("registered_nodes"));
                    }

                    registered_nodes = Some(map.next_value()?);
                }
            }
        }

        let blocks = blocks.ok_or_else(|| de::Error::missing_field("blocks"))?;
        let transactions_to_process = transactions_to_process
            .ok_or_else(|| de::Error::missing_field("transactions_to_process"))?;
        let registered_nodes =
            registered_nodes.ok_or_else(|| de::Error::missing_field("registered_nodes"))?;

        Ok(Blockchain::from(
            blocks,
            transactions_to_process,
            registered_nodes,
            ProofOfWork::new(PROOF_OF_WORK_DIFFICULTY, Sha256ProofValidator),
            Sha256BlockHasher,
        ))
    }
}

impl<'de> Deserialize<'de> for Sha256Blockchain {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct(
            "Blockchain<Sha256ProofValidator, Sha256BlockHasher>",
            &FIELDS,
            BlockchainVisitor,
        )
    }
}

impl<T, U> Serialize for Blockchain<T, U>
where
    T: ProofValidator,
    U: BlockHasher + Clone,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut blockchain = serializer.serialize_struct("Blockchain", self::FIELDS_COUNT)?;

        blockchain.serialize_field("blocks", &self.blocks()).ok();

        blockchain
            .serialize_field("transactions_to_process", &self.transactions_to_process())
            .ok();

        blockchain
            .serialize_field("registered_nodes", &self.neighbour_nodes())
            .ok();

        blockchain.end()
    }
}
