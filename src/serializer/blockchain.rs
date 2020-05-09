use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

use crate::model::block::BlockHasher;
use crate::model::blockchain::Blockchain;
use crate::model::proof_of_work::ProofValidator;

const FIELDS_COUNT: usize = 2;

impl<T, U> Serialize for Blockchain<T, U>
where
    T: ProofValidator,
    U: BlockHasher + Clone,
{
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut blockchain = serializer.serialize_struct("Blockchain", self::FIELDS_COUNT)?;

        blockchain
            .serialize_field("count", &self.blocks().len())
            .ok();
        blockchain.serialize_field("blocks", &self.blocks()).ok();

        blockchain.end()
    }
}
