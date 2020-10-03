use crate::model::block::BlockHasher;
use crate::model::blockchain::Blockchain;
use crate::model::proof_of_work::ProofValidator;

pub fn is_blockchain_authoritative<T, U>(blockchain: &Blockchain<T, U>, current_size: usize) -> bool
where
    T: ProofValidator,
    U: BlockHasher + Clone,
{
    blockchain.blocks().len() > current_size && blockchain.is_valid()
}
