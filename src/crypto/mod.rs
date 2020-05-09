use crate::crypto::block::Sha256BlockHasher;
use crate::crypto::proof_of_work::Sha256ProofValidator;
use crate::model::blockchain::Blockchain;

pub mod block;
pub mod node;
pub mod proof_of_work;

pub type Sha256Blockchain = Blockchain<Sha256ProofValidator, Sha256BlockHasher>;
